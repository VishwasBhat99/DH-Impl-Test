use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;
use structs::{get_amts, write_output, Account, MasterMapping};

mod structs;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut skip_rec_count = 0;
    let mut op_writer = get_writer(config_params.output_file_path());

    //Reading Mapping Master Data File
    let mut mapping_master = open_workbook_auto(config_params.mapping_master_file())
        .expect("Could Not Read Mapping Master Data File");
    let mut mapping_master_map: HashMap<String, MasterMapping> = HashMap::new();
    if let Some(Ok(reader)) = mapping_master.worksheet_range(config_params.mapping_sheet_name()) {
        for mapping_master_data in reader.rows().skip(1) {
            let gl_code = mapping_master_data[0].to_string();
            let mapping_data = MasterMapping::new(mapping_master_data);
            if !gl_code.is_empty() {
                mapping_master_map
                    .entry(gl_code.to_string())
                    .or_insert_with(|| mapping_data);
            }
        }
    }
    //Reading Input Master File
    let input = File::open(config_params.input_file_path()).expect("Could Not Read Input File");
    let input_reader = BufReader::new(input);
    for (line_no, line) in input_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    _log,
                    "Cannot read line {} from input file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let input_fields: Vec<&str> = acc_info.split('|').collect();
        if input_fields.len() != 20 {
            log_error!(
                _log,
                "Account: {:?} with Incorrect Column Count: {:?} found in Line Number: {:?}, Expected 20 Columns for each account",
                acc_info,
                input_fields.len(),
                line_no + 1,
            );
        }
        let gl_code = input_fields[1].to_string().trim().to_string();
        let mut acc = Account::new(input_fields);
        acc.ccy = gl_code[5..8].to_string();
        let gl_lookup = gl_code[8..].to_string();
        if !mapping_master_map.contains_key(&gl_lookup.to_string()) {
            log_warn!(
                _log,
                "Defaulting group and llg as data not found for GL: `{}` 
                    in Master-Mapping-File for Acct:`{}`",
                gl_lookup,
                acc.key_1.to_string()
            );
        } else {
            acc.group = mapping_master_map
                .get(&gl_lookup.to_string())
                .unwrap_or(&MasterMapping::def())
                .group
                .to_string();
            acc.llg = mapping_master_map
                .get(&gl_lookup.to_string())
                .unwrap_or(&MasterMapping::def())
                .llg
                .to_string();
        }
        get_amts(&mut acc, config_params);
        let output = write_output(&acc);
        op_writer
            .write_all(output.as_bytes())
            .expect("Error writing records to output path!!");
    }

    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    }
}
