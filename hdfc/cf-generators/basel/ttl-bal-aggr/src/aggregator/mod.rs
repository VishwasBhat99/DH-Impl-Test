use self::account_reader::InputAccountReader;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::Write;

mod account_reader;
mod reader;

pub fn aggregate(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let reader = InputAccountReader::new(config_params.input_file_path(), log);
    // init ret and non-ret writers
    let op = format!("{}.txt", config_params.output_file_path());
    let mut writer = match buf_file_wrtr(&op, None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                op,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut reader_iterator = reader.into_iter();
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;

    // read ca ret file
    let ca_file = match new_buf_rdr(config_params.ca_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.ca_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut ca_master: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in ca_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.ca_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let key = fields[1].to_string();
        let amt = fields[7].parse::<f64>().unwrap_or(0.0);
        ca_master
            .entry(key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }
    log_info!(log, "Reading CA data completed.");

    // read sa ret file
    let sa_file = match new_buf_rdr(config_params.sa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.sa_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut sa_master: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in sa_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.sa_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let key = fields[1].to_string();
        let amt = fields[7].parse::<f64>().unwrap_or(0.0);
        sa_master
            .entry(key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }
    log_info!(log, "Reading SA data completed.");

    // read td ret file
    let td_file = match new_buf_rdr(config_params.td_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.td_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut td_master: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in td_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let key = fields[1].to_string();
        let amt = fields[7].parse::<f64>().unwrap_or(0.0);
        td_master
            .entry(key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }

    // read rd file
    let rd_file = match new_buf_rdr(config_params.rd_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.rd_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut rd_master: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in rd_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let key = fields[1].to_string();
        let amt = fields[7].parse::<f64>().unwrap_or(0.0);
        rd_master
            .entry(key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);
    }

    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unable to parse record into input struct");
        let mut total_balance = 0.0;
        total_balance += match ca_master.get(&input_account.customer_id) {
            Some(val) => *val,
            None => 0.0,
        };
        total_balance += match sa_master.get(&input_account.customer_id) {
            Some(val) => *val,
            None => 0.0,
        };
        total_balance += match td_master.get(&input_account.customer_id) {
            Some(val) => *val,
            None => 0.0,
        };
        total_balance += match rd_master.get(&input_account.customer_id) {
            Some(val) => *val,
            None => 0.0,
        };
        tot_amt += total_balance;
        write!(
            writer,
            "{}|{:.2}\n",
            &input_account.customer_id, total_balance
        )
        .expect("Error while writing output file.");
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
