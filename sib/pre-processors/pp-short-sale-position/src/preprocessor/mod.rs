use crate::configuration_parameter::ConfigurationParameters;
use crate::macros;
use chrono::{Days, Months};
use rbdate::num_days_start_to_end;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod derive_fields;
use derive_fields::derive_output;
mod hashvalue_struct;
mod output_account;
use hashvalue_struct::OutputDatas;

pub fn dayss(year: i32, month: u32, days: u64, config_params: &ConfigurationParameters) -> i64 {
    let year_day = config_params
        .as_on_date()
        .checked_add_months(Months::new(year as u32 * 12))
        .unwrap_or(*config_params.as_on_date());
    let month_day = year_day
        .checked_add_months(Months::new(month))
        .unwrap_or(*config_params.as_on_date());
    let day_day = month_day
        .checked_add_days(Days::new(days))
        .unwrap_or(*config_params.as_on_date());
    let no_days = num_days_start_to_end(*config_params.as_on_date(), day_day);
    no_days
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let input_file = match File::open(config_params.input_file()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.input_file(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.input_file(),
                error
            );
        }
    };
    let input_file_reader = BufReader::new(input_file);

    let mr_timeband = match File::open(config_params.mr_timeband()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.mr_timeband(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.mr_timeband(),
                error
            );
        }
    };
    let mr_timeband_reader = BufReader::new(mr_timeband);
    let mut master_map: HashMap<(i64, i64), OutputDatas> = HashMap::new();

    for line in mr_timeband_reader.lines() {
        let line = line.expect("Not reading lines from mr_timeband file");
        let line_split: Vec<&str> = line.split('|').collect();
        let startdays = dayss(
            line_split[0].parse::<i32>().unwrap_or(0),
            line_split[1].parse::<u32>().unwrap_or(0),
            line_split[2].parse::<u64>().unwrap_or(0),
            config_params,
        );
        let enddays = dayss(
            line_split[3].parse::<i32>().unwrap_or(0),
            line_split[4].parse::<u32>().unwrap_or(0),
            line_split[5].parse::<u64>().unwrap_or(0),
            config_params,
        );
        master_map.insert(
            (startdays, enddays),
            OutputDatas {
                time_band: line_split[6].to_string(),
                period: line_split[7].to_string(),
                zone: line_split[8].to_string(),
            },
        );
    }

    derive_output(input_file_reader, &master_map, config_params);
}
