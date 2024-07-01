#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy::panicking_unwrap, clippy::unwrap_used)
)]
extern crate chrono;
extern crate clap;
extern crate health_report;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod process;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;
fn main() {
    let start_main_timer = SystemTime::now();
    let app_name = "pp-lcbg";

    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    process::process(config_params, &log, &diagnostics_log);
    let end_main_timer = SystemTime::now();
    let total_duration = end_main_timer
        .duration_since(start_main_timer)
        .expect("Could not calculate total duration for the LCBG Pre-Processor.");
    info!(
        log,
        "Total Duration for Processing LCBG-PP Data: {:?}", total_duration
    );
    println!(
        "Total Duration for Processing LCBG-PP Data: {:?}",
        total_duration
    );
}