use self::get_dates::GetDates;
use self::input_account::{InputAccount, InputParsedAccount};
use self::reader::get_data;
use self::structs::AggregateData;
use self::writer::get_op_line;
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::time::SystemTime;

mod get_dates;
mod input_account;
mod reader;
mod structs;
mod writer;

pub fn generate_averages(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let dates = GetDates::new(config_params.as_on_date());

    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    let mut ttl_amt = DEFAULT_FLOAT;
    let mut bal_org: HashMap<String, AggregateData> = HashMap::new();
    get_data(&dates, &config_params, &mut bal_org, log);
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(diag_log, "Reading Files, Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    get_op_line(
        dates.no_of_days as f64,
        &mut bal_org,
        &mut ttl_amt,
        &mut writer,
    );
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for FTP Consolidation.");
    debug!(diag_log, "Writing Records, Total Duration: {:?}.", duration);

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
