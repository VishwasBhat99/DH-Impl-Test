use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

mod exchange_rate;

#[derive(Debug)]
pub struct ConsolCurrency {
    consol_ccy: String,
    display_consol_ccy: String,
}

pub fn converter(config_params: &ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let config_rdr = match new_buf_rdr(config_params.consol_config_file_path()) {
        Ok(rdr) => rdr,
        Err(err) => panic!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.consol_config_file_path, err
        ),
    };
    let mut consol_config_map: HashMap<String, Vec<ConsolCurrency>> = HashMap::new();
    for line in config_rdr.lines() {
        let config_line = line.unwrap();
        let fields: Vec<&str> = config_line.split('|').collect();
        let consol_ccy = ConsolCurrency {
            consol_ccy: fields[1].to_string(),
            display_consol_ccy: fields[2].to_string(),
        };
        if consol_config_map.contains_key(fields[0]) {
            let prev_val = consol_config_map
                .remove(fields[0])
                .expect("Cannot read value for key in consol config map.");
            let mut consols = prev_val;
            consols.push(consol_ccy);
            consol_config_map.insert(fields[0].to_string(), consols);
        } else {
            let consols: Vec<ConsolCurrency> = vec![consol_ccy];
            consol_config_map.insert(fields[0].to_string(), consols);
        }
    }
    let exchange_rate_map =
        exchange_rate::read_exchange_rate(config_params.currency_conversion_file_path());
    let bucket_count = vec![499, 499, 112, 499, 499, 67, 499, 499, 112, 499, 499, 67, 8];
    let mut input_file_name;
    let mut output_file_name;
    for (file_index, _) in bucket_count.iter().enumerate().take(12 + 1) {
        if !config_params.is_maturity() && file_index != 12 {
            continue;
        }
        let mut header_size = if config_params.input_files_type == "BRWISE" {
            6
        } else {
            5
        };
        if file_index == 12 {
            header_size = if config_params.input_files_type == "BRWISE" {
                7
            } else {
                6
            };
            input_file_name = format!("{}-summary.txt", config_params.input_file_path());
            output_file_name = format!("{}-summary-consol.txt", config_params.output_file_path());
        } else {
            input_file_name = format!("{}-{}.txt", config_params.input_file_path(), file_index);
            output_file_name = format!(
                "{}-consol-{}.txt",
                config_params.output_file_path(),
                file_index
            );
        }
        let input_file = File::open(&input_file_name).expect("Cannot open input file.");
        let mut writer = match buf_file_wrtr(&output_file_name, None) {
            Ok(wrtr) => wrtr,
            Err(error) => {
                panic!(
                    "Could not create file: `{}` on location `{}` : {:?}.",
                    config_params.output_file_path(),
                    env::current_exe()
                        .expect("Unable to find current directory path!")
                        .display(),
                    error
                );
            }
        };
        let mut op: HashMap<Vec<String>, Vec<f64>> = HashMap::new();
        for (line_num, line) in BufReader::new(input_file).lines().enumerate() {
            let record = line.expect("Cannot read line from input file.");
            let fields: Vec<&str> = record.split('|').collect();
            let currency = if config_params.input_files_type == "BRWISE" {
                fields[4]
            } else {
                fields[2]
            }
            .to_string();
            let target_consols = match consol_config_map.get(&currency) {
                Some(val) => val,
                None => {
                    log_error!(
                        log,
                        "Consol Currency {} Not Found in Consol Config File.",
                        currency
                    );
                    continue;
                }
            };
            for consol in target_consols {
                let mut new_value_map: HashMap<usize, String> = HashMap::new();
                let exchange_rate = match exchange_rate::get_exch_rate(
                    &currency,
                    &consol.consol_ccy,
                    &exchange_rate_map,
                ) {
                    Ok(val) => val,
                    Err(_) => {
                        log_warn!(
                            log,
                            "Default exchange rate used for: {}",
                            &consol.consol_ccy
                        );
                        1.0
                    }
                };
                for bucket_no in (header_size..=bucket_count[file_index]).step_by(3) {
                    let bucket_value = fields[bucket_no];
                    let current_value: f64 = bucket_value.parse().unwrap_or_else(|_| {
                        panic!(
                            "Error while reading {} as a decimal value.\nInvalid data from {}th column of {}th row(line) from {} file",
                            bucket_value,
                            bucket_no + 1,
                            line_num + 1,
                            input_file_name,
                        )
                    });
                    let converted_value = format!("{:.4}", current_value * exchange_rate);
                    new_value_map.insert(bucket_no, converted_value);
                }
                let mut op_line: String = String::new();
                for (index, field) in fields.iter().enumerate() {
                    if index < header_size {
                        if (index == 4 && config_params.input_files_type == "BRWISE")
                            || (index == 2 && config_params.input_files_type == "BALM")
                        {
                            op_line.push_str(&consol.display_consol_ccy);
                        } else {
                            op_line.push_str(field);
                        }
                    } else {
                        match new_value_map.get(&index) {
                            Some(val) => {
                                op_line.push_str(val);
                            }
                            None => {
                                op_line.push_str(field);
                            }
                        }
                    }
                    op_line.push('|');
                }
                op_line.pop();
                let op_fields: Vec<&str> = op_line.split('|').collect();
                let mut key: Vec<String> = Vec::new();
                for val in &op_fields[0..header_size] {
                    key.push(val.to_string());
                }
                let mut value: Vec<f64> = Vec::new();
                for val in &op_fields[header_size..op_fields.len()] {
                    let f64_val = val.parse().unwrap_or(0.0);
                    value.push(f64_val);
                }
                if op.contains_key(&key) {
                    let prev_val: Vec<f64> = match op.get(&key) {
                        Some(buckets) => buckets.clone(),
                        None => Vec::new(),
                    };
                    let mut final_val: Vec<f64> = Vec::new();
                    for i in (0..prev_val.len()).step_by(3) {
                        let tot_amt = prev_val[i] + value[i];
                        final_val.push(tot_amt);
                        let weighted_int =
                            (prev_val[i] * prev_val[i + 1]) + (value[i] * value[i + 1]);
                        if tot_amt != 0.0 {
                            final_val.push(weighted_int / tot_amt);
                        } else {
                            final_val.push(0.0);
                        }
                        if file_index != 12 {
                            final_val.push(prev_val[i + 2]);
                        }
                    }
                    op.insert(key.clone(), final_val);
                } else {
                    op.insert(key.clone(), value);
                }
            }
        }
        for (key, value) in op.drain() {
            let mut final_op_line: String = String::new();
            for val in key {
                final_op_line.push_str(&val);
                final_op_line.push('|');
            }
            for val in value {
                final_op_line.push_str(&format!("{:.4}", val));
                final_op_line.push('|');
            }
            final_op_line.pop();
            final_op_line.push('\n');
            let output_as_bytes = final_op_line.as_bytes();
            match writer.write(output_as_bytes) {
                Ok(_val) => {}
                Err(err) => println!("Error writing to output file. Error: {}", err),
            }
        }
    }
}
