use self::account_reader::InputAccountReader;
use self::cashflow_data_appender::append_cf_data;
use self::repricing_file::RepData;
use self::structs::RepaySchedData;
use calamine::open_workbook_auto;
use calamine::Reader;
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
use std::path::Path;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_data_appender;
mod repricing_file;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let reader = InputAccountReader::new(config_params.input_file_path(), logger);
    let mut writer = AccountWithCashflowsWriter::new(config_params.output_file_path(), logger);
    let mut reader_iterator = reader;

    //Calculate next_rep_date
    let mut rep_master_map: HashMap<String, NaiveDate> = HashMap::new();
    let rep_master_file_path = config_params.rep_master_file_path();
    let mut master_file_path = open_workbook_auto(config_params.rep_master_file_path())
        .expect("Unable to open the repricing master xlsx file.");
    if let Some(Ok(master_file_reader)) =
        master_file_path.worksheet_range(config_params.rep_sheet_name())
    {
        for (row_no, row) in master_file_reader.rows().enumerate().skip(1) {
            let rep_data = RepData::new_from_xlsx(row, config_params);
            let acct_id = rep_data.account_id;
            if acct_id.len() >= 6 {
                let first_six_digits: &str = &acct_id[..6];
                let rep_date = rep_data.next_repriced_date;
                rep_master_map.insert(first_six_digits.to_string(), rep_date);
            }else{
                log_info!(logger, "The length for the account id : {:?} is less than 6 in master file",acct_id);
            }
        }
    }
    //taking max_date as 31-12-9999
    let max_date = rbdate::date_from_timestamp(253402214400);
    let mut call_date_map: HashMap<String, NaiveDate> = HashMap::new();
    let is_call_date_present = Path::new(config_params.call_date_file()).exists();
    if is_call_date_present {
        let call_date_file = match new_buf_rdr(config_params.call_date_file()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                config_params.call_date_file(),
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error
            ),
        };
        for (line_num, lines) in call_date_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.call_date_file(),
                    line_num + 1,
                    error
                ),
            };
            let call_date_fields: Vec<&str> = line.split("|").collect();
            if call_date_fields.len() != 2 {
                log_warn!(
                    logger,
                    "Skipping line: `{}` from : `{}`",
                    line_num + 1,
                    config_params.call_date_file()
                );
            }
            let call_date = rbdate::NaiveDate::parse_from_str(&call_date_fields[1], "%d-%m-%Y")
                .unwrap_or(max_date);
            call_date_map.insert(call_date_fields[0].to_string(), call_date);
        }
    }

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
        let mut int_amt = fields[3].parse().unwrap_or(0.0);
        if !config_params.write_int_cashflows() {
            int_amt = 0.0;
        }
        let data = RepaySchedData {
            cf_date: min(
                rbdate::NaiveDate::parse_from_str(&converted_cfdate, "%d-%m-%Y")
                    .unwrap_or(*config_params.as_on_date()),
                *call_date_map
                    .get(&fields[0].to_string())
                    .unwrap_or(&max_date),
            ),
            principal_amount: fields[2].parse().unwrap_or(0.0),
            interest_amount: int_amt,
        };
        repaysched_map
            .entry(fields[0].to_string())
            .and_modify(|prev_data| prev_data.push(data.clone()))
            .or_insert(vec![data]);
    }
    for values in repaysched_map.values_mut() {
        values.sort_by(|a, b| a.cf_date.cmp(&b.cf_date));
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
        total_balance_input += account_data.loan_bal;

        let mut maturity_date = account_data
            .mat_dt
            .expect("Cannot read account maturity date");
        let asondate = account_data.asondate.expect("Cannot read account AsOndate");

        if maturity_date < asondate {
            let mut cashflows: Vec<Cashflow> = Vec::new();
            let mut out_acc = Account::new();
            append_cf_data(
                &mut out_acc,
                &account_data,
                *config_params.as_on_date(),
                &rep_master_map,
                &config_params,
            );

            let mut new_cf = Cashflow::new();
            new_cf.date = (asondate + chrono::Duration::days(config_params.od_additional_day()))
                .and_hms(0, 0, 0)
                .timestamp();
            new_cf.principal_amount = account_data.loan_bal;
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
                append_cf_data(
                    &mut out_acc,
                    &account_data,
                    *config_params.as_on_date(),
                    &rep_master_map,
                    &config_params,
                );
                let mut new_cf = Cashflow::new();
                new_cf.date = maturity_date.and_hms(0, 0, 0).timestamp();
                new_cf.principal_amount = account_data.loan_bal;
                new_cf.interest_amount = 0.0;
                total_balance_output += new_cf.principal_amount;
                cashflows.push(new_cf);
                total_cfs += cashflows.len();
                out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
                writer.write(out_acc);
                continue;
            }
        };

        let mut no_of_due: f64 = ((account_data.theo_loan_bal - account_data.loan_bal)
            / (account_data.loan_repay))
            .round();
        log_debug!(
            diag_logger,
            "Account: {0}; Due: {1}; ",
            account_data.key_1,
            no_of_due,
        );
        let mut cashflows: Vec<Cashflow> = Vec::new();
        let mut out_acc = Account::new();
        let mut od_bal = 0.0;
        let mut os_bal = account_data.loan_bal;
        let mut start_date = asondate;
        let mut adj_bal = 0.0 as f64;
        let mut prev_cf_date = 0.0 as i64;
        if no_of_due < 0.0 {
            od_bal = account_data.loan_bal - account_data.theo_loan_bal.abs();
            if od_bal > 0.0 {
                let mut new_cf = Cashflow::new();
                start_date = asondate + chrono::Duration::days(config_params.od_additional_day());
                new_cf.date = start_date.and_hms(0, 0, 0).timestamp();
                new_cf.principal_amount = od_bal;
                new_cf.interest_amount = 0.0;
                os_bal = os_bal - od_bal;
                prev_cf_date = new_cf.date;
                if start_date > maturity_date {
                    maturity_date = start_date;
                }
                total_balance_output += new_cf.principal_amount;
                cashflows.push(new_cf);
                od_bal = 0.0;
            }
        }
        append_cf_data(
            &mut out_acc,
            &account_data,
            *config_params.as_on_date(),
            &rep_master_map.clone(),
            &config_params,
        );
        for (pos, repaydata) in acc_repay_data.iter().enumerate() {
            if repaydata.cf_date >= start_date && repaydata.cf_date < maturity_date {
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
                    let mut is_mat = "N";
                    if config_params.adjust_to_prev_cf() {
                        if pos < acc_repay_data.len() && prev_cf_date != 0 {
                            new_cf.date = prev_cf_date;
                        } else {
                            new_cf.date = maturity_date.and_hms(0, 0, 0).timestamp();
                            is_mat = "Y";
                        }
                    } else {
                        if pos < acc_repay_data.len() && prev_cf_date != 0 && os_bal < 0.0 {
                            new_cf.date = prev_cf_date;
                        } else if pos < acc_repay_data.len() && os_bal > 0.0 {
                            new_cf.date = repaydata.cf_date.and_hms(0, 0, 0).timestamp();
                        } else {
                            new_cf.date = maturity_date.and_hms(0, 0, 0).timestamp();
                            is_mat = "Y";
                        }
                    }
                    log_debug!(
                        diag_logger,
                        "AccNo: {} | OS_bal: {} | CFdate: {} | isMaturity: {} ",
                        account_data.key_1,
                        os_bal,
                        NaiveDateTime::from_timestamp(new_cf.date, 0),
                        is_mat
                    );
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
