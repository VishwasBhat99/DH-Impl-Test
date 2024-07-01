use self::io::*;
use calamine::{open_workbook_auto, Reader};
use chrono::{Duration, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::config::File;
use rbdate::decr_dt_by_mon_presrv_eom;
use regex::Regex;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;
pub mod config;
mod io;

pub fn validate_file(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let as_on_date = NaiveDate::parse_from_str(&config_params.as_on_date(), "%d%m%Y")
        .expect("Cannot parse as_on_date to a valid NaiveDate type.");
    let date_folder = as_on_date.format("%d%m%Y").to_string();
    let format1 = as_on_date.format("%Y%m%d").to_string();
    let format2 = as_on_date.format("%m%Y").to_string();
    let mut config_reader = open_workbook_auto(&config_params.config_file_path())
        .expect("Unable to open Configuration File.");
    if let Some(Ok(reader)) = config_reader.worksheet_range(config_params.config_sheet_name()) {
        let mut file_list: HashMap<String, File> = HashMap::new();
        //Header is present in config file.
        for row in reader.rows().skip(1) {
            let lookup_file_path = &row[0].to_string();
            let input_file_name = lookup_file_path
                .replace("{ddmmyyyy}", &date_folder)
                .replace("{yyyymmdd}", &format1)
                .replace("{mmyyyy}", &format2);
            file_list.insert(
                input_file_name,
                File {
                    input_sheet_name: row[1].to_string(),
                    validate_flag: row[2].to_string(),
                    percent: row[3].to_string().parse().unwrap_or(0.0),
                    duration: row[4].to_string(),
                },
            );
        }

        //Get the type of the input file.
        let input_file_path = config_params.input_file_path();
        let mut last_element = String::new();
        if input_file_path.contains('.') {
            let v: Vec<&str> = input_file_path.split('.').collect();
            last_element = v.last().unwrap_or(&"na").to_string();
        }
        let file = file_list.get(input_file_path);
        if let Some(file) = file {
            let mut tot_rec = 0;
            match last_element.to_lowercase().as_str() {
                "txt" | "csv" => {
                    let input_reader = read_file(input_file_path, logger);
                    for _line_num in input_reader.lines().enumerate() {
                        tot_rec += 1;
                    }
                }
                "xlsx" | "xls" => {
                    let mut input_reader = open_workbook_auto(&input_file_path)
                        .expect("Unable to open Input xlsx File.");
                    if let Some(Ok(reader)) = input_reader.worksheet_range(&file.input_sheet_name) {
                        for _row in reader.rows() {
                            tot_rec += 1;
                        }
                    }
                }
                _ => {
                    let err_msg = format!(
                        "Unknown File format: `{}` for `{}`.",
                        last_element, input_file_path,
                    );
                    log_error!(logger, "{}", err_msg);
                }
            }
            let health_report = HealthReport::new(tot_rec as i64, tot_rec as i64, 0, 0.0, 0.0, 0);
            log_info!(
                logger,
                "File:{}\n{}",
                input_file_path,
                health_report.display()
            );
            health_report.gen_health_rpt(input_file_path);

            //Check if validation flag for the file is set to true.
            if file.validate_flag.to_lowercase() == "true" || file.validate_flag == "1" {
                //Duration is of the format eg: 1D, 1M or 1Y.
                let mut duration = file.duration.to_string();
                let time_period = duration.chars().last().unwrap();
                duration.pop();
                let duration = duration.parse::<i64>().unwrap_or(0);
                let old_date = match time_period.to_string().to_uppercase().as_str() {
                    "D" => as_on_date - Duration::days(duration),
                    "M" => decr_dt_by_mon_presrv_eom(as_on_date, duration as usize).unwrap(),
                    "Y" => decr_dt_by_mon_presrv_eom(as_on_date, (duration * 12) as usize).unwrap(),
                    _ => as_on_date,
                };
                log_info!(logger, "old date:{}", old_date.format("%d-%m-%Y"));

                let old_date_formatted = old_date.format("%d%m%Y").to_string();
                //Create the back-dated report file name.
                let re = Regex::new("[0-9]{8}").unwrap();
                let old_file_path = re
                    .replace(input_file_path, old_date_formatted)
                    .replace(".txt", "")
                    .replace(".csv", "")
                    .replace(".xlsx", "")
                    .replace(".xls", "")
                    + "-health-check-report.json";
                let old_report = new_buf_rdr(&old_file_path);
                if let Ok(old_report) = old_report {
                    for (line_num, lines) in old_report.lines().enumerate() {
                        //Total number of records is found on the 2nd line of health check report.
                        if line_num == 1 {
                            let mut line: String = match lines {
                                Ok(line) => line.to_string(),
                                Err(error) => panic!(
                                    "Unable to read report `{}` at line number: `{}` : {}",
                                    old_file_path,
                                    line_num + 1,
                                    error
                                ),
                            };
                            //Remove the comma at the end of the line and get the number alone.
                            line.pop();
                            let parts: Vec<&str> = line.split(' ').collect();
                            let old_rows = parts[parts.len() - 1].parse::<i32>().unwrap_or(0);

                            let difference: i64 = (old_rows - tot_rec).abs() as i64;
                            let difference_percent = difference as f64 / tot_rec as f64 * 100.0;
                            log_info!(
                                logger,
                                "Set percentage difference:{} \nDifference percentage of the two files:{}",
                                file.percent,difference_percent
                            );
                            if difference_percent >= file.percent {
                                log_error!(logger, "File {} is not valid!! Percentage difference greater than provided value by {}.",input_file_path,(difference_percent-file.percent) as f64);
                                let panic_msg=format!("File {} is not valid!! Percentage difference greater than provided value by {}.",input_file_path,(difference_percent-file.percent) as f64);
                                panic!("{}", panic_msg);
                            } else {
                                log_info!(
                                    logger,
                                    "Input file is valid. Difference lies within set value."
                                );
                            }
                        }
                    }
                } else {
                    log_error!(
                        logger,
                        "Could not find the report {} for the older date:{}.",
                        old_file_path,old_date.format("%d-%m-%Y");
                    );
                }
            }
        } else {
            log_error!(
                logger,
                "File {} not found in the configuration file.",
                input_file_path
            );
        }
    }
}
