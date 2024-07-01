use self::derive_ex_rt_file::{append_ccy, get_ex_rt_lines};
use self::derive_fields::{append_op_line, get_op_line};
use self::get_ex_rate_config::{write_config_output,write_config_exrt};
use self::structs::CurrencyConverter;
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
use std::fs;
use std::collections::HashMap;

mod derive_ex_rt_file;
mod derive_fields;
mod structs;
mod get_ex_rate_config;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    let mut tot_lines: i64 = 0;
    let mut tot_suc_lines: i64 = 0;
    let mut ex_rt_lines: String = String::new();
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut lcy_equi_rt: f64 = 1.0;
    let mut usl_inr_conv_val: f64 = 0.0;
    for (line_num, lines) in reader.deserialize().enumerate() {
        let ccy: CurrencyConverter = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };

        if ccy.source == config_param.ccy() && ccy.target == "INR" {
            lcy_equi_rt = ccy.ex_rt;
        }
    }
    let config_file = fs::read_to_string((&config_param.config_file_path()).to_string()).expect("Failed to read config file!");
    let mut config_map:HashMap<String, String> = HashMap::new();
    for config_line in config_file.lines() {
        let mut config_str = String::new();
        let config_vec: Vec<&str> = config_line.split("|").collect();
        config_str.push_str(&config_vec[0]);
        config_str.push('|');
        config_str.push_str(&config_vec[1]);
        config_map.insert(config_str,config_vec[2].to_string());
    };    

    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in reader.deserialize().enumerate() {
        let mut ccy: CurrencyConverter = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        let mut input_str = String::new();
        input_str.push_str(&ccy.source);
        input_str.push('|');
        input_str.push_str(&ccy.target);
        if config_map.contains_key(&input_str){
            let key_val = config_map.get(&input_str).unwrap();
            output_line.push_str(&write_config_output(&mut ccy,key_val,*config_param.as_on_date()));
            ex_rt_lines.push_str(&write_config_exrt(&mut ccy,key_val));
        }
        tot_lines += 1;
        if ccy.ex_rt == 0.0 {
            log_error!(
                log,
                "Invalid Exchange Rate format `{:?}` at line number: `{}`.",
                ccy,
                line_num + 1
            );
            continue;
        }
        if ccy.source == config_param.ccy() && ccy.target == config_param.ccy() {
            log_error!(
                log,
                "Duplicate {} currency found : `{:?}` at line number: `{}`.",
                config_param.ccy(),
                ccy,
                line_num + 1
            );
            continue;
        }
        if ccy.source == "INR" {
            usl_inr_conv_val = 1.0 / ccy.ex_rt;
        }
        if ccy.target == "INR" {
            usl_inr_conv_val = ccy.ex_rt;
        }
        output_line.push_str(&get_op_line(
            &mut ccy,
            *config_param.as_on_date(),
            lcy_equi_rt,
            &config_param,
        ));
        ex_rt_lines.push_str(&get_ex_rt_lines(&ccy, lcy_equi_rt, &config_param));
        tot_suc_lines += 1;
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    if usl_inr_conv_val == 1.0 {
        log_error!(
            log,
            "USD to INR equivalent exchange rate and vice versa not present in input file."
        )
    }
    append_op_line(&mut output_line, usl_inr_conv_val, &config_param);

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

    append_ccy(&mut ex_rt_lines, usl_inr_conv_val, &config_param);

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
        tot_lines,
        tot_suc_lines,
        tot_lines - tot_suc_lines,
    );
    let health_report = HealthReport::new(
        tot_lines,
        tot_suc_lines,
        tot_lines - tot_suc_lines,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
