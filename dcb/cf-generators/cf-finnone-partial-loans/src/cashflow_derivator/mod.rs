mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod der_cashflows;
mod tenor_calculations;

use health_report::HealthReport;
use slog::Logger;

use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::append_cashflows;
use self::append_write_cashflows::create_io_workers;
use self::append_write_cashflows::write_cashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use macros;
use protobuf::Clear;
use statics::*;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;

    let start_derive_timer = SystemTime::now();
    let mut cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let mut a_w_cf = AccountWithCashflows::new();
    let mut ost_bal = DEFAULT_FLOAT;
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
            if ost_bal > 0.0 && !cfs.is_empty() {
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

        cfs = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.account_number
            )],
            derive_cashflows(&mut input_account, &config_params, log).unwrap_or_default()
        );
        tot_acc_encntrd += 1;
        tot_acc_with_cfs += 1;
        tot_prin_in_ip += input_account.current_book_balance;
        tot_int_in_ip += input_account.int_amt;
        a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
        let amts = calc_ttl_cf_amts(&cfs);
        tot_int_in_op += amts.total_interest_amount;
        tot_prin_in_op += amts.total_principal_amount;

        write_cashflows(&mut writer, log, diag_log, &a_w_cf);
        tot_cfs += cfs.len();
        a_w_cf.clear();
        cfs.clear();
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