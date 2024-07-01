use configuration_parameters::ConfigurationParameters;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

mod exchange_rate;

pub fn converter(config_params: &ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let exchange_rate_map =
        exchange_rate::read_exchange_rate(config_params.currency_conversion_file_path());
    let bucket_count = vec![499, 499, 112, 499, 499, 67, 499, 499, 112, 499, 499, 67, 8];
    let mut input_file_name;
    let mut output_file_name;
    let mut init_file_idx = 0;
    if !config_params.is_maturity() {
        init_file_idx = 12
    }
    for file_index in init_file_idx..=12 {
        let mut header_size = 5;
        if file_index == 12 {
            header_size = 6;
            input_file_name = format!("{}-summary.txt", config_params.input_file_path());
            output_file_name = format!("{}-summary-converted.txt", config_params.input_file_path());
        } else {
            input_file_name = format!("{}-{}.txt", config_params.input_file_path(), file_index);
            output_file_name = format!(
                "{}-converted-{}.txt",
                config_params.input_file_path(),
                file_index
            );
        }
        let input_file = File::open(&input_file_name).expect("Cannot open input file.");
        let mut writer = match buf_file_wrtr(&output_file_name, None) {
            Ok(wrtr) => wrtr,
            Err(error) => {
                panic!(
                    "Could not create file: `{}` on location `{}` : {:?}.",
                    config_params.input_file_path(),
                    env::current_exe()
                        .expect("Unable to find current directory path!")
                        .display(),
                    error
                );
            }
        };
        for line in BufReader::new(input_file).lines() {
            let record = line.expect("Cannot read line from input file.");
            let fields: Vec<&str> = record.split('|').collect();
            let currency = fields[2].to_string();
            let mut new_value_map: HashMap<usize, String> = HashMap::new();
            let exchange_rate = exchange_rate::get_exch_rate(
                currency,
                config_params.base_currency(),
                &exchange_rate_map,
            );
            for bucket_no in (header_size..=bucket_count[file_index]).step_by(3) {
                let bucket_value = fields[bucket_no];
                let current_value: f64 = bucket_value.parse().expect("Not a floating number.");
                let converted_value = format!("{:.2}", current_value * exchange_rate);
                new_value_map.insert(bucket_no, converted_value);
            }
            let mut op_line: String = String::new();
            for (index, field) in fields.iter().enumerate() {
                match new_value_map.get(&index) {
                    Some(val) => {
                        op_line.push_str(val);
                    }
                    None => {
                        op_line.push_str(field);
                    }
                }
                op_line.push('|');
            }
            op_line.pop();
            op_line.push('\n');
            let output_as_bytes = op_line.as_bytes();
            match writer.write(output_as_bytes) {
                Ok(_val) => {}
                Err(err) => println!("Error writing to output file. Error: {}", err),
            }
        }
    }
}
