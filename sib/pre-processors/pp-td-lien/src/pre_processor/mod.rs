use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod derive_fields;
use self::derive_fields::derive_output;

use self::deposits_lien_struct::TdAccountStructValues;
mod deposits_lien_struct;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let td_file_paths = match File::open(config_params.td_file_path()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.td_file_path(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.td_file_path(),
                error
            );
        }
    };
    let td_file_path_reader = BufReader::new(td_file_paths);

    let lien_file_paths = match File::open(config_params.lien_file_path()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.lien_file_path(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.lien_file_path(),
                error
            );
        }
    };
    let lien_file_path_reader = BufReader::new(lien_file_paths);
    let mut finacle_data_map: HashMap<String, TdAccountStructValues> = HashMap::new();

    for line in td_file_path_reader.lines() {
        let line = line.expect("Not reading lines from finacle file");
        let fields: Vec<&str> = line.split('|').collect();

        if fields[47] == "Current Deposits"
            || fields[47] == "Saving Bank Deposits"
            || fields[47] == "Term Deposits"
        {
            let account_values = TdAccountStructValues {
                acid: fields[47].trim().to_string(),
                currency: fields[17].trim().to_string(),
                clr_bal_amt: fields[3].trim().parse::<f64>().unwrap_or(0.0),
                const_code: fields[46].to_string(),
                gl_sub_head_code: fields[13].trim().to_string(),
            };
            finacle_data_map.insert(fields[0].trim().to_string(), account_values);
        } else {
            info!(logger, "Skipping storing value   of {:?}", fields[47]);
        }
    }

    derive_output(
        lien_file_path_reader,
        &finacle_data_map,
        config_params,
        logger,
    );
}
