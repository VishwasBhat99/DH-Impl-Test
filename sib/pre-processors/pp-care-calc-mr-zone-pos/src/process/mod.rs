use self::structs::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
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
    let mut master_hm: HashMap<String, Data> = HashMap::new();
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
        let data = Data::new(&fields, *config_params.as_on_date());
        master_hm.insert(fields[6].to_string(), data);
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
        amt += fields[45].parse::<f64>().unwrap_or(0.0);
        master_hm
            .entry(fields[58].to_string())
            .and_modify(|data| data.append_data(fields, *config_params.as_on_date()));
    }
    for (key, val) in master_hm.iter() {
        let op_line = format!(
            "{}|{}|{}|{}|{}|{}\n",
            val.as_on_date.format("%d-%m-%Y"),
            key,
            val.zone,
            val.long_pos,
            val.short_pos,
            val.long_pos - val.short_pos
        );
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
