extern crate serde;
use self::format::get_op_line;
use self::format::*;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
mod format;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut tot_acc = 0;
    let mut tot_succ = 0;
    let mut ttl_amt = 0.0;
    let mut tot_cfs = 0;

    let mut writer = BufWriter::new(output_file);
    let cashflow_file = match new_buf_rdr(config_param.cashflow_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.cashflow_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut cashflow_map: HashMap<String, Vec<CashflowData>> = HashMap::new();
    for (line_num, lines) in cashflow_file.lines().enumerate() {
        let cashflow_line = match lines {
            Ok(cashflow_line) => cashflow_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cashflow_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = cashflow_line.split('|').collect();
        let cashflow = get_cashflow_data(&fields);
        cashflow_map
            .entry(fields[1].to_string())
            .and_modify(|cf| cf.push(cashflow.to_owned()))
            .or_insert(vec![cashflow]);
    }

    let gl_file = match new_buf_rdr(config_param.investments_gl_master()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.investments_gl_master(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut gl_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in gl_file.lines().enumerate() {
        let gl_line = match lines {
            Ok(gl_line) => gl_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.investments_gl_master(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = gl_line.split('|').collect();
        let key = format!("{}_{}", fields[0], fields[1]);
        gl_map.insert(key, fields[2].to_string());
    }

    let mapping_master = match new_buf_rdr(config_param.mapping_master()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.mapping_master(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut master_map: HashMap<String, Vec<MappingMaster>> = HashMap::new();
    for (line_num, lines) in mapping_master.lines().enumerate() {
        let mapping_line = match lines {
            Ok(mapping_line) => mapping_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.mapping_master(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = mapping_line.split('|').collect();
        let mapping_acc = MappingMaster {
            c_d: fields[3].to_string(),
            group: fields[4].to_string(),
            llg: fields[5].to_string(),
            other_llg_classification: fields[6].to_string(),
        };
        master_map
            .entry(fields[0].to_string())
            .and_modify(|mapping_val| mapping_val.push(mapping_acc.to_owned()))
            .or_insert_with(|| vec![mapping_acc.to_owned()]);
    }

    //Sort the cashflows based on cf-date.
    for (_, cashflow_list) in cashflow_map.iter_mut() {
        cashflow_list.sort_by(|a, b| a.cf_date.cmp(&b.cf_date));
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

    for (line_no, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.input_file_path(),
                line_no + 1,
                error
            ),
        };
        tot_acc += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let default_cf_value = vec![CashflowData {
            cf_type: "PRINCIPAL".to_string(),
            cf_sub_type: fields[2].to_string(),
            cf_amount: fields[4].parse::<f64>().unwrap_or(0.0),
            currency: "INR".to_string(),
            cf_date: NaiveDate::parse_from_str(fields[11], "%d-%m-%Y")
                .unwrap_or(NaiveDate::from_ymd(1900, 1, 1)),
        }];

        let gl_key = format!("{}_{}", fields[2], fields[6]);
        let default_gl_code = "9999".to_string();
        let gl_code = gl_map.get(&gl_key).unwrap_or(&default_gl_code);
        let cf_vec: &Vec<CashflowData> = match cashflow_map.get(&fields[1].to_string()) {
            Some(cashflow_vec) => cashflow_vec,
            None => &default_cf_value,
        };
        let default_mapping_val = MappingMaster {
            c_d: "".to_string(),
            group: "".to_string(),
            llg: "".to_string(),
            other_llg_classification: "".to_string(),
        };
        let mut mapping_val = default_mapping_val;
        match master_map.get(gl_code) {
            Some(master_val) => {
                for val in master_val.iter() {
                    if fields[4].parse::<f64>().unwrap_or(0.0) < 0.0
                        && val.c_d.to_uppercase() == 'C'.to_string()
                    {
                        mapping_val = val.to_owned();
                    } else if fields[4].parse::<f64>().unwrap_or(0.0) >= 0.0
                        && val.c_d.to_uppercase() == 'D'.to_string()
                    {
                        mapping_val = val.to_owned();
                    }
                }
            }
            None => {
                log_error!(
                    log,
                    "Cannot fetch mapping master data for acc:{}",
                    fields[13]
                )
            }
        };

        for cf in cf_vec.iter() {
            tot_cfs += 1;
            ttl_amt += cf.cf_amount;
            let op_line = get_op_line(&fields, cf, gl_code.to_string(), mapping_val.to_owned());
            match writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
        }
        tot_succ += 1;
    }
    let health_report = HealthReport::new(
        tot_acc,
        tot_succ,
        tot_acc - tot_succ,
        ttl_amt,
        ttl_amt,
        tot_cfs,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}