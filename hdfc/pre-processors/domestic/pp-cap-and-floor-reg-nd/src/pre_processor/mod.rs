use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use std::collections::HashMap;
use std::fs::File;
use std::io::*;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let op_path = format!("{}.txt", config_params.output_file_path());
    let mut op_writer = get_writer(&op_path);
    let op_path_nd = format!("{}_ND.txt", config_params.output_file_path());
    let mut op_writer_nd = get_writer(&op_path_nd);

    let input = File::open(&config_params.input_file_path()).expect("Could Not read input file.");
    let input_reader = BufReader::new(input);
    let mut tot_acc_encntrd = 0;

    let mut ref_map: HashMap<String, _> = HashMap::new();
    let mut ref_excel = open_workbook_auto(config_params.ref_file_path())
        .expect("Unable to open Reference Master File.");
    if let Some(Ok(reader)) = ref_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(1) {
            ref_map.insert(row[4].to_string(), "CAP_N_FLOOR".to_string());
        }
    }

    for (_index, line) in input_reader.lines().enumerate() {
        let line = line
            .expect("Could not read line in input file.")
            .to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        if ref_map.contains_key(&input_fields[31].to_string()) {
            writeln!(op_writer_nd, "{}", line);
        } else {
            writeln!(op_writer, "{}", line);
        }
    }
    let health_report = HealthReport::new(tot_acc_encntrd, tot_acc_encntrd, 0, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
