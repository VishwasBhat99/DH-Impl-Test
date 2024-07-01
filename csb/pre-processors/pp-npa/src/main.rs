#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]

extern crate calamine;
extern crate chrono;
extern crate clap;
extern crate csv;
extern crate health_report;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod pre_processor;
mod statics;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "npa-pp";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.diagnostics_flag());

    pre_processor::process(config_param, &log, &diagnostics_log);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main timer.");
    info!(
        log,
        "Total Duration taken by NPA pre-processor: {:?}", total_duration
    );
    println!(
        "Total Duration taken by NPA pre-processor: {:?}",
        total_duration
    );
}
