use super::calamine::Reader;
use configuration_parameters::ConfigurationParameters;
use serde::Deserialize;
use slog::Logger;
use std::fs::File;
use std::io::BufReader;

mod io;

#[derive(Deserialize, Debug)]
struct Files {
    input_excel_name: String,
    excel_sheet_name: String,
    column_position: usize,
}
#[derive(Deserialize, Debug)]
struct FilesInfo {
    files: Vec<Files>,
}
pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let config_file = File::open(config_params.input_file_config());
    let buf_reader = BufReader::new(config_file.expect("Error while opening the configuration file"));
    let mut output_file = io::create_file(config_params.output_file_path());
    let config_info: FilesInfo = serde_json::from_reader(buf_reader).unwrap();
    let mut col_data: Vec<String> = Vec::new();
    for file in 0..config_info.files.len() {
        let mut workbook = io::open_file(&config_info.files[file].input_excel_name);
        if let Some(Ok(reader)) = workbook.worksheet_range(&config_info.files[file].excel_sheet_name)
        {
            for column in reader.rows().skip(2) {
                let col_no = config_info.files[file].column_position;
                if col_data.contains(&column[col_no].to_string()) {
                    continue;
                }
                col_data.push(column[col_no].to_string());
            }
        }
    }
    io::write_file(col_data, &mut output_file);
}
