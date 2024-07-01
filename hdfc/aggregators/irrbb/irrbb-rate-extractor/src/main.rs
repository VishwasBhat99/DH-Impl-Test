extern crate chrono;
extern crate clap;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog;
mod calculation;
mod configuration_parameters;
mod file_writer;
mod log;
mod macros;
mod read_create;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "rate-extractor";
    let config_parameter = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_parameter.log_file_path(),
        config_parameter.diagnostics_file_path(),
    );
    config_parameter.log_parameters(&log);
    LOG_PARAMS.set_once_diagnostic_level(config_parameter.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_parameter.is_perf_diagnostics_enabled());

    calculation::operation(config_parameter, &log, &diagnostics_log);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main timer.");
    info!(
        log,
        "Total Duration taken by Rate EXtractor: {:?}", total_duration
    );
    println!(
        "Total Duration taken by Rate Extractor: {:?}",
        total_duration
    );
}
