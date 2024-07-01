#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod format;
mod io;
mod log;
mod process;
mod script_reader;
mod structs;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use process::*;
use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "copy-script-generator-1.0.0";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.diagnostics_flag());

    process(config_param, &log, &diagnostics_log);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main timer.");
    info!(
        log,
        "Total Duration taken by Script Generator program: {:?}", total_duration
    );
    println!(
        "Total Duration taken by Script Generator program: {:?}",
        total_duration
    );
}
