use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use serde_json::json;
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod datatype_lookup;
mod json_generator;

pub fn generate_metadata(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut output_file_path = String::from(config_param.input_file_path());
    output_file_path = output_file_path.replace(".proto", ".json");
    let mut writer = match buf_file_wrtr(&output_file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            &output_file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut is_cashflow = false;
    let mut json_records: Vec<serde_json::Value> = Default::default();
    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };

        if line.contains("message") && line.to_lowercase().contains(" cashflow ") {
            is_cashflow = true;
        }

        // To skip message Cashflow and its contents
        if is_cashflow && !line.contains("}") {
            continue;
        } else if is_cashflow && line.contains("}") {
            is_cashflow = false;
            continue;
        }

        // To write the data into JSON
        if line.contains(";") && !line.contains("\"") {
            let json_record = json_generator::generate_json_record(line);
            json_records.push(json_record);
        }
    }

    let metadata = json!({ "fields": json_records });

    let output_line = match serde_json::to_string_pretty(&metadata) {
        Ok(val) => val,
        Err(err) => {
            log_error!(log, "Metadata formatting error.\n{}", err);

            err.to_string()
        }
    };

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();

    match writer.write_all(output_line.as_bytes()) {
        Ok(_val) => println!("Successfully processed all fields"),
        Err(error) => {
            panic!("Cannot process the input file: {:?}", error);
        }
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
}
