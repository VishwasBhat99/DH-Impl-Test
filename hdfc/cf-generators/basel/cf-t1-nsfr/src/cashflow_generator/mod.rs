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
use rbdate::incr_dt_by_mon_presrv_eom;
use sdb_dyn_proto_rdr::reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::DEFAULT_FLOAT;
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

pub struct RWData {
    rw_prcnt: f64,
}

pub struct RFFlag {
    rf_flag: String,
}

pub struct RMFlag {
    rm_flag: String,
}
pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    // init account reader
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );

    let method_reader = reader::Reader::new_at_path(
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
                config_params.cust_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(',').collect();
        cust_master.insert(fields[0].trim().to_string(), fields[1].trim().to_string());
    }
    // read risk weight master file
    let rw_master_file = match new_buf_rdr(config_params.rw_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.rw_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rw_master: HashMap<String, RWData> = HashMap::new();
    for (line_num, lines) in rw_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.rw_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let data = RWData {
            rw_prcnt: fields[1].trim().parse().unwrap_or(DEFAULT_FLOAT),
        };
        rw_master.insert(fields[0].trim().to_string(), data);
    }
    // read restructured flag master file
    let rf_master_file = match new_buf_rdr(config_params.restructured_flag_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.restructured_flag_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rf_master: HashMap<String, RFFlag> = HashMap::new();
    for (line_num, lines) in rf_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.restructured_flag_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let data = RFFlag {
            rf_flag: fields[1].trim().parse().unwrap_or("".to_string()),
        };
        rf_master.insert(fields[0].trim().to_string(), data);
    }
    // read residential mortgage flag master file
    let rm_master_file = match new_buf_rdr(config_params.residential_mortgage_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.residential_mortgage_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rm_master: HashMap<String, RMFlag> = HashMap::new();
    for (line_num, lines) in rm_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.residential_mortgage_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let data = RMFlag {
            rm_flag: fields[1].trim().parse().unwrap_or("".to_string()),
        };
        rm_master.insert(fields[0].trim().to_string(), data);
    }
    // read prod code file
    let mut prod_map: HashMap<String, String> = HashMap::new();
    if !config_params.prod_code_file().is_empty() {
        let prod_code_file = match new_buf_rdr(config_params.prod_code_file()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` due to: {}.",
                config_params.prod_code_file(),
                error
            ),
        };
        for (line_num, lines) in prod_code_file.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.prod_code_file(),
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();
            prod_map.insert(fields[0].trim().to_string(), "".to_string());
        }
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
        tot_rec += 1;
        let account_data = append_data(
            account,
            &method_reader,
            &keys,
            &prd_slabs,
            &cust_master,
            &rw_master,
            &rf_master,
            &rm_master,
            &config_params,
            diag_log,
            &prod_map,
        );
        writer.write(account_data);
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
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else if info.contains("Y") {
        let period: usize = info
            .trim_matches('Y')
            .parse::<usize>()
            .expect("Invalid from year format");
        let new_date = incr_dt_by_mon_presrv_eom(*as_on_date, period * 12)
            .expect("Cannot add month to as on date as per prd slab config");
        return rbdate::num_days_start_to_end(*as_on_date, new_date) as f64;
    } else {
        panic!("Invalid period type in prd config file.");
    }
}
