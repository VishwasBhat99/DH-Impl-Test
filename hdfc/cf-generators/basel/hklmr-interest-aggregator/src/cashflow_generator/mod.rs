mod account_as_cashflows;
mod account_field_names;
mod account_writer;
mod cashflow_appender;
pub mod config;
pub mod derive_fields;
pub mod reader;

use self::cashflow_appender::append_data;
use cashflow_generator::account_field_names::AccFieldNames;
use cashflow_generator::account_writer::AccountWriter;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::increment_date_by_months;
// use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let files_config = config::get_files(config_params.config_file_path());
    let mut writer = AccountWriter::new(&files_config.output_file_path, log);

    let mut tot_rec = 0;
    let skp_rec = 0;
    let tot_amt = 0.0;
    for file in files_config.files {
        let mut deal_id_map: HashMap<String, String> = HashMap::new();
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);
        let cust_master_file = match new_buf_rdr(&file.cust_master_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                &file.cust_master_file_path,
                current_dir()
                    .expect("Error while Customer Master file path.")
                    .display(),
                error
            ),
        };
        let mut cust_master: HashMap<String, String> = HashMap::new();
        for (line_num, lines) in cust_master_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    &file.cust_master_file_path,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split(',').collect();
            cust_master.insert(fields[0].trim().to_string(), fields[1].trim().to_string());
        }
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let mut from_bucket_days = if file.from_period.contains("-") {
            -1 * get_days(
                file.from_period.replace("-", "").as_str(),
                config_params.as_on_date(),
            )
        } else {
            get_days(&file.from_period, config_params.as_on_date())
        };
        let mut to_bucket_days = if file.to_period.contains("-") {
            -1 * get_days(
                file.to_period.replace("-", "").as_str(),
                config_params.as_on_date(),
            )
        } else {
            get_days(&file.to_period, config_params.as_on_date())
        };

        for account in file_rdr.iter() {
            tot_rec += 1;
            let account_data = append_data(
                account,
                &keys,
                &config_params,
                diag_log,
                to_bucket_days,
                from_bucket_days,
                &file.int_basis,
                &cust_master,
                &mut deal_id_map,
            );
            if account_data.is_ok() {
                writer.write(account_data.expect("Unexpected unwrap error occured"));
            }
        }
        let health_report =
            HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
        log_info!(log, "{}", health_report.display());
        health_report.gen_health_rpt(&files_config.output_file_path);
    }
}

fn get_days(info: &str, as_on_date: &NaiveDate) -> i64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}
fn num_days(info: &str, as_on_date: &NaiveDate) -> i64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = increment_date_by_months(*as_on_date, period as u16);
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = increment_date_by_months(*as_on_date, (period * 12) as u16);
        return rbdate::num_days_start_to_end(*as_on_date, new_date);
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
