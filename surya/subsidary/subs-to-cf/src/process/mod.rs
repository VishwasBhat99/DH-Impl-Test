use self::process_data::get_cashflows;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
mod cashflow_output;
mod process_data;
mod writer;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut output_file =
        File::create(config_params.output_file_path()).expect("unable to create output file");
    let mut workbook = open_workbook_auto(config_params.input_file_path())
        .expect("Error while opening the input file");
    let days_range: Vec<&str> = config_params.days_range().split(',').collect();
    // TODO: Take config reader out in a function
    let config_rdr = match sdb_io::new_buf_rdr(config_params.config_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.config_file_path(),
            e
        )),
    };
    let mut config_map: HashMap<String, String> = HashMap::new();
    for line in config_rdr.lines() {
        let line_info = match line {
            Ok(line) => line,
            Err(error) => panic!("Unable to config file: {}", error),
        };
        let data: Vec<&str> = line_info.split('|').collect();
        config_map.insert(data[0].to_string(), data[1].to_string());
    }
    let mut ttl_rec_encntrd: i64 = 0;
    let skp_acc: i64 = 0;
    if let Some(Ok(reader)) = workbook.worksheet_range(config_params.input_sheet_name()) {
        let mut row_num = 1;
        for row in reader.rows() {
            if let Some(llg_id) = config_map.get(&row_num.to_string()) {
                let mut data: Vec<f64> = Vec::new();
                for val in 0..days_range.len() {
                    let cell_num = config_params.amt_col_pos() - 1 + val;
                    data.push(
                        row[cell_num].get_float().unwrap_or(0.0) * config_params.denomination(),
                    );
                }
                let cashflows = get_cashflows(&data, &days_range, config_params.as_on_date());
                let output_account_data = writer::get_write_date(llg_id, cashflows);
                writer::write_to_file(&mut output_file, output_account_data);
            }
            row_num += 1;
            ttl_rec_encntrd = row_num;
        }
    }

    let health_report = HealthReport::new(
        ttl_rec_encntrd,
        ttl_rec_encntrd - skp_acc,
        skp_acc,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
