use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod der_cashflows;

use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::append_cashflows;
use self::append_write_cashflows::create_io_workers;
use self::append_write_cashflows::write_cashflows;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use statics::*;
use std::time::SystemTime;

pub fn derive(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut prev_acc_num: String = String::new();
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;

    let start_time = SystemTime::now();
    let mut cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let mut a_w_cf = AccountWithCashflows::new();
    let mut prev_record = InputAccount::new();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_acc_encntrd
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            if !cfs.is_empty() {
                a_w_cf = append_cashflows(diag_log, &prev_record, &cfs);
                let amts = calc_ttl_cf_amts(&cfs);
                tot_int_in_op += amts.total_interest_amount;
                tot_prin_in_op += amts.total_principal_amount;
                write_cashflows(&mut writer, log, diag_log, &a_w_cf);
                tot_cfs += cfs.len();
                a_w_cf.clear();
                cfs.clear();
            }
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.deal_id
            )],
            derive_cashflows(&mut input_account, config_params, log)
        );
        if prev_acc_num.is_empty() {
            tot_acc_encntrd += 1;
            tot_acc_with_cfs += 1;
            tot_prin_in_ip += input_account.crnt_deal_amt.abs();
            tot_int_in_ip += input_account.int_amt;
            prev_acc_num = input_account.deal_id.to_string();
            cfs.push(cashflows);
            prev_record = input_account.clone();
        } else if prev_acc_num != input_account.deal_id {
            tot_acc_encntrd += 1;
            tot_prin_in_ip += input_account.crnt_deal_amt.abs();
            tot_int_in_ip += input_account.int_amt;
            tot_acc_with_cfs += 1;
            a_w_cf = append_cashflows(diag_log, &prev_record, &cfs);
            let amts = calc_ttl_cf_amts(&cfs);
            tot_cfs += cfs.len();
            tot_int_in_op += amts.total_interest_amount;
            tot_prin_in_op += amts.total_principal_amount;
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            cfs.clear();
            cfs.push(cashflows);
            prev_acc_num = input_account.deal_id.to_string();
            prev_record = input_account.clone();
        } else {
            cfs.push(cashflows);
        }
    }

    writer.close();

    let end_time = SystemTime::now();
    let tot_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_report.gen_health_rpt(config_params.output_file_path());

    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}

struct Amounts {
    total_interest_amount: f64,
    total_principal_amount: f64,
}

fn calc_ttl_cf_amts(cashflows: &Vec<Cashflow>) -> Amounts {
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;
    for cf in cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    Amounts {
        total_interest_amount: tot_int_amt,
        total_principal_amount: tot_prin_amt,
    }
}
