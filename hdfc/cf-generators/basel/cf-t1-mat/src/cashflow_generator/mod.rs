mod account_as_cashflows;
mod account_field_names;
mod account_writer;
mod cashflow_appender;

use self::cashflow_appender::append_data;
use cashflow_generator::account_field_names::AccFieldNames;
use cashflow_generator::account_writer::AccountWriter;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::increment_date_by_months;
use sdb_dyn_proto_rdr::reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct RangeSlab {
    id: String,
    from: f64,
    to: f64,
}

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    // init account reader
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    // read cust master file
    let cust_master_file = match new_buf_rdr(config_params.cust_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.cust_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
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
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(',').collect();
        cust_master.insert(fields[0].trim().to_string(), fields[1].trim().to_string());
    }
    // init req fields
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    // init writer
    let mut writer = AccountWriter::new(config_params.output_file_path(), log);
    let prd_slabs: Vec<RangeSlab> =
        get_prd_slabs(config_params.slabs_file_path(), config_params.as_on_date());
    let mut tot_rec = 0;
    let skp_rec = 0;
    let tot_amt = 0.0;
    for account in account_reader.iter() {
        let acc_id = account
            .get_string_for_key(&keys.acc_id)
            .expect("Error while reading account id.")
            .to_string();
        tot_rec += 1;
        let account_data = append_data(
            account,
            &keys,
            &prd_slabs,
            &cust_master,
            &config_params,
            diag_log,
        );
        if !account_data.is_ok() {
            info!(log, "Could not read cahflows for acc {}", acc_id);
            continue;
        }
        writer.write(account_data.expect("Could not find account data"));
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn get_prd_slabs(path: &str, as_on_date: &NaiveDate) -> Vec<RangeSlab> {
    let mut slabs: Vec<RangeSlab> = Vec::new();
    let input_file = match File::open(path) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(slab_info) => {
                let info: Vec<&str> = slab_info.split('|').collect();
                //TODO: Change this approach of handling cases like "-1D"
                let from_days = if info[1].contains("-") {
                    -1.0 * get_days(info[1].replace("-", "").as_str(), as_on_date)
                } else {
                    get_days(info[1], as_on_date)
                };
                let to_days = if info[2].contains("-") {
                    -1.0 * get_days(info[2].replace("-", "").as_str(), as_on_date)
                } else {
                    get_days(info[2], as_on_date)
                };
                let new_slab = RangeSlab {
                    id: info[0].to_string(),
                    from: from_days,
                    to: to_days,
                };
                slabs.push(new_slab)
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    slabs
}

fn get_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    let mut alpha_code: Vec<&str> = info.split(|c: char| c.is_numeric()).collect();
    alpha_code.retain(|&x| x != "");
    let mut num_code: Vec<&str> = info.split(|c: char| c.is_alphabetic()).collect();
    num_code.retain(|&x| x != "");
    let mut days = 0.0;
    for (i, num_val) in num_code.iter().enumerate() {
        let period = num_val.to_string() + alpha_code[i];
        days += num_days(&period, as_on_date);
    }
    days
}
fn num_days(info: &str, as_on_date: &NaiveDate) -> f64 {
    if info.contains("D") {
        let period: i64 = info
            .trim_matches('D')
            .parse::<i64>()
            .expect("Invalid from day format");
        return period as f64;
    } else if info.contains("M") {
        let period: usize = info
            .trim_matches('M')
            .parse::<usize>()
            .expect("Invalid from month format");
        let new_date = increment_date_by_months(*as_on_date, period as u16);
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = increment_date_by_months(*as_on_date, (period * 12) as u16);
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
