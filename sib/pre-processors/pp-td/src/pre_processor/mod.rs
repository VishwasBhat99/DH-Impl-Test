extern crate csv;
extern crate serde;
use crate::pre_processor::input_account::EitData;

use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::{InputAccount, IntRateData};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::time::SystemTime;
mod derive_fields;
mod input_account;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut acc_enc: i64 = 0;
    let mut acc_succ = 0;
    let mut tot_amt = 0.0;
    let mut writer = BufWriter::new(output_file);
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut int_rate_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.int_rate_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.int_rate_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut int_rate_map: HashMap<String, IntRateData> = HashMap::new();
    for (line_num, lines) in int_rate_reader.deserialize().enumerate() {
        let int_rate_data: IntRateData = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        int_rate_map.insert(int_rate_data.acid.to_owned(), int_rate_data);
    }

    let mut eit_map: HashMap<String, EitData> = HashMap::new();
    if Path::new(config_param.eit_file_path()).exists() {
        let eit_reader = match new_buf_rdr(config_param.eit_file_path()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found npa live file: `{}` on location `{}` : {}.",
                config_param.eit_file_path(),
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        for (line_no, lines) in eit_reader.lines().enumerate() {
            let eit_line = match lines {
                Ok(eit_line) => eit_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.eit_file_path(),
                    line_no + 1,
                    error
                ),
            };

            let eit_fields = eit_line.split('|').collect::<Vec<&str>>();
            eit_map.insert(
                eit_fields[0].to_string(),
                EitData {
                    nrml_accrued_amount_cr: eit_fields[14].parse::<f64>().unwrap_or(0.0),
                    nrml_interest_amount_cr: eit_fields[18].parse::<f64>().unwrap_or(0.0),
                },
            );
        }
    }

    for (line_num, lines) in reader.deserialize().enumerate() {
        let mut input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };

        acc_enc += 1;
        let temp_string = get_op_line(
            &mut input_account,
            &int_rate_map,
            &eit_map,
            config_param.as_on_date(),
            &mut tot_amt,
        );
        op_line.push_str(temp_string.as_str());
        op_line.push('\n');
        acc_succ += 1;
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
    let health_report =
        HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(&config_param.output_file_path);
}
