use self::structs::ExchKey;
use configuration_parameters::ConfigurationParameters;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;

mod structs;

pub fn extract(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    // Reader
    let file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` : {}.",
            config_params.input_file_path(),
            error
        ),
    };
    // Writer
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` : {}.",
            config_params.output_file_path(),
            error,
        ),
    };

    let mut ex_map: HashMap<ExchKey, f64> = HashMap::new();
    let mut exist_ex_map: HashMap<ExchKey, f64> = HashMap::new();
    // Extract
    for (line_num, lines) in file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}.",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let info: Vec<&str> = line.split('|').collect();

        let ex_key = ExchKey {
            from: info[0].to_string(),
            to: info[1].to_string(),
        };
        let ex_rate: f64 = info[2].parse().expect("Invalid Rate in file.");
        if ex_key.to == ex_key.from {
            writer
                .write(line.as_bytes())
                .expect("Error writing to output file.");
            writer.write(b"\n").expect("Error writing to output file.");
            continue;
        } else if ex_key.to == config_params.base_currency() {
            ex_map.insert(ex_key, ex_rate);
        } else if ex_key.from == config_params.base_currency() {
            exist_ex_map.insert(ex_key, ex_rate);
            continue;
        }
        writer
            .write(line.as_bytes())
            .expect("Error writing to output file.");
        writer.write(b"\n").expect("Error writing to output file.");
    }
    // Convert base currency equivalent rate
    for (ex_key, value) in ex_map {
        let new_exch_key = ExchKey {
            from: ex_key.to,
            to: ex_key.from,
        };
        let op = format!(
            "{}|{}|{:.3$}\n",
            new_exch_key.from,
            new_exch_key.to,
            1.0 / value,
            config_params.precision()
        );
        writer
            .write(op.as_bytes())
            .expect("Error writing to output file.");
        // Remove converted rates to avoid duplicates
        exist_ex_map.remove_entry(&new_exch_key);
    }

    // Process records where base ccy was from ccy in input
    for (ex_key, value) in exist_ex_map {
        let op = format!("{}|{}|{}", ex_key.from, ex_key.to, value,);
        writer
            .write(op.as_bytes())
            .expect("Error writing to output file.");
        writer.write(b"\n").expect("Error writing to output file.");

        let rev_op = format!(
            "{}|{}|{:.3$}",
            ex_key.to,
            ex_key.from,
            1.0 / value,
            config_params.precision()
        );
        writer
            .write(rev_op.as_bytes())
            .expect("Error writing to output file.");
        writer.write(b"\n").expect("Error writing to output file.");
    }

    writer
        .flush()
        .expect("Error writing flush buf to output file.");
}
