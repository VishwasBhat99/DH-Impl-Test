use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use std::collections::HashMap;
use std::fs::File;
use std::io::*;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let output_path = format!("{}.txt", &config_params.output_file_path());
    let mut op_writer = get_writer(&output_path);
    let output_path_nd = format!("{}_ND.txt", &config_params.output_file_path());
    let mut op_writer_nd = get_writer(&output_path_nd);

    let input = File::open(&config_params.input_file_path()).expect("Could Not Read Input File");
    let input_reader = BufReader::new(input);
    let mut tot_acc_encntrd = 0;

    let mut ref_map: HashMap<String, _> = HashMap::new();
    let mut ref_file = open_workbook_auto(config_params.ref_file_path())
        .expect("Unable to open Reference File.");
    if let Some(Ok(reader)) = ref_file.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(1) {
            ref_map.insert(row[1].to_string(), "CCY_IRS".to_string());
        }
    }

    for line in input_reader.lines() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        if ref_map.contains_key(&input_fields[67].to_string()) {
            write!(op_writer_nd, "{}\n", line);
        } else {
            write!(op_writer, "{}\n", line);
        }
    }
    let health_report = HealthReport::new(tot_acc_encntrd, tot_acc_encntrd, 0, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
