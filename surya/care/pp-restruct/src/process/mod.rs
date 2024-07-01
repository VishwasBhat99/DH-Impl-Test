use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod process;
mod reader;
use super::*;
use calamine::*;
use process::process::process_la;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    begin_process(config_params, logger);
}

pub fn begin_process(config_params: &ConfigurationParameters, logger: &Logger) {
    match reader::reader(config_params.input_file_path(), logger) {
        None => {
            println!("Error: input file not found");
        }
        Some(val) => {
            process_la(val, config_params, logger);
        }
    };
}
