use self::derive_fields::get_op_line;
use self::structs::{Balance, BalanceWithDate, Output};
use calamine::{open_workbook, Reader, Xlsx};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;
    let mut tot_acc_skpd = 0;

    let start_derive_timer = SystemTime::now();
    let mut data = BalanceWithDate::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut input_file: Xlsx<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            tot_acc_encntrd += 1;
            if row[1].to_string().parse::<f64>().is_err() {
                tot_acc_skpd += 1;
                log_debug!(log, "Skipped record: `{:?}`.", row);
                continue;
            }
            let amt = get_op_line(&row, &mut data);
            tot_amt_ip += amt;
            tot_amt_op += amt;
        }
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let op_line: Output = data.print(config_param.llg_code(),config_param.currency());
    let mut open_file_path = String::new();
    open_file_path.push_str(config_param.output_file_path());
    open_file_path = open_file_path.replace(".txt", "") + "-opening.txt";
    let mut open_writer = match buf_file_wrtr(&open_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            open_file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match open_writer.write_all(op_line.opening.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to the file `{}`: {}.",
            open_file_path, error,
        ),
    }

    let mut close_file_path = String::new();
    close_file_path.push_str(config_param.output_file_path());
    close_file_path = close_file_path.replace(".txt", "") + "-closing.txt";
    let mut open_writer = match buf_file_wrtr(&close_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            close_file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match open_writer.write_all(op_line.closing.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to the file `{}`: {}.",
            close_file_path, error,
        ),
    }
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing records and reconcilation file.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skpd,
        tot_acc_skpd,
    );
    info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skpd,
        tot_acc_skpd,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
}
