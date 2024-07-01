use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod config;
use calamine::{open_workbook_auto, Reader};
use rbdate::DateParser;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use chrono::{NaiveDate,NaiveTime};

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let all_files = config::get_all_files(&config_params.input_file_path());
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut output_file = File::create(&config_params.output_file_path()).expect("Create Failed.");
    for value in all_files.files {
        let mut output: String = String::new();
        let mut workbook = open_workbook_auto(&value.input_file).expect("Cannot open file");
        let input_file_info = Command::new("stat")
            .arg(&value.input_file)
            .output()
            .expect("stat command failed to start");
        let cmd_output = String::from_utf8(input_file_info.stdout).unwrap();
        let info: Vec<&str> = cmd_output.split('\n').collect();
        let time_stamp: Vec<&str> = info[4].split(": ").collect();
        let time = NaiveTime::parse_from_str(time_stamp[1],"%Y-%m-%d %H:%M:%S%.3f %z").unwrap().to_string();
        let date = NaiveDate::parse_from_str(time_stamp[1],"%Y-%m-%d %H:%M:%S%.9f %z").unwrap().to_string();
        let new_time: Vec<&str> = time.split(".").collect();

        if let Some(Ok(range)) = workbook.worksheet_range(&value.sheet_name) {
            for row in range.rows() {
                output.push_str(&config_params.as_on_date().to_string());
                output.push('|');
                output.push_str(&value.master_name);
                output.push('|');
                output.push_str(&row[0].to_string());
                output.push('|');
                output.push_str(&row[1].to_string());
                output.push('|');
                let dt = date_parser.parse(&row[2].to_string()).format("%d-%m-%Y");
                output.push_str(&dt.to_string());
                output.push('|');
                output.push_str(&(date.to_string()+" "+new_time[0]));
                output.push_str("\n");
            }
        }
        output_file
            .write_all(output.as_bytes())
            .expect("Cannot write to Output file");
    }
}
