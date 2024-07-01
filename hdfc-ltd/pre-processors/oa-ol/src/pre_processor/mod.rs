use self::derive_fields::*;
use self::io::*;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use statics::*;
use std::io::BufRead;
use std::path::Path;
use std::time::SystemTime;

mod derive_fields;
mod io;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let mut tot_amt = DEFAULT_FLOAT;
    let skp_rec = DEFAULT_INT;

    let start_time = SystemTime::now();
    let mut row_count = 1;
    let mut input_file =
        open_workbook_auto(config_param.input_file_path()).expect("Unable to open Input File.");
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            if row_count == 1 {
                row_count += 1;
                continue;
            }
            tot_rec += 1;
            get_alm_master_data(row, &mut op_line, &mut tot_amt);
        }
    }
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    println!("{}", health_report.display());
    log_info!(log, "{}", &health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let end_time = SystemTime::now();

    let duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing OA OL, Total Duration: {:?}.", duration);
}
