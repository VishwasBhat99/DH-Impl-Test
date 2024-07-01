use crate::configuration_parameters::ConfigurationParameters;
extern crate chrono;
use crate::macros;
use health_report::HealthReport;
use rbdate::DateParser;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::default;
use std::env;
use std::env::current_dir;
use std::fs::File;
use std::hash::Hash;
use std::io::BufWriter;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;
mod config;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_program_timer = SystemTime::now();
    let mut input_file_vec: Vec<String> = Vec::new();
    let mut is_cust_id_null: bool = false;
    let mut is_aorl_null = false;
    let mut top_n_cust: i64 = 0;
    let cust_id_postion = config_params.cust_id_position();
    let cust_name_postion = config_params.cust_name_position();
    let as_on_month = *config_params.as_on_date();
    let files_config = config::get_files(config_params.config_file_path());
    let mut tot_acc_encntrd = 0;
    let mut tot_acct_in_topn = 0;
    let mut exc_llg_vec: Vec<String> = Vec::new();
    for config_fields in files_config.files {
        input_file_vec = config_fields.input_file_path;
        is_cust_id_null = config_fields.is_cust_id_null;
        is_aorl_null = config_fields.is_aorl_null;
        top_n_cust = config_fields.top_n_customer;
        exc_llg_vec = config_fields.exclude_llg;
    }
    let cust_def_file = match new_buf_rdr(config_params.cust_def_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.cust_def_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut grouping_map: HashMap<String, f64> = HashMap::new();
    for input_file_path in input_file_vec {
        let start_time = SystemTime::now();
        let input_file = match new_buf_rdr(&input_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found input file: `{}` due to error :{}.",
                config_params.config_file_path(),
                error
            ),
        };
        for (line_num, lines) in input_file.lines().enumerate().skip(1) {
            tot_acc_encntrd += 1;
            let input_line = match lines {
                Ok(input_line) => input_line,
                Err(error) => panic!(
                    "Unable to read file `{}` due to error : `{}`",
                    input_file_path, error
                ),
            };
            let input_fields = input_line.split('|').collect::<Vec<&str>>();
            if input_fields.len() < 60 {
                info!(
                    logger,
                    "Current field for file :{} does not match the required field at row number :{}", input_file_path,line_num
                );
                break;
            }
            let exclude_llg_val = input_fields[43].to_string();
            if exc_llg_vec.contains(&exclude_llg_val) {
                continue;
            }
            let as_on_month = &chrono::NaiveDate::parse_from_str(input_fields[0], "%d-%m-%y")
                .unwrap_or(*config_params.as_on_date());
            if as_on_month == config_params.as_on_date() {
                let cust_id = input_fields[42].to_string();
                let mut profit = 0.0;
                let act_int_amt_hcy = input_fields[9].parse::<f64>().unwrap_or(0.0);
                let ftpamt_hcy = input_fields[36].parse::<f64>().unwrap_or(0.0);
                if !is_aorl_null {
                    if input_fields[37].trim().to_uppercase() == "A" {
                        profit = act_int_amt_hcy - ftpamt_hcy;
                    } else {
                        profit = ftpamt_hcy - act_int_amt_hcy;
                    }
                }
                profit += grouping_map.get(&cust_id).unwrap_or_else(|| &0.0);
                grouping_map.insert(cust_id.to_string(), profit);
            }
        }
        let end_timer = SystemTime::now();
        let process_duration = end_timer
            .duration_since(start_time)
            .expect("Could not calculate total duration.");
        info!(
            logger,
            "Time for processing the input file {:?}, will be {:?}",
            input_file_path,
            process_duration
        );
    }
    let mut sorted_vec: Vec<(&String, &f64)> = grouping_map.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let mut rank_map: HashMap<&String, i64> = HashMap::new();
    for (index, (key, value)) in sorted_vec.iter().enumerate() {
        let rank = index as i64;
        let cust_info = rank_map.insert(key, rank + 1);
    }

    let len_vec = sorted_vec.len();
    let mut sorted_vec: Vec<(&String, &f64)> = grouping_map.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let mut tot_customer_enc = 0;
    let mut tot_cust_data = 0;
    let mut op_line: String = String::new();
    let mut flag = false;
    for (line_num, lines) in cust_def_file.lines().enumerate().skip(1) {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.cust_def_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();
        let cust_id = input_fields[0].to_string();
        let cust_name = match input_fields.get(1) {
            Some(name) => name.to_string(),
            None => "NA".to_string(),
        };
        let mut found = false;
        if rank_map.contains_key(&cust_id) {
            let customer_id = cust_id.to_string();
            let mut rank = &rank_map.get(&cust_id).unwrap_or(&0).clone();
            if rank <= &&top_n_cust {
                tot_customer_enc += 1;
                op_line.push_str(&format!(
                    "{}|{}|{}|{}\n",
                    as_on_month.format("%d-%m-%Y"),
                    &cust_id,
                    rank.to_string(),
                    cust_name,
                ));
                rank_map.remove(&customer_id);
                // Exit the loop once found
                if tot_customer_enc >= top_n_cust {
                    flag = true;
                    break;
                }
            } else {
                continue;
            }
        }
        if tot_customer_enc >= top_n_cust {
            break;
        }
    }
    for (cust_id, rank) in rank_map {
        if rank <= top_n_cust && tot_customer_enc <= top_n_cust {
            op_line.push_str(&format!(
                "{}|{}|{}|{}\n",
                as_on_month.format("%d-%m-%Y"),
                cust_id,
                rank,
                "NA",
            ));
            tot_customer_enc += 1;
        }
    }

    let mut op_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match op_writer.write_all(op_line.as_bytes()) {
        Ok(_) => info!(logger, "Successfully written outputfile."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_params.output_file_path(),
            error
        ),
    }
    let total_program_end_timer = SystemTime::now();
    let process_duration = total_program_end_timer
        .duration_since(start_program_timer)
        .expect("Could not calculate total duration.");
    info!(
        logger,
        "Time for processing the entire input file will be {:?}", process_duration
    );
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_customer_enc,
        tot_acc_encntrd - tot_customer_enc,
        0.0,
        0.0,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
