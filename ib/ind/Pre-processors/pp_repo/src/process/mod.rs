use crate::configuration_parameters::ConfigurationParameters;
use crate::process::account::{InputAccount, OutputAccount, master_val};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use rbdate::datevalue_to_naive_date;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
mod account;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    let mut bgl_cgl_map: HashMap<String, String> = HashMap::new();
    let bgl_cgl_file = match new_buf_rdr(config_params.bgl_cgl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.bgl_cgl_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in bgl_cgl_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields = line.split('|').collect::<Vec<&str>>();
        if config_params.is_perf_diagnostics_enabled() {
            info!(
                diag_log,
                "bgl_cgl_map: mapped key: {} to val: {}", fields[0], fields[1]
            );
        }
        bgl_cgl_map.insert(fields[0].to_string(), fields[1].to_string());
    }

    let mut master_map: HashMap<String, master_val> = HashMap::new();
    let mut master_excel = open_workbook_auto(config_params.master_file_path())
        .expect("Unable to open Mapping Master File.");
    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(0) {
            let gl_acc_no = row[0].to_string();
            let grp = row[3].to_string();
            let llg = row[4].to_string();
            if config_params.is_perf_diagnostics_enabled() {
                info!(
                    diag_log,
                    "master_map: mapped key: {} to val: [{}, {}]", gl_acc_no, grp, llg
                );
            }
            master_map.insert(gl_acc_no, master_val::new(grp, llg));
        }
    }

    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let mut output_line = "".to_string();
        tot_rec += 1;
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();

        succ_rec += 1;

        let input_acc = InputAccount::new(input_fields);

        let output_acc = OutputAccount::new(input_acc, config_params, &bgl_cgl_map, &log, &line_num, &master_map);

        writeln!(writer, "{}", account::format_output(&output_acc))
            .expect("Output Line can not be written");
    }
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
