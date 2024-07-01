use self::derive_ex_rt_file::{append_ccy, get_ex_rt_lines};
use self::derive_fields::{append_op_line, get_op_line};
use self::get_ex_rate_config::{write_config_exrt, write_config_output};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_ex_rt_file;
mod derive_fields;
mod get_ex_rate_config;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let file = match new_buf_rdr(config_param.input_file_path()) {
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
    let config_file = fs::read_to_string((&config_param.config_file_path()).to_string())
        .expect("Failed to read config file!");
    let mut config_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut count_map: HashMap<String, String> = HashMap::new();

    for config_line in config_file.lines() {
        let mut config_str = String::new();
        let config_vec: Vec<&str> = config_line.split("|").collect();
        if config_vec.len() != 3 {
            continue;
        }
        config_str.push_str(&config_vec[0]);
        config_str.push('|');
        config_str.push_str(&config_vec[1]);

        if config_map.contains_key(&config_str) {
            let mut configs = config_map.remove(&config_str).unwrap();
            configs.push(config_vec[2].to_string());
            config_map.insert(config_str.clone(), configs);
        } else {
            config_map.insert(config_str, vec![config_vec[2].to_string()]);
        }
    }
    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    let mut ttl_lines: i64 = 0;
    let mut ttl_suc_lines: i64 = 0;
    let mut ex_rt_lines: String = String::new();
    for (line_num, lines) in file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        ttl_lines += 1;
        let mut fields: Vec<&str> = line.split('|').collect();
        if fields.len() == 3 {
            if fields[2].parse::<f64>().is_err() {
                log_error!(
                    log,
                    "Invalid Exchange Rate format `{:?}` at line number: `{}`.",
                    fields,
                    line_num + 1
                );
                continue;
            }
            let mut input_str = String::new();
            input_str.push_str(&fields[0]);
            input_str.push('|');
            input_str.push_str(&fields[1]);
            if config_map.contains_key(&input_str) {
                let key_values = config_map.get(&input_str).unwrap();
                for key_val in key_values.iter() {
                    let count_map_key = format!("{}|{}", &input_str, key_val);
                    if count_map.contains_key(&count_map_key) {
                        continue;
                    } else {
                        count_map.insert(count_map_key.clone(), "".to_string());
                    }
                    output_line.push_str(&write_config_output(
                        &key_val,
                        &fields,
                        *config_param.as_on_date(),
                    ));
                    ex_rt_lines.push_str(&write_config_exrt(&key_val, &fields));
                }
            }
            if fields[0] == "INR" && fields[1] == "INR" {
                log_error!(
                    log,
                    "Duplicate INR currency found : `{:?}` at line number: `{}`.",
                    fields,
                    line_num + 1
                );
                continue;
            }
            output_line.push_str(&get_op_line(&mut fields, *config_param.as_on_date()));
            ex_rt_lines.push_str(&get_ex_rt_lines(&fields));
            ttl_suc_lines += 1;
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    append_op_line(&mut output_line, &config_param);

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match writer.write_all(output_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all lines."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.output_file_path(),
            error,
        ),
    }

    append_ccy(
        &mut ex_rt_lines,
        config_param.ccy(),
        config_param.lcy(),
        config_param.fcy(),
    );

    let mut ex_writer = match buf_file_wrtr(config_param.ex_rt_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create exchange rate file: `{}` on location `{}` : {}",
            config_param.ex_rt_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match ex_writer.write_all(ex_rt_lines.as_bytes()) {
        Ok(_) => println!("Successfully processed all lines."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.ex_rt_file_path(),
            error,
        ),
    }
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
    let report_string = format!(
        "Total lines encountered: {}\n\
         Lines proccessed suceessfully: {}\n\
         Lines failed to process: {}",
        ttl_lines,
        ttl_suc_lines,
        ttl_lines - ttl_suc_lines,
    );
    let health_report = HealthReport::new(
        ttl_lines,
        ttl_suc_lines,
        ttl_lines - ttl_suc_lines,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
