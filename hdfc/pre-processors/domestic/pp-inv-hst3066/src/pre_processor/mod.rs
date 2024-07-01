use self::appender::get_op_line;
use calamine::{open_workbook, Reader, Xls};
use configuration_parameters::ConfigurationParameters;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
use health_report::HealthReport;

mod appender;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let as_on_dt = config_param.as_on_date();
    let st_tm_read = SystemTime::now();
    let mut output: String = String::new();
    let mut tot_acc_encntrd = 0;
    let mut skipped_accounts = 0;
    let mut input_file: Xls<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    let mut tot_amt = 0.0;
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            tot_acc_encntrd += 1;
            let acc_info = row[0].to_string();
            let mut fields: Vec<&str> = acc_info.split('#').collect();
            if fields[0] == "PORTFOLIO NAME" {
                continue;
            }
            if fields.len() != 7 {
                skipped_accounts += 1;
                continue;
            }
            let amt = fields[5].parse::<f64>().unwrap_or(0.0);
            tot_amt += amt;
            output.push_str(&get_op_line(&mut fields, *as_on_dt, log));
        }
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_write = SystemTime::now();
    let mut op_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match op_writer.write_all(output.as_bytes()) {
        Ok(_) => println!("Successfully processed all lending accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}",
            config_param.output_file_path(),
            error
        ),
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skipped_accounts,
        skipped_accounts,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_write = SystemTime::now();
    let duration = ed_tm_write
        .duration_since(st_tm_write)
        .expect("Could not calculate output write total duration.");
    debug!(
        diag_log,
        "Writing output Records, Total Duration: {:?}.", duration
    );
}
