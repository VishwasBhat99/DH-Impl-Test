extern crate serde;
extern crate serde_json;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::num_days_start_to_end;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::io::prelude::*;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut writer = match buf_file_wrtr(&config_params.output_file_path(), None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}` : {:?}.",
                &config_params.output_file_path(),
                env::current_exe()
                    .unwrap_or_else(|error| {
                        panic!("Unable to find current directory path: {}", error);
                    })
                    .display(),
                error
            );
        }
    };
    let mut cust_id_lien_amt_map: HashMap<String, CustData> = HashMap::new();
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };
    for (line_num, lines) in input_file.lines().enumerate() {
        tot_rec += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 10 {
            log_error!(log, "Column are less than 10 for line {}", line);
            skp_rec += 1;
            continue;
        }
        let cust_id = fields[config_params.cust_id_column() - 1].to_string();
        let fd_lien_amt = fields[3].parse().unwrap_or(0.0);
        let fd_level_out_amt = fields[4].parse().unwrap_or(0.0);
        let exp_dt =
            NaiveDate::parse_from_str(fields[5], "%d-%m-%Y").unwrap_or(*config_params.as_on_date());

        let mut cust_data = cust_id_lien_amt_map
            .entry(cust_id.clone())
            .or_insert_with(|| CustData::new(0, 0.0, 0.0));

        cust_data.count += 1;
        if num_days_start_to_end(*config_params.as_on_date(),exp_dt) > config_params.residual_mat_days() {
            cust_data.fd_lien_amount += fd_lien_amt;
            cust_data.fd_level_out_amt += fd_level_out_amt;
        }
    }

    for (cust_id, cust_data) in cust_id_lien_amt_map {
        let amt = if cust_data.fd_lien_amount < cust_data.fd_level_out_amt {
            cust_data.fd_lien_amount
        } else {
            cust_data.fd_level_out_amt
        };
        let op_str = format!("{}|{}", cust_id, amt);
        writeln!(writer, "{}", op_str).unwrap_or_else(|error| {
            panic!("Unable to write to the output file: {}", error);
        });
    }
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, 0.0, 0.0, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

#[derive(Debug, Clone)]
pub struct CustData {
    pub count: i64,
    pub fd_lien_amount: f64,
    pub fd_level_out_amt: f64,
}

impl CustData {
    pub fn new(count: i64, fd_lien_amount: f64, fd_level_out_amt: f64) -> Self {
        CustData {
            count,
            fd_lien_amount,
            fd_level_out_amt,
        }
    }
}
