use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use chrono::NaiveDate;
use chrono::Datelike;
use rbdate::*;
use slog::Logger;
mod io;
use macros;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut tot_rec: u32 = 0;
    let mut succ_rec: u32 = 0;

    let mut total_undrawn_amt = 0.0;
    let mut total_native_amt = 0.0;
    let mut config_llg_mapping: HashMap<String, String> = HashMap::new();
    let mut llg_native_mapping: HashMap<String, f64> = HashMap::new();
    let mut last_mon_llg_native_map: HashMap<String, f64> = HashMap::new();
    let as_on_dt = config_params.as_on_date().format("%d-%m-%Y");

    //Output writer:
    let mut op_p1_writer = get_writer(&config_params.output_file_p1_path());
    let mut op_p2_writer = get_writer(&config_params.output_file_p2_path());

    //Summary_file:
    let summary_file = match new_buf_rdr(config_params.summary_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.summary_file_path(),
            current_dir()
                .expect("Error while getting summary directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in summary_file.lines().enumerate() {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.summary_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();
        total_undrawn_amt += input_fields[3].parse::<f64>().unwrap_or(0.0);
    }

    //Config_file:
    let config_file = match new_buf_rdr(config_params.config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.config_file_path(),
            current_dir()
                .expect("Error while getting config directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in config_file.lines().enumerate() {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.config_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();
        let input_llg = input_fields[0].to_string();
        let output_llg = input_fields[1].to_string();

        config_llg_mapping.insert(input_llg, output_llg);
    }

    //Input_file:
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Error while getting input directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in input_file.lines().enumerate() {
        tot_rec += 1;

        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();

        let llg_id = input_fields[3].to_string();
        let native_amt = input_fields[4].parse::<f64>().unwrap_or(0.0);

        //get output_llg w.r.t llg_id:
        let mut output_llg = "".to_string();
        if config_llg_mapping.contains_key(&llg_id) {
            output_llg = match config_llg_mapping.get(&llg_id) {
                Some(val) => val.to_string(),
                None => {
                    log_warn!(
                        logger,
                        "Values Defaulted as data not found in NPA file for key: {}",
                        llg_id
                    );
                    "".to_string()
                }
            }
        } else {
            continue;
        }

        //get sum of native_amt w.r.t output_llg:
        total_native_amt += native_amt;
        if llg_native_mapping.contains_key(&output_llg) {
            let mut native_total_sum = *llg_native_mapping.get(&output_llg).unwrap();
            native_total_sum += native_amt;
            llg_native_mapping.insert(output_llg, native_total_sum);
        } else {
            llg_native_mapping.insert(output_llg, native_amt);
        }
        succ_rec += 1;
    }

    //writing P1 output:
    for (key, value) in llg_native_mapping.clone() {
        let native_amt = total_undrawn_amt * value / total_native_amt;
        write!(
            op_p1_writer,
            "{}|{}|{}|{}|{:.3}|{:.3}\n",
            as_on_dt,
            config_params.country(),
            config_params.currency(),
            key,
            native_amt,
            native_amt
        )
        .expect("Error in writing output");
    }

    //PROCESS 2:
    let asOnDate = *config_params.as_on_date();
    let output_path = config_params.output_file_p1_path().to_string();
    let curr_date = asOnDate.format("%d%m%Y").to_string();
    let last_eom_date = get_last_eom_date(asOnDate).format("%d%m%Y").to_string();
    let last_eom_file_path = output_path.replace(&curr_date,&last_eom_date);

    if is_month_end_date(asOnDate) {
        for (key, value) in llg_native_mapping {
            write!(
                op_p2_writer,
                "{}|{}|{}|{}|{:.3}|{:.3}\n",
                as_on_dt,
                config_params.country(),
                config_params.currency(),
                key,
                0.0,
                0.0
            )
            .expect("Error in writing output");
        }
    } else {
        let last_eom_file = match new_buf_rdr(&last_eom_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found input file: `{}` on location `{}` : {}.",
                last_eom_file_path,
                current_dir()
                    .expect("Error while getting last month-end directory path.")
                    .display(),
                error
            ),
        };
        println!("Last Month-end file used: {}",last_eom_file_path);
        for (line_num, lines) in last_eom_file.lines().enumerate() {
            let input_line = match lines {
                Ok(input_line) => input_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.config_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let input_fields = input_line.split('|').collect::<Vec<&str>>();
            let llg = input_fields[3].to_string();
            let prev_native_amt = input_fields[4].parse::<f64>().unwrap_or(0.0);

            last_mon_llg_native_map.insert(llg, prev_native_amt);
        }

        for (key, value) in llg_native_mapping {
            let mut final_native_amt = 0.0;
            let native_amt = total_undrawn_amt * value / total_native_amt;
            if last_mon_llg_native_map.contains_key(&key) {
                let prev_nat_amt = *last_mon_llg_native_map.get(&key).unwrap();
                final_native_amt = native_amt - prev_nat_amt;
            }

            write!(
                op_p2_writer,
                "{}|{}|{}|{}|{:.3}|{:.3}\n",
                as_on_dt,
                config_params.country(),
                config_params.currency(),
                key,
                final_native_amt,
                final_native_amt
            )
            .expect("Error in writing output");
        }
    }
}

pub fn get_last_eom_date(date: NaiveDate) -> NaiveDate {
    let mut last_mon_date = date;
    if vec![1].contains(&date.month()) {
        last_mon_date = NaiveDate::from_ymd_opt(date.year()-1, 12, 1).unwrap_or(date);
    }
    else {
        last_mon_date = NaiveDate::from_ymd_opt(date.year(), date.month() - 1, 1).unwrap_or(date);
    }
    get_month_end_date(last_mon_date)
}