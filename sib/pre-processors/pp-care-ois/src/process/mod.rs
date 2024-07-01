use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
mod derive_fields;
use self::derive_fields::get_op_line;
use self::structs::*;
use macros;
mod structs;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut amt = 0.0;
    let input_file_path = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let master_file_path = match new_buf_rdr(config_params.master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found master file: `{}` on location `{}` : {}.",
            config_params.master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut master_hm: HashMap<PeriodKey, TimeBandData> = HashMap::new();
    for (line_num, lines) in master_file_path.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let period_key = PeriodKey::get_period_key(fields.to_owned());
        let timeband_data = TimeBandData::get_timeband_fields(fields);
        master_hm.insert(period_key, timeband_data);
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

    for (line_num, lines) in input_file_path.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        acc_enc += 1;
        let fields: Vec<&str> = line.split('|').collect();
        amt += fields[8].parse::<f64>().unwrap_or(0.0);
        let cashflow_date =
            NaiveDate::parse_from_str(fields[4], "%d-%m-%Y").unwrap_or(*config_params.as_on_date());
        let value_date =
            NaiveDate::parse_from_str(fields[2], "%d-%m-%Y").unwrap_or(*config_params.as_on_date());
        let res_tenor = rbdate::num_days_start_to_end(value_date, cashflow_date);
        let timeband_val = get_timeband(res_tenor, &master_hm);
        let op_line = get_op_line(fields, res_tenor, cashflow_date, value_date, timeband_val);
        match writer.write_all(op_line.as_bytes()) {
            Ok(_) => log_info!(logger, "Successfully processed all accounts."),
            Err(error) => panic!(
                "Unable to write processed lines on file `{}`: {}.",
                config_params.output_file_path(),
                error,
            ),
        }
        acc_succ += 1;
    }
    let health_stat = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, amt, amt, 0);
    health_stat.gen_health_rpt(config_params.output_file_path())
}

pub fn get_timeband(res_tenor: i64, master_hm: &HashMap<PeriodKey, TimeBandData>) -> TimeBandData {
    let mut timeband_data = TimeBandData::new();
    for (key, val) in master_hm.iter() {
        if res_tenor >= key.start_days && res_tenor < key.end_days {
            timeband_data = val.to_owned();
        }
    }
    timeband_data
}
