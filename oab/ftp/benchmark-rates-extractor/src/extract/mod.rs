use configuration_parameters::ConfigurationParameters;
use extract::rates_key::RatesKey;
use extract::rates_value::RatesValue;
use macros;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use sdb_util::expand;
use slog::Logger;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;

mod rates_key;
mod rates_value;

pub fn extract_rates(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut date_wise_data: HashMap<NaiveDate, Vec<String>> = HashMap::new();
    let parse_from_str = NaiveDate::parse_from_str;
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not find file `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    // We need to find the start_date and end_date
    let mut start_date = NaiveDate::from_ymd(2099, 12, 31);
    let mut end_date = NaiveDate::from_ymd(1970, 1, 1);
    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num,
                    error
                );

                "".to_string()
            }
        };

        if line.matches("|").count() != 4 {
            log_error!(log, "Line {} not well formatted", line_num + 1);
            continue;
        }
        // Split the string at index
        let (date, data) = line.split_at(11);
        let date = match parse_from_str(&date, "%d-%m-%Y|") {
            Ok(val) => val,
            Err(err) => panic!("Error in parsing date. Error: {}", err),
        };
        if date < start_date {
            start_date = date;
        }
        if date > end_date {
            end_date = date;
        }
        if start_date < *config_params.as_on_date() {
            end_date = *config_params.as_on_date()
        }
        match date_wise_data.entry(date) {
            Entry::Vacant(e) => {
                e.insert(vec![data.to_string()]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(data.to_string());
            }
        }
    }

    // Copy data from previous date, for each missing date between start_date and end_date
    let mut date = start_date;
    while date <= end_date {
        if !date_wise_data.contains_key(&date) {
            let date_data: Vec<String> = match date_wise_data.get(&date.pred()) {
                Some(val) => val.to_vec(),
                None => Default::default(),
            };
            date_wise_data.insert(date, date_data);
            debug!(
                diag_log,
                "Data for the date {} duplicated from the previous date", date
            );
        }
        date = date.succ();
    }

    // Store Data for each file
    let mut rates: HashMap<RatesKey, Vec<RatesValue>> = HashMap::new();
    for (date, date_data) in &date_wise_data {
        for line in date_data.iter() {
            let fields: Vec<String> = expand(line.to_string(), '|');

            let rate_key = RatesKey::new(fields[0].parse().unwrap_or(0), *date);
            let rate_value = RatesValue::new(
                fields[1].parse().unwrap_or(0),
                fields[2].to_string(),
                fields[3].parse().unwrap_or(0.0),
            );

            match rates.entry(rate_key) {
                Entry::Vacant(e) => {
                    e.insert(vec![rate_value]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(rate_value);
                }
            }
        }
    }

    // Write the files for each date and curve_id
    for (key, value) in &rates {
        let mut op_line = String::new();
        let filepath = format!(
            "{}{}_{}.txt",
            &config_params.output_file_path(),
            key.as_on.format("%d-%m-%Y"),
            key.curve_id
        );
        debug!(diag_log, "Writing Records to file: {}.", filepath);
        let mut out_writer = match buf_file_wrtr(&filepath, None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file `{}` on location `{}` : {}",
                filepath,
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error
            ),
        };

        for rec in value {
            let line = format!("{}|{}|{}", rec.period, rec.uom, rec.rate);
            op_line.push_str(&line);
            op_line.push_str("\n");
        }

        match out_writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => panic!(
                "Unable to write processed lines to output file `{}`: {}.",
                config_params.output_file_path(),
                error
            ),
        };
    }
}
