extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::InputAccount;
use self::input_account::MasterSheetAccount;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;
mod input_account;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let mut op_line: String = String::new();
    let mut master_map: HashMap<String, MasterSheetAccount> = HashMap::new();
    let mut master_excel = open_workbook_auto(config_params.master_file_path())
        .expect("Unable to open Mapping Master File.");
    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(0) {
            let mut master_acc = MasterSheetAccount::new();
            master_acc.gl_acc_no = row[0].to_string();
            master_acc.description = row[1].to_string();
            master_acc.classification = row[2].to_string();
            master_acc.group = row[3].to_string();
            master_acc.llg = row[4].to_string();
            master_acc.other_llg_classification = row[5].to_string();
            master_acc.logic = row[6].to_string();
            master_map.insert(row[0].to_string(), master_acc);
        }
    }

    let mut writer = BufWriter::new(output_file);

    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_params.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in reader.deserialize().enumerate().skip(1) {
        if line_num == 0 {
            continue;
        }
        tot_rec += 1;
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        succ_rec += 1;
        let temp_string = get_op_line(
            &input_account,
            config_params.as_on_date,
            &master_map,
            config_params.input_date_format().to_string(),
        );
        op_line.push_str(temp_string.as_str());
        op_line.push('\n');
    }

    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
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
