use self::derive_op_fields::*;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
mod derive_op_fields;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_timer = SystemTime::now();
    let mut master_accnos: HashSet<String> = HashSet::new();
    let date_parser1 = DateParser::new("%d-%m-%y".to_string(), false);

    let mut op_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create ouput file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let cf_gen_file = match new_buf_rdr(config_param.cf_gen_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found cf generic file: `{}` on location `{}` : {}.",
            config_param.cf_gen_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    //get cf data
    let mut cf_file_date: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (index, lines) in cf_gen_file.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.cf_gen_file_path(),
                    index + 1,
                    error
                );
                continue;
            }
        };

        let mut cf_record: Vec<String> = Vec::new();
        for component in line.split('|') {
            cf_record.push(component.to_string());
        }

        let key = cf_record[2].to_string().split('.').collect::<Vec<&str>>()[0].to_string();

        if cf_file_date.contains_key(&key.to_owned()) {
            cf_file_date
                .get_mut(&key.to_string())
                .as_mut()
                .expect("Cannot parse Cashflow Date")
                .push(cf_record);
        } else {
            let mut cf_val: Vec<Vec<String>> = Vec::new();
            cf_val.push(cf_record);
            cf_file_date.insert(key.to_string(), cf_val);
        }
    }

    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let mut op_line: String = String::new();
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let info: Vec<&str> = line.split('~').collect();
        if info.len() < 83 {
            log_info!(
                log,
                "Skipping line: {} as it does not have enough fields",
                line_num + 1
            );
            continue;
        }
        let mut ip_fields: Vec<&str> = Vec::new();
        for data in info {
            ip_fields.push(data.trim());
        }

        master_accnos.insert(ip_fields[1].to_string());

        let cf_data = match cf_file_date.get(&ip_fields[1].to_string()) {
            Some(val) => val,
            None => {
                log_info!(
                    log,
                    "Cashflow data not found for trade id: {}",
                    ip_fields[1]
                );
                continue;
            }
        };

        get_op_line(cf_data, &mut ip_fields, &mut op_line);
        get_last_line(cf_data, &mut ip_fields, &mut op_line);

        op_writer
            .write_all(op_line.as_bytes())
            .expect("Cannot write ouput to output file.")
    }

    let mut op_line = String::new();
    for (key, val) in cf_file_date {
        for record in val {
            let cf_date = date_parser1
                .parse_opt(&record[30].replace('/', &'-'.to_string()))
                .expect("Cannot parse cashflow date");
            let flow_amt = format!("{:.4}", record[29].parse::<f64>().unwrap_or(0.0));
            if record[10] == "OPT"
                && ((record[24] == "CAP"
                    && (record[25] != "IPAY" || record[28] != "INT")
                    && (record[25] != "XIT" || record[28] != "INT")
                    && (record[25] != "IPAY" || record[28] != "MID" || record[26] != "ADD")
                    && (record[25] != "IPAY" || record[28] != "ADD"))
                    || (record[24] == "BRK"
                        && (record[25] != "BFEE" || record[28] != "INT")
                        && (record[25] != "CIFE" || record[28] != "INT")))
                && flow_amt.parse::<f64>().unwrap_or(0.0) != 0.0
            {
                log_debug!(log, "cf data{:?}", key);
                op_line.push_str(&record[1]);
                op_line.push('|');
                op_line.push_str(record[2].to_string().split('.').collect::<Vec<&str>>()[0]);
                op_line.push('|');
                op_line.push_str(&record[3]);
                op_line.push('|');
                append_input_fields(&mut op_line, 3_usize, 16_usize);

                if record[11] == *"INT".to_string() {
                    op_line.push_str("I|");
                } else {
                    op_line.push_str("E|");
                }

                op_line.push('|');

                op_line.push('|');

                op_line.push('|');
                op_line.push('|');
                append_input_fields(&mut op_line, 22_usize, 36_usize);

                op_line.push('|');
                append_input_fields(&mut op_line, 38_usize, 69_usize);

                op_line.push('|');

                if !record[24].is_empty() {
                    op_line.push_str(&record[24]);
                } else {
                    op_line.push_str("NA");
                }
                op_line.push('|');
                if !record[25].is_empty() {
                    op_line.push_str(&record[25]);
                } else {
                    op_line.push_str("NA");
                }
                op_line.push('|');
                if !record[26].is_empty() {
                    op_line.push_str(&record[26]);
                } else {
                    op_line.push_str("NA");
                }
                op_line.push('|');
                if !record[27].is_empty() {
                    op_line.push_str(&record[27]);
                } else {
                    op_line.push_str("NA");
                }
                op_line.push('|');
                if !record[28].is_empty() {
                    op_line.push_str(&record[28]);
                } else {
                    op_line.push_str("NA");
                }
                op_line.push('|');
                if !record[29].is_empty() {
                    op_line.push_str(&record[29]);
                } else {
                    op_line.push('0');
                }
                op_line.push('|');
                if !record[31].is_empty() {
                    op_line.push_str(&record[31]);
                } else {
                    op_line.push('0');
                }
                op_line.push('|');
                op_line.push_str(&cf_date.format("%d-%m-%Y").to_string());
                op_line.push('|');
                op_line.push('|');
                op_line.push('|');

                op_line.push('|');
                append_input_fields(&mut op_line, 77_usize, 85_usize);
                // op_line.push('3');
                op_line.pop();
                op_line.push('\n');
            }
        }
    }
    op_writer
        .write_all(op_line.as_bytes())
        .expect("Cannot write output to output file");
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
}

fn append_input_fields(op_line: &mut String, start_index: usize, end_index: usize) {
    for _index in start_index..end_index + 1 {
        op_line.push('|');
    }
}
