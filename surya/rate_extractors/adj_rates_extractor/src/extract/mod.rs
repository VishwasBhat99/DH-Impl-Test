use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::{env::current_dir, io::prelude::*};

pub fn extract_rates(config_params: &ConfigurationParameters, log: &Logger) {
    let mut out_str: String = String::new();

    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();

        if fields.len() != 4 {
            log_error!(log, "Line `{}` is not well-formatted.", line);
            continue;
        }

        let ason = NaiveDate::parse_from_str(fields[1], "%d-%m-%Y")
            .expect("error while parsing ason date");

        let future_dt = NaiveDate::parse_from_str(fields[2], "%d-%m-%Y")
            .expect("error while parsing future date ");

        let days_diff =
            rbdate::num_days_start_to_end(ason, future_dt) + 1 + config_params.derive_till_n_days();
        let mut date = ason;
        if days_diff > 1 {
            out_str.push_str(&date.format("%d-%m-%Y").to_string());
            out_str.push('|');
            out_str.push_str(fields[0]);
            out_str.push('|');
            out_str.push_str(fields[3]);
            out_str.push('\n');
            for _ in 1..days_diff {
                date = date.succ_opt().expect("Error in getting succeeding date");
                out_str.push_str(&date.format("%d-%m-%Y").to_string());
                out_str.push('|');
                out_str.push_str(fields[0]);
                out_str.push('|');
                out_str.push_str(fields[3]);
                out_str.push('\n');
            }
        } else {
            out_str.push_str(fields[1]);
            out_str.push('|');
            out_str.push_str(fields[0]);
            out_str.push('|');
            out_str.push_str(fields[3]);
            out_str.push('\n');
        }
    }

    let mut out_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file `{}` on location `{}` : {}",
            &config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(out_str.as_bytes()) {
        Ok(_) => "",
        Err(error) => panic!(
            "Unable to write processed lines to output file `{}`: {}.",
            config_params.output_file_path(),
            error
        ),
    };
}
