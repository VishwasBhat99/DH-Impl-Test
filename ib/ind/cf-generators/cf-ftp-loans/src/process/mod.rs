use self::account_reader::input_account::AddBORMData;
use self::account_reader::InputAccountReader;
use self::cashflow_data_appender::append_cf_data;
use self::structs::RepaySchedData;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::account_with_cashflows::Account;
use process::account_with_cashflows::Cashflow;
use process::account_with_cashflows_writer::AccountWithCashflowsWriter;
use rbdate::*;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::cmp::min;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env::current_dir;
use std::io::prelude::*;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_data_appender;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let reader = InputAccountReader::new(config_params.input_file_path(), logger);
    let mut writer = AccountWithCashflowsWriter::new(config_params.output_file_path(), logger);
    let mut reader_iterator = reader;
    let repayment_schedule_file = match new_buf_rdr(config_params.repayment_schedule_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.repayment_schedule_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut repaysched_map: HashMap<String, Vec<RepaySchedData>> = HashMap::new();
    for (line_num, lines) in repayment_schedule_file.lines().enumerate().skip(15) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.repayment_schedule_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split("|").collect();
        let converted_cfdate = rbdate::datevalue_to_naive_date(
            &(fields[1].parse::<i64>().unwrap() + 1 as i64).to_string(),
        )
        .unwrap()
        .format("%d-%m-%Y")
        .to_string();
        let data = RepaySchedData {
            cf_date: rbdate::NaiveDate::parse_from_str(&converted_cfdate, "%d-%m-%Y")
                .unwrap_or(*config_params.as_on_date()),
            principal_amount: fields[2].parse().unwrap_or(0.0),
            interest_amount: fields[3].parse().unwrap_or(0.0),
        };
        repaysched_map
            .entry(fields[0].to_string())
            .and_modify(|prev_data| prev_data.push(data.clone()))
            .or_insert(vec![data]);
    }
    for values in repaysched_map.values_mut() {
        values.sort_by(|a, b| a.cf_date.cmp(&b.cf_date));
    }
    //Reading Additional BORM File
    let add_borm_data = match new_buf_rdr(config_params.additional_borm_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find file `{}` : {}.",
            config_params.additional_borm_file_path(),
            error
        ),
    };
    let mut borm_map: HashMap<String, AddBORMData> = HashMap::new();
    for (line_no, line) in add_borm_data.lines().enumerate().skip(1) {
        let borm_info: String = match line {
            Ok(borm_info) => borm_info,
            Err(error) => {
                log_error!(
                    logger,
                    "Cannot read line {} from Additional BORM file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let borm_fields: Vec<&str> = borm_info.split('|').collect();
        let borm_data = AddBORMData::new(borm_fields, line_no);
        borm_map.insert(borm_data.key_1.to_string(), borm_data);
    }

    let mut account_encountered = 0;
    let mut account_skipped = 0;
    let mut total_balance_input = 0.0;
    let mut total_balance_output = 0.0;
    let mut total_cfs = 0;
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        account_encountered += 1;
        let account_data =
            account_opt.expect("Unexpected error occured while unwraping account data");
        let theo_loan_bal = if borm_map.contains_key(&account_data.key_1.to_string()) {
            let borm_data = borm_map
                .get(&account_data.key_1.to_string())
                .unwrap_or_else(|| {
                    panic!("Could not get BORM data for Acct: {}", account_data.key_1)
                });
            if borm_data.comp_freq == "0" || borm_data.comp_freq == "00" {
                borm_data.theo_unpd_prin_bal
            } else {
                account_data.theo_loan_bal
            }
        } else {
            account_data.theo_loan_bal
        };
        total_balance_input += account_data.a4.parse::<f64>().expect(
            &format!("Error while parsing ftp Osbal for {}", account_data.key_1).to_string(),
        );

        let maturity_date = account_data
            .mat_dt
            .expect("Cannot read account maturity date");
        let asondate = account_data.asondate.expect("Cannot read account AsOndate");

        if maturity_date < asondate {
            let mut cashflows: Vec<Cashflow> = Vec::new();
            let mut out_acc = Account::new();
            append_cf_data(&mut out_acc, &account_data, *config_params.as_on_date());
            let mut new_cf = Cashflow::new();
            new_cf.date = (asondate + chrono::Duration::days(config_params.od_additional_day()))
                .and_hms(0, 0, 0)
                .timestamp();
            new_cf.principal_amount = account_data.a4.parse::<f64>().expect(
                &format!("Error while parsing ftp Osbal for {}", account_data.key_1).to_string(),
            );
            new_cf.interest_amount = 0.0;

            total_balance_output += new_cf.principal_amount;
            cashflows.push(new_cf);
            total_cfs += cashflows.len();
            out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
            writer.write(out_acc);
            continue;
        }

        let acc_repay_data: &Vec<RepaySchedData> = match repaysched_map.get_mut(&account_data.key_1)
        {
            Some(val) => val,
            None => {
                let mut cashflows: Vec<Cashflow> = Vec::new();
                let mut out_acc = Account::new();
                append_cf_data(&mut out_acc, &account_data, *config_params.as_on_date());
                let mut new_cf = Cashflow::new();
                new_cf.date = maturity_date.and_hms(0, 0, 0).timestamp();
                new_cf.principal_amount = account_data.a4.parse::<f64>().expect(
                    &format!("Error while parsing ftp Osbal for {}", account_data.key_1)
                        .to_string(),
                );
                new_cf.interest_amount = 0.0;
                total_balance_output += new_cf.principal_amount;
                cashflows.push(new_cf);
                total_cfs += cashflows.len();
                out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
                writer.write(out_acc);
                continue;
            }
        };

        let near_cf_index = acc_repay_data.into_iter().enumerate().fold(
            (0, acc_repay_data[0].cf_date),
            |min, (ind, val)| {
                if i64::from(val.cf_date.num_days_from_ce() - asondate.num_days_from_ce()).abs()
                    < i64::from(min.1.num_days_from_ce() - asondate.num_days_from_ce()).abs()
                {
                    (ind, val.cf_date)
                } else {
                    min
                }
            },
        );
        let mut no_of_due: f64 =
            ((theo_loan_bal - account_data.loan_bal) / (account_data.loan_repay)).round();
        let mut cf_begin_index: usize = ((near_cf_index.0) as f64 + no_of_due) as usize;
        log_debug!(
            diag_logger,
            "Account: {0}; Near CF Index: {1}; Near CF Date: {2}; Due: {3}; CF Begin Index: {4}",
            account_data.key_1,
            near_cf_index.0,
            near_cf_index.1,
            no_of_due,
            cf_begin_index
        );
        let mut cashflows: Vec<Cashflow> = Vec::new();
        let mut out_acc = Account::new();
        let mut od_bal = 0.0;
        let mut os_bal = account_data.a4.parse::<f64>().expect(
            &format!("Error while parsing ftp Osbal for {}", account_data.key_1).to_string(),
        );
        if no_of_due < 0.0 {
            od_bal = account_data.a4.parse::<f64>().expect(
                &format!("Error while parsing ftp Osbal for {}", account_data.key_1).to_string(),
            ) - theo_loan_bal.abs();
        }
        let mut adj_bal = 0.0 as f64;
        let mut prev_cf_date = 0.0 as i64;
        append_cf_data(&mut out_acc, &account_data, *config_params.as_on_date());
        if account_data.loan_repay == 0.0 {
            cf_begin_index = 0;
        }
        for (pos, repaydata) in acc_repay_data.iter().enumerate().skip(cf_begin_index) {
            if repaydata.cf_date >= asondate && repaydata.cf_date < maturity_date {
                if od_bal > 0.0 {
                    let mut new_cf = Cashflow::new();
                    new_cf.date = repaydata.cf_date.and_hms(0, 0, 0).timestamp();
                    new_cf.principal_amount = od_bal;
                    new_cf.interest_amount = 0.0;
                    prev_cf_date = new_cf.date;
                    os_bal = os_bal - od_bal;
                    total_balance_output += new_cf.principal_amount;
                    cashflows.push(new_cf);
                    od_bal = 0.0;
                }
                if os_bal == 0.0 {
                    break;
                } else if os_bal > 0.0 && os_bal >= repaydata.principal_amount {
                    let mut new_cf = Cashflow::new();
                    new_cf.date = repaydata.cf_date.and_hms(0, 0, 0).timestamp();
                    new_cf.principal_amount = repaydata.principal_amount;
                    new_cf.interest_amount = repaydata.interest_amount;
                    prev_cf_date = new_cf.date;
                    os_bal = os_bal - repaydata.principal_amount;
                    total_balance_output += new_cf.principal_amount;
                    cashflows.push(new_cf);
                } else if os_bal < 0.0 || os_bal < repaydata.principal_amount {
                    let mut new_cf = Cashflow::new();
                    if pos < acc_repay_data.len() && prev_cf_date != 0 {
                        new_cf.date = prev_cf_date;
                    } else {
                        new_cf.date = maturity_date.and_hms(0, 0, 0).timestamp();
                    }
                    new_cf.principal_amount = os_bal;
                    new_cf.interest_amount = 0.0;
                    os_bal = 0.0;
                    prev_cf_date = new_cf.date;
                    total_balance_output += new_cf.principal_amount;
                    cashflows.push(new_cf);
                    break;
                }
            }
        }
        if os_bal != 0.0 {
            let mut new_cf = Cashflow::new();
            new_cf.date = maturity_date.and_hms(0, 0, 0).timestamp();
            new_cf.principal_amount = os_bal;
            new_cf.interest_amount = 0.0;
            total_balance_output += new_cf.principal_amount;
            cashflows.push(new_cf);
            os_bal = 0.0;
        }
        total_cfs += cashflows.len();
        out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
        writer.write(out_acc);
    }
    let health_report = HealthReport::new(
        account_encountered,
        account_encountered - account_skipped,
        account_skipped,
        total_balance_input,
        total_balance_output,
        total_cfs as i64,
    );
    health_report.gen_health_rpt(config_params.output_file_path())
}
