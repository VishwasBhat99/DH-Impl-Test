#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]
extern crate clap;
extern crate health_report;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod derive;
mod log;
mod statics;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let app_name = "gl-npa-cf";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());

    derive::process(config_param, &log, &diagnostics_log);
    let end_time = SystemTime::now();
    let total_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    info!(
        log,
        "Total Duration for GL NPA CF Pre-Processor: {:?}", total_duration
    );
    println!(
        "Total Duration for GL NPA CF Pre-Processor: {:?}",
        total_duration
    );
}
