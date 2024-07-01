use self::account_reader::InputAccountReader;
use self::cashflow_data_appender::append_cf_data;
use self::structs::ResData;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use math::round::half_away_from_zero;
use process::account_with_cashflows::Account;
use process::account_with_cashflows::Cashflow;
use process::account_with_cashflows_writer::AccountWithCashflowsWriter;
use rbdate::*;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env::current_dir;
use std::io::prelude::*;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_data_appender;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let reader = InputAccountReader::new(config_params.input_file_path(), logger);
    let mut writer = AccountWithCashflowsWriter::new(config_params.output_file_path(), logger);
    let op_path_securitised = format!("{}_securitized", config_params.output_file_path());
    let mut writer_non_hdfc = AccountWithCashflowsWriter::new(&op_path_securitised, logger);

    let mut op_map_hdfc: HashMap<String, Account> = HashMap::new();
    let mut op_map_non_hdfc: HashMap<String, Account> = HashMap::new();

    let mut reader_iterator = reader;
    let restructure_file = match new_buf_rdr(config_params.restructure_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.restructure_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut res_map: HashMap<String, Vec<ResData>> = HashMap::new();
    for (line_num, lines) in restructure_file.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.restructure_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let data = ResData {
            struct_number: fields[1].to_string(),
            interest_type: fields[2].to_string(),
            expected_interest_rate: fields[3].parse().unwrap_or(0.0),
            ei_period: fields[4].to_string(),
            ei_amount: fields[5].parse().unwrap_or(0.0),
            ei_start_date: rbdate::NaiveDate::parse_from_str(fields[6], "%d/%m/%Y")
                .unwrap_or(*config_params.as_on_date()),
            ei_end_date: rbdate::NaiveDate::parse_from_str(fields[7], "%d/%m/%Y")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2099, 12, 31).expect("Cannot getting date")),
        };
        res_map
            .entry(fields[0].to_string())
            .and_modify(|prev_data| prev_data.push(data.clone()))
            .or_insert_with(|| vec![data]);
    }

    // Sort Restructure account data by EI start date
    for values in res_map.values_mut() {
        values.sort_by(|a, b| a.ei_start_date.cmp(&b.ei_start_date));
    }

    let mut account_encountered = 0;
    let mut account_skipped = 0;
    let mut total_balance_input = 0.0;
    let mut total_balance_output = 0.0;
    let mut total_balance_sec_output = 0.0;
    let mut total_cfs = 0;

    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        account_encountered += 1;
        let account_data =
            account_opt.expect("Unexpected error occured while unwraping account data");
        total_balance_input += account_data.os_loan_bal_local_currency;
        let acc_no = &account_data.acc_no;
        let acc_res_data: &Vec<ResData> = match res_map.get_mut(acc_no) {
            Some(val) => val,
            None => {
                log_error!(logger, "Restructured data not found for: {}", acc_no);
                account_skipped += 1;
                continue;
            }
        };
        let mut current_ost_bal = account_data.os_loan_bal_local_currency;
        let mut current_ost_bal_sec = account_data.os_loan_bal_local_currency;
        let inst_period = match account_data.ei_payment_frequency_current.as_str() {
            "MONTHLY" => 1,
            "QUARTERLY" => 3,
            "HALFYEARLY" => 6,
            "YEARLY" => 12,
            _ => panic!("Invalid Payment Freq Encountered!!"),
        };
        let mut inst_start_date = account_data
            .ei_start_date_current
            .expect("Cannot read ei start date")
            .pred_opt()
            .expect("Cannot getting previous date");
        let maturity_date = account_data
            .maturity_date
            .expect("Cannot read account maturity date");
        let ei_pay_day: i64 = account_data
            .ei_payment_day
            .parse()
            .expect("Cannot read ei_payment_day");
        for data in acc_res_data {
            if inst_start_date <= data.ei_start_date {
                inst_start_date = data.ei_start_date.pred_opt().expect("Cannot getting previous date");
            }
            let inst_end_date = if data.ei_end_date > maturity_date {
                maturity_date
            } else {
                data.ei_end_date
            };
            let cashflows = generate_cashflows(
                inst_end_date,
                inst_period,
                &mut current_ost_bal,
                data.expected_interest_rate,
                data.ei_amount,
                &mut total_balance_output,
                account_data.hdfc_ltd_percentage,
                *config_params.as_on_date(),
                ei_pay_day,
                config_params,
            );
            let mut outstanding_amount = ((account_data.os_loan_bal_local_currency
                + account_data.pre_ei_bal_local_curr)
                * (account_data.hdfc_ltd_percentage))
                / 100.0;
            outstanding_amount = half_away_from_zero(outstanding_amount, 0);
            let mut tot_prin_amt = 0.0;
            for cf in &cashflows {
                tot_prin_amt += cf.principal_amount;
            }
            if tot_prin_amt != outstanding_amount && tot_prin_amt != 0.0 {
                log_debug!(
                    logger,
                    "acc_no = {}, total_cf_prin_amt = {}, outstanding_amt = {}, Difference = {}",
                    acc_no,
                    tot_prin_amt,
                    outstanding_amount,
                    tot_prin_amt - outstanding_amount
                );
            }

            total_cfs += cashflows.len();
            // non-hdfc or securitized portion cashflows
            let cashflows_non_hdfc = generate_cashflows_securitized(
                inst_end_date,
                inst_period,
                &mut current_ost_bal_sec,
                data.expected_interest_rate,
                data.ei_amount,
                &mut total_balance_sec_output,
                account_data.securitization_percentage,
                *config_params.as_on_date(),
                ei_pay_day,
                config_params,
            );

            let mut out_acc = Account::new();
            append_cf_data(&mut out_acc, &account_data, *config_params.as_on_date());

            out_acc.structure_number = data.struct_number.to_string();
            out_acc.maturity_date = data.ei_end_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
            out_acc.memi = data.ei_amount;
            out_acc.currency = account_data.currency.to_string();
            out_acc.ost_bal = current_ost_bal;
            out_acc.roi = data.expected_interest_rate;
            out_acc.ost_bal = current_ost_bal;
            let mut tot_int_amt = 0.0;
            let mut tot_prin_amt = 0.0;
            for cf in &cashflows {
                tot_int_amt += cf.interest_amount;
                tot_prin_amt += cf.principal_amount;
            }
            out_acc.tot_prin_amt = tot_prin_amt;
            out_acc.tot_int_amt = tot_int_amt;
            out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
            append_cashflows(&mut op_map_hdfc, out_acc, logger, "HDFC");

            // securitized portion cashflow accounts
            let mut out_acc_non_hdfc = Account::new();
            append_cf_data(
                &mut out_acc_non_hdfc,
                &account_data,
                *config_params.as_on_date(),
            );

            out_acc_non_hdfc.structure_number = data.struct_number.to_string();
            out_acc_non_hdfc.maturity_date = data.ei_end_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
            out_acc_non_hdfc.memi = data.ei_amount;
            out_acc_non_hdfc.currency = account_data.currency.to_string();
            out_acc_non_hdfc.ost_bal = current_ost_bal_sec;
            out_acc_non_hdfc.roi = data.expected_interest_rate;
            out_acc_non_hdfc.ost_bal = current_ost_bal_sec;
            let mut tot_int_amt = 0.0;
            let mut tot_prin_amt = 0.0;
            for cf in &cashflows_non_hdfc {
                tot_int_amt += cf.interest_amount;
                tot_prin_amt += cf.principal_amount;
            }
            out_acc_non_hdfc.tot_prin_amt = tot_prin_amt;
            out_acc_non_hdfc.tot_int_amt = tot_int_amt;
            out_acc_non_hdfc.cashflows = protobuf::RepeatedField::from_vec(cashflows_non_hdfc);
            append_cashflows(&mut op_map_non_hdfc, out_acc_non_hdfc, logger, "Non-HDFC");
        }
        // Bullet CF for remaining outstanding amount
        if current_ost_bal != 0.0 {
            let mut out_acc = Account::new();
            append_cf_data(&mut out_acc, &account_data, *config_params.as_on_date());

            out_acc.structure_number = "99".to_string();
            out_acc.maturity_date = maturity_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
            out_acc.memi = account_data.ei_amount_current;
            out_acc.currency = account_data.currency.to_string();
            out_acc.roi = account_data.curr_applicable_interest_rate;
            out_acc.ost_bal = current_ost_bal;
            let mut cashflows: Vec<Cashflow> = Vec::new();
            let interest_amt = (current_ost_bal
                * (account_data.curr_applicable_interest_rate / (12.0 * 100.0)))
                * (account_data.hdfc_ltd_percentage / 100.0);
            let mut new_cf = Cashflow::new();
            new_cf.interest_amount = half_away_from_zero(interest_amt, config_params.precision());
            let hdfc_principal_amt = current_ost_bal * (account_data.hdfc_ltd_percentage / 100.0);
            new_cf.principal_amount =
                half_away_from_zero(hdfc_principal_amt, config_params.precision());
            new_cf.date = maturity_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
            cashflows.push(new_cf);
            out_acc.tot_int_amt = half_away_from_zero(interest_amt, config_params.precision());
            out_acc.tot_prin_amt = half_away_from_zero(current_ost_bal, config_params.precision());
            out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

            total_balance_output += current_ost_bal;
            append_cashflows(&mut op_map_hdfc, out_acc, logger, "HDFC");
        }
        if current_ost_bal_sec != 0.0 {
            // securitized portion cashflow accounts
            let mut out_acc_non_hdfc = Account::new();
            append_cf_data(
                &mut out_acc_non_hdfc,
                &account_data,
                *config_params.as_on_date(),
            );

            out_acc_non_hdfc.structure_number = "99".to_string();
            out_acc_non_hdfc.maturity_date = maturity_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
            out_acc_non_hdfc.memi = account_data.ei_amount_current;
            out_acc_non_hdfc.currency = account_data.currency;
            out_acc_non_hdfc.roi = account_data.curr_applicable_interest_rate;
            out_acc_non_hdfc.ost_bal = current_ost_bal_sec;
            let mut cashflows_sec: Vec<Cashflow> = Vec::new();
            let interest_amt = (current_ost_bal_sec
                * (account_data.curr_applicable_interest_rate / (12.0 * 100.0)))
                * (account_data.securitization_percentage / 100.0);
            let mut new_cf = Cashflow::new();
            new_cf.interest_amount = half_away_from_zero(interest_amt, config_params.precision());
            let sec_pricipal_amt =
                current_ost_bal_sec * (account_data.securitization_percentage / 100.0);
            new_cf.principal_amount =
                half_away_from_zero(sec_pricipal_amt, config_params.precision());
            new_cf.date = maturity_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
            cashflows_sec.push(new_cf);
            out_acc_non_hdfc.tot_int_amt =
                half_away_from_zero(interest_amt, config_params.precision());
            if account_data.securitization_percentage != 0.0 {
                out_acc_non_hdfc.tot_prin_amt =
                    half_away_from_zero(current_ost_bal, config_params.precision());
            } else {
                out_acc_non_hdfc.tot_prin_amt = 0.0;
            }
            out_acc_non_hdfc.cashflows = protobuf::RepeatedField::from_vec(cashflows_sec);

            total_balance_sec_output += current_ost_bal_sec;
            append_cashflows(&mut op_map_non_hdfc, out_acc_non_hdfc, logger, "Non-HDFC");
        }
        writer.write(
            op_map_hdfc
                .get(acc_no)
                .expect("Could Not get Account Data")
                .clone(),
        );
        writer_non_hdfc.write(
            op_map_non_hdfc
                .get(acc_no)
                .expect("Could Not get Non-HDFC Account Data")
                .clone(),
        );
        op_map_hdfc.clear();
        op_map_non_hdfc.clear();
    }

    let health_stat = HealthReport::new(
        account_encountered,
        account_encountered - account_skipped,
        account_skipped,
        total_balance_input,
        total_balance_input,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn generate_cashflows(
    end_date: NaiveDate,
    inst_period: usize,
    ost_bal: &mut f64,
    roi: f64,
    memi: f64,
    total_balance_output: &mut f64,
    hdfc_ltd_percentage: f64,
    as_on_date: NaiveDate,
    ei_pay_day: i64,
    config_params: &ConfigurationParameters,
) -> Vec<Cashflow> {
    let mut cashflows: Vec<Cashflow> = Vec::new();
    let prev_cf_date = if i64::from(as_on_date.day()) < ei_pay_day {
        match NaiveDate::from_ymd_opt(
            as_on_date.year(),
            as_on_date.month(),
            ei_pay_day.try_into().expect("Cannot read ei_pay_day"),
        ) {
            Some(prev_cf_date) => prev_cf_date,
            None => get_month_end_date(as_on_date),
        }
    } else {
        increment_date_by_months(
            match NaiveDate::from_ymd_opt(
                as_on_date.year(),
                as_on_date.month(),
                ei_pay_day.try_into().expect("Cannot read ei_pay_day"),
            ) {
                Some(prev_cf_date) => prev_cf_date,
                None => get_month_end_date(as_on_date),
            },
            1,
        )
    };
    let mut new_cf_date = prev_cf_date;
    let mut month_to_incr = inst_period;
    while new_cf_date < end_date {
        let initial_interest_amt = half_away_from_zero(
            *ost_bal * (roi / (12.0 * 100.0)),
            config_params.precision(),
        );
        let initial_principal_amt =
            if *ost_bal < half_away_from_zero(memi, config_params.precision()) {
                *ost_bal
            } else {
                half_away_from_zero(memi - initial_interest_amt, config_params.precision())
            };
        let final_principal_amt =
            half_away_from_zero(initial_principal_amt, config_params.precision())
                * (hdfc_ltd_percentage / 100.0);
        let final_interest_amount =
            half_away_from_zero(initial_interest_amt, config_params.precision())
                * (hdfc_ltd_percentage / 100.0);
        *total_balance_output +=
            half_away_from_zero(initial_principal_amt, config_params.precision());

        let mut new_cf = Cashflow::new();
        new_cf.interest_amount =
            half_away_from_zero(final_interest_amount, config_params.precision());
        new_cf.principal_amount =
            half_away_from_zero(final_principal_amt, config_params.precision());
        new_cf.date = new_cf_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
        cashflows.push(new_cf);

        *ost_bal -= half_away_from_zero(initial_principal_amt, config_params.precision());

        let mut new_date =
            rbdate::increment_date_by_months(prev_cf_date, month_to_incr.try_into().expect("Error while incrementing date"));
        if ei_pay_day == 31 && prev_cf_date.day() == 30 {
            new_date = get_month_end_date(new_date);
        }
        new_cf_date = if new_date >= end_date {
            end_date
        } else {
            new_date
        };
        month_to_incr += inst_period;
    }
    cashflows
}

// derive cashflows for non-hdfc portion
fn generate_cashflows_securitized(
    end_date: NaiveDate,
    inst_period: usize,
    ost_bal_sec: &mut f64,
    roi: f64,
    memi: f64,
    total_balance_sec_output: &mut f64,
    securitization_percentage: f64,
    as_on_date: NaiveDate,
    ei_pay_day: i64,
    config_params: &ConfigurationParameters,
) -> Vec<Cashflow> {
    let mut cashflows: Vec<Cashflow> = Vec::new();
    let prev_cf_date = if i64::from(as_on_date.day()) < ei_pay_day {
        match NaiveDate::from_ymd_opt(
            as_on_date.year(),
            as_on_date.month(),
            ei_pay_day.try_into().expect("Cannot read ei_pay_day"),
        ) {
            Some(prev_cf_date) => prev_cf_date,
            None => get_month_end_date(as_on_date),
        }
    } else {
        increment_date_by_months(
            match NaiveDate::from_ymd_opt(
                as_on_date.year(),
                as_on_date.month(),
                ei_pay_day.try_into().expect("Cannot read ei_pay_day"),
            ) {
                Some(prev_cf_date) => prev_cf_date,
                None => get_month_end_date(as_on_date),
            },
            1,
        )
    };
    let mut new_cf_date = prev_cf_date;
    let mut month_to_incr = inst_period;
    while new_cf_date < end_date {
        let initial_interest_amt = half_away_from_zero(
            *ost_bal_sec * (roi / (12.0 * 100.0)),
            config_params.precision(),
        );
        let initial_principal_amt =
            if *ost_bal_sec < half_away_from_zero(memi, config_params.precision()) {
                *ost_bal_sec
            } else {
                half_away_from_zero(memi - initial_interest_amt, config_params.precision())
            };
        let final_principal_amt =
            half_away_from_zero(initial_principal_amt, config_params.precision())
                * (securitization_percentage / 100.0);
        let final_interest_amount =
            half_away_from_zero(initial_interest_amt, config_params.precision())
                * (securitization_percentage / 100.0);
        *total_balance_sec_output +=
            half_away_from_zero(initial_principal_amt, config_params.precision());

        let mut new_cf = Cashflow::new();
        new_cf.interest_amount =
            half_away_from_zero(final_interest_amount, config_params.precision());
        new_cf.principal_amount =
            half_away_from_zero(final_principal_amt, config_params.precision());
        new_cf.date = new_cf_date.and_hms_opt(0, 0, 0).expect("Error getting date in hms").timestamp();
        cashflows.push(new_cf);

        *ost_bal_sec -= half_away_from_zero(initial_principal_amt, config_params.precision());

        let mut new_date =
            rbdate::increment_date_by_months(prev_cf_date, month_to_incr.try_into().expect("Error while incrementing date"));
        if ei_pay_day == 31 && prev_cf_date.day() == 30 {
            new_date = get_month_end_date(new_date);
        }
        new_cf_date = if new_date >= end_date {
            end_date
        } else {
            new_date
        };
        month_to_incr += inst_period;
    }
    cashflows
}

fn append_cashflows(
    data_map: &mut HashMap<String, Account>,
    curr_data: Account,
    logger: &Logger,
    output: &str,
) {
    let mut prev_cfs = Vec::new();
    if data_map.contains_key(&curr_data.account_number) {
        prev_cfs = data_map
            .get(&curr_data.account_number)
            .expect("Error getting cashflows")
            .cashflows
            .to_vec();
        log_info!(logger, "Appending {} cashflows for Account: {}\nAppending Curr-PI-Amts: ({},{}) into Prev-PI-Amt: ({},{})", 
        output,
        curr_data.account_number,
        curr_data.cashflows.to_vec().iter().fold(0.0, |sum, val| sum + val.principal_amount),
        curr_data.cashflows.to_vec().iter().fold(0.0, |sum, val| sum + val.interest_amount),
        prev_cfs.iter().fold(0.0, |sum, val| sum + val.principal_amount),
        prev_cfs.iter().fold(0.0, |sum, val| sum + val.interest_amount),
        );
    }
    prev_cfs.extend(curr_data.cashflows.to_vec());
    data_map
        .entry(curr_data.account_number.to_string())
        .and_modify(|prev_data| {
            prev_data.cashflows = protobuf::RepeatedField::from_vec(prev_cfs);
            prev_data.tot_int_amt += curr_data.tot_int_amt;
            prev_data.tot_prin_amt += curr_data.tot_prin_amt;
        })
        .or_insert(curr_data);
}
