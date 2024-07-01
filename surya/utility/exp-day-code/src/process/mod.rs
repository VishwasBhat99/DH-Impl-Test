use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
use std::process;

pub fn get_exp_day_code(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let as_on_day = config_params
        .as_on_date()
        .weekday()
        .to_string()
        .to_uppercase();
    if as_on_day == config_params.day().to_uppercase() {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
