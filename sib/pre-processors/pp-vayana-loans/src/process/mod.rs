use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use process::derive_fields::get_op_line;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
mod derive_fields;
use self::structs::*;
use macros;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut amt = 0.0;
    let vayana_loans_file_path = match new_buf_rdr(config_params.vayana_loans_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.vayana_loans_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let gam_file_path = match new_buf_rdr(config_params.gam_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.gam_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut gam_hm: HashMap<i64, GAMFields> = HashMap::new();
    for (line_num, lines) in gam_file_path.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.gam_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        gam_hm.insert(
            fields[1].parse::<i64>().unwrap_or(0), //foracid
            GAMFields::get_gam_fields(fields),
        );
    }

    let npa_file_path = match new_buf_rdr(config_params.npa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.npa_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut npa_hm: HashMap<String, NPAFields> = HashMap::new();
    for (line_num, lines) in npa_file_path.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.npa_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        npa_hm.insert(
            fields[0].to_string(), //foracid
            NPAFields::get_npa_fields(fields),
        );
    }
    let mut op_line = String::new();
    for (line_num, lines) in vayana_loans_file_path.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.vayana_loans_file_path(),
                line_num + 1,
                error
            ),
        };
        acc_enc += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let cust_id = fields[1].parse::<i64>().unwrap_or(0); //To remove leading 0s and to match with gam.foracid
        amt += fields[12].parse::<f64>().unwrap_or(0.0);
        match gam_hm.get(&cust_id) {
            Some(data) => {
                op_line.push_str(&get_op_line(data, fields, &npa_hm));
            }
            None => {
                log_debug!(logger, "Foracid not found for CustID: {}. ", fields[1]);
                op_line.push_str(&get_op_line(&GAMFields::new(), fields, &npa_hm));
            }
        };
        acc_succ += 1;
    }

    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match writer.write_all(op_line.as_bytes()) {
        Ok(_) => log_info!(logger, "Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_params.output_file_path(),
            error,
        ),
    }
    let health_stat = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, amt, amt, 0);
    health_stat.gen_health_rpt(config_params.output_file_path())
}
