use self::account_with_cashflows::Cashflow;
use self::append_write_cashflows::{append_cashflows, create_io_workers, write_cashflows};
use self::get_cashflow::{get_balance_cashflow, get_securitized_cashflow};
use chrono::{Datelike, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{get_days_from_month, timestamp};
use slog::Logger;
use statics::*;
use std::time::SystemTime;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod get_cashflow;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_op = DEFAULT_FLOAT;
    let mut tot_accs_skipped = DEFAULT_INT;
    let tot_int_op = DEFAULT_FLOAT;
    let start_derive_timer = SystemTime::now();
    let (reader, mut writer, mut writer_sec) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;

    reader_iterator.next(); //skip the header
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
            break;
        }

        let input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;
        tot_prin_op += input_account.principal_os;
        let mut acc_with_cf = append_cashflows(diag_log, &input_account, config_params, log);
        let mut acc_with_cf_sec = acc_with_cf.clone();
        let date = NaiveDate::parse_from_str(&input_account.date, "%d-%m-%Y").unwrap();

        if date > *config_params.as_on_date() {
            let cfdate = timestamp(date);

            //Balance sheet portion cashflow
            get_balance_cashflow(&input_account, &mut acc_with_cf, cfdate);
            write_cashflows(&mut writer, log, diag_log, &acc_with_cf);

            // Securitized portion cashflow
            get_securitized_cashflow(&input_account, &mut acc_with_cf_sec, cfdate);
            write_cashflows(&mut writer_sec, log, diag_log, &acc_with_cf_sec);

            tot_cfs += 1;
        } else {
            tot_accs_skipped += 1;
        }
    }
    writer.close();
    writer_sec.close();

    let end_derive_timer = SystemTime::now();
    let tot_duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total duration for derive timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Accounts Skipped: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_encntrd,
        tot_accs_skipped,
        tot_cfs,
        tot_duration,
        tot_prin_op,
        tot_prin_op,
        tot_int_op,
        tot_int_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd,
        0,
        tot_prin_op,
        tot_prin_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}
