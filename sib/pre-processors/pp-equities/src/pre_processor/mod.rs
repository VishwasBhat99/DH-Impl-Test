extern crate serde;
use self::format::get_op_line;
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
use std::time::SystemTime;
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
    let gl_file = match new_buf_rdr(config_param.investments_gl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.investments_gl_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut gl_map: HashMap<(String, String), String> = HashMap::new();
    let as_on_date = config_param.as_on_date();
    for (line_num, lines) in gl_file.lines().enumerate() {
        let gl_line = match lines {
            Ok(gl_line) => gl_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.investments_gl_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = gl_line.split('|').collect();
        gl_map.insert(
            (fields[0].to_string(), fields[1].to_string()),
            fields[2].to_string(),
        );
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
        let gl_key = (fields[1].to_string(), fields[3].to_string());
        let mut gl_code = 9999.to_string();
        if let Some(val) = gl_map.get(&gl_key) {
            gl_code = val.to_string();
        }
        let op_line = get_op_line(&fields, gl_code);
        match writer.write_all(op_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
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
