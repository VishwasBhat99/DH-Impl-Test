use self::derive_fields::get_op_line;
use self::structs::{Records, RecordsWithBalance};
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod manual_handler;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut input_file: Xlsx<_> =
        open_workbook(config_param.input_file_path()).expect("Error while opening Input File.");
    let mut tot_acc_encntrd = DEFAULT_INT;
    let mut tot_acc_skipped = DEFAULT_INT;
    let mut tot_amt_inp = DEFAULT_FLOAT;
    let mut tot_amt_out = DEFAULT_FLOAT;
    let mut data = RecordsWithBalance::new();
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.input_sheet_name()) {
        let mut is_header: bool = true;
        for row in reader.rows() {
            tot_acc_encntrd += 1;
            if is_header {
                is_header = false;
                tot_acc_skipped += 1;
                log_error!(log, "Skipped record: `{:?}", row);
                continue;
            }
            let amt = get_op_line(&row, &mut data);
            tot_amt_inp += amt;
            tot_amt_out += amt;
        }
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    match writer.write_all(data.print().as_bytes()) {
        Ok(_val) => println!("Successfully processed all accounts"),
        Err(error) => {
            panic!("Cannot pre process the input file: {:?}", error);
        }
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skipped,
        tot_acc_skipped,
        tot_amt_inp,
        tot_amt_out,
        0,
    );
    println!("{}", health_report.display());
    info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());
}
