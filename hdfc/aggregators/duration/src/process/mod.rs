use configuration_parameters::ConfigurationParameters;
use slog::Logger;
pub mod currency;
pub mod process_data;
pub mod read_data;
pub mod writer;
pub extern crate chrono;
use std::fs::File;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let acc_op = format!(
        "{}{}",
        config_params.output_file_path(),
        "-account-level.txt"
    );
    let acc_summary_op = format!("{}{}", config_params.output_file_path(), "-acc-summary.txt");
    let output_acount_level =
        File::create(acc_op).expect("unable to create account level output file");
    let output_summary_level =
        File::create(acc_summary_op).expect("unable to create final summary output file");
    read_data::process_data(
        config_params,
        &output_acount_level,
        &output_summary_level,
        logger,
    );
}
