extern crate serde;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use rbdate::get_month_end_date;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut writer = BufWriter::new(output_file);

    let as_on_date = config_param.as_on_date();
    let month_end_date = get_month_end_date(*as_on_date);
    let day_num = if as_on_date == &month_end_date {
        31
    } else {
        as_on_date.day() as i64
    };

    let from_bucket = config_param.from_bucket().parse::<i64>().unwrap_or(0);
    let to_bucket = config_param.to_bucket().parse::<i64>().unwrap_or(0);

    let seasonal_dis_rules_file = match new_buf_rdr(config_param.seasonal_dis_rules_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found inpseasonal dis rules data file: `{}` on location `{}` : {}.",
            config_param.seasonal_dis_rules_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut seasonal_dis_rules_map: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in seasonal_dis_rules_file.lines().enumerate() {
        let seasonal_dis_line = match lines {
            Ok(seasonal_dis_line) => seasonal_dis_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.seasonal_dis_rules_path(),
                line_num + 1,
                error
            ),
        };
        let seasonal_dis_rules_fields = seasonal_dis_line.split('|').collect::<Vec<&str>>();
        //Check for the BucketID and Day_num
        if seasonal_dis_rules_fields[4].parse::<i64>().unwrap_or(0) == day_num
            && (seasonal_dis_rules_fields[3].parse::<i64>().unwrap_or(0) >= from_bucket
                && seasonal_dis_rules_fields[3].parse::<i64>().unwrap_or(0) <= to_bucket)
        {
            if seasonal_dis_rules_map.contains_key(&seasonal_dis_rules_fields[0].to_string()) {
                let temp_pcnt_1 = seasonal_dis_rules_map
                    .get_mut(&seasonal_dis_rules_fields[0].to_string())
                    .expect("Could not fetch seasonal dis rules percentage value from hashmap.");
                *temp_pcnt_1 += seasonal_dis_rules_fields[5].parse::<f64>().unwrap_or(0.0);
            } else {
                seasonal_dis_rules_map.insert(
                    seasonal_dis_rules_fields[0].to_string(),
                    seasonal_dis_rules_fields[5].parse::<f64>().unwrap_or(0.0),
                );
            }
        }
    }
    let distribution_rules_file = match new_buf_rdr(config_param.distribution_rules_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found distribution rules data file: `{}` on location `{}` : {}.",
            config_param.distribution_rules_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut distribution_rules_map: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in distribution_rules_file.lines().enumerate() {
        let distribution_rules_line = match lines {
            Ok(distribution_rules_line) => distribution_rules_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.distribution_rules_path(),
                line_num + 1,
                error
            ),
        };
        let distribution_rules_fields = distribution_rules_line.split('|').collect::<Vec<&str>>();

        if distribution_rules_fields[4].parse::<i64>().unwrap_or(0) >= from_bucket
            && distribution_rules_fields[4].parse::<i64>().unwrap_or(0) <= to_bucket
        {
            if distribution_rules_map.contains_key(&distribution_rules_fields[3].to_string()) {
                let temp_inp2_percentage = distribution_rules_map
                    .get_mut(&distribution_rules_fields[3].to_string())
                    .expect("Could not fetch percentage value from hashmap.");
                *temp_inp2_percentage += distribution_rules_fields[5].parse::<f64>().unwrap_or(0.0);
            } else {
                distribution_rules_map.insert(
                    distribution_rules_fields[3].to_string(),
                    distribution_rules_fields[5].parse::<f64>().unwrap_or(0.0),
                );
            }
        }
    }
    let master_file = match new_buf_rdr(config_param.master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found master data file: `{}` on location `{}` : {}.",
            config_param.master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut master_llg_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields = line.split('|').collect::<Vec<&str>>();
        master_llg_map.insert(fields[1].to_string(), fields[0].to_string());
    }

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in input_file.lines().enumerate() {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };

        let input_fields = input_line.split('|').collect::<Vec<&str>>();

        let mut amt = input_fields[4].parse::<f64>().unwrap_or(0.0);
        let mut amt_lcy = input_fields[5].parse::<f64>().unwrap_or(0.0);

        if let Some(balm_llg) = master_llg_map.get(&input_fields[3].to_string()) {
            if *config_param.is_seasonal() {
                match seasonal_dis_rules_map.get(balm_llg) {
                    Some(seasonal_pcnt) => {
                        amt = amt * seasonal_pcnt / 100.0;
                        amt_lcy = amt_lcy * seasonal_pcnt / 100.0;
                    }
                    None => {
                        if let Some(non_seasonal_pcnt) = distribution_rules_map.get(balm_llg) {
                            amt = amt * non_seasonal_pcnt / 100.0;
                            amt_lcy = amt_lcy * non_seasonal_pcnt / 100.0
                        };
                    }
                };
            } else if let Some(non_seasonal_pcnt) = distribution_rules_map.get(balm_llg) {
                amt = amt * non_seasonal_pcnt / 100.0;
                amt_lcy = amt_lcy * non_seasonal_pcnt / 100.0;
            }
        };
        //Divide the amount and amount_lcy by 100 to get the actual percentage value.

        let op_string = format!(
            "{}|{}|{}|{}|{:.2}|{:.2}\n",
            input_fields[0], input_fields[1], input_fields[2], input_fields[3], amt, amt_lcy
        );
        writer
            .write_all(op_string.as_bytes())
            .expect("Error writing to output file!!");
    }
}
