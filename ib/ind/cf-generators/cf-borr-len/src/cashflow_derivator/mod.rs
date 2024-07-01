use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;

use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use rbdate::{timestamp, NaiveDate};
// use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::{append_cashflows, create_io_workers, write_cashflows};
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use hashbrown::HashMap;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use sdb_io::*;
use statics::*;
use std::env::*;
use std::io::BufRead;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut prev_acc_num: String = "".to_string();
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;
    //read Repayment file

    let start_derive_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithCashflows::new();

    // reader_iterator.next();
    loop {
        let mut cf: Cashflow = Cashflow::new();
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_acc_encntrd
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }

        let input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;
        tot_acc_with_cfs += 1;

        if input_account.cf_type == *"Principal" {
            tot_prin_in_op += input_account.cf_amount;
            tot_cfs += 1;
            tot_prin_in_ip += input_account.cf_amount;
            cf = new_cashflow(
                0.0,
                input_account.cf_amount,
                if input_account.cf_date.is_some() {
                    rbdate::timestamp(input_account.cf_date.unwrap_or(*config_params.as_on_date()))
                } else {
                    0
                },
            );
        }
        if input_account.cf_type == *"Interest" {
            tot_int_in_op += input_account.cf_amount;
            tot_cfs += 1;
            tot_int_in_ip += input_account.cf_amount;
            cf = new_cashflow(input_account.cf_amount, 0.0, {
                if input_account.cf_date.is_some() {
                    rbdate::timestamp(input_account.cf_date.unwrap_or(*config_params.as_on_date()))
                } else {
                    0
                }
            });
        }
        let cfs: Vec<Cashflow> = vec![cf];
        a_w_cf = append_cashflows(diag_log, &input_account, config_params, &cfs);
        write_cashflows(&mut writer, log, diag_log, &a_w_cf);
        a_w_cf.clear();
    }

    writer.close();

    let end_derive_timer = SystemTime::now();
    let tot_duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total duration for derive timer.");
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
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_with_cfs,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
