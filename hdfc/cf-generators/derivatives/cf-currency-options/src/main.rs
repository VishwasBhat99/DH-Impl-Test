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
#[macro_use]
extern crate slog;
extern crate calamine;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

mod statics;
#[macro_use]
mod macros;
mod cashflow_derivator;
mod configuration_parameters;
mod log;

use cashflow_derivator::derive;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_generate_timer = SystemTime::now();
    let app_name = "ccy-opt";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    derive(config_params, &log, &diagnostics_log);
    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for Currency Options.");
    info!(
        log,
        "Total duration for Currency Options cashflow generation: `{:?}`.", total_duration
    );
    println!(
        "Total Duration for Currency Options cashflow generation: {:2?}",
        total_duration
    );
}
