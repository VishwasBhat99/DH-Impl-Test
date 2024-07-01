use self::get_close_accounts::*;
use self::get_dates::GetDates;
use self::input_account::{InputAccount, InputParsedAccount};
use self::io::*;
use self::reader::get_data;
use self::structs::AggregateData;
use self::writer::get_op_line;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::time::SystemTime;

mod get_close_accounts;
mod get_dates;
mod input_account;
mod io;
mod reader;
mod structs;
mod writer;

pub fn process(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let dates = GetDates::new(config_params.as_on_date());

    let mut writer = get_writer(config_params.output_file_path());

    let mut ttl_amt = DEFAULT_FLOAT;
    let mut bal_org: HashMap<String, AggregateData> = HashMap::new();
    let mut account_pool: HashMap<String, Vec<f64>> = HashMap::new();
    let mut closed_acc_nos: Vec<String> = Vec::new();

    get_data(&dates, &config_params, &mut bal_org, &mut account_pool, log);
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_write_timer = SystemTime::now();
    get_op_line(
        dates.no_of_days as f64,
        &mut bal_org,
        &mut ttl_amt,
        &mut writer,
    );

    get_closed_acc_nos(&mut account_pool, &mut closed_acc_nos);
    let end_write_timer = SystemTime::now();

    let mut closed_acc_writer = get_writer(config_params.close_accs_file_path());
    write_closed_accounts_data(&mut closed_acc_nos, &mut closed_acc_writer);
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
    let health_report = HealthReport::new(
        bal_org.len() as i64,
        bal_org.len() as i64,
        DEFAULT_INT,
        ttl_amt,
        ttl_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        bal_org.len(),
        bal_org.len(),
        DEFAULT_INT,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
