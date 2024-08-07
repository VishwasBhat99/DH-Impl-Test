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
extern crate chrono;
extern crate health_report;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_day_convention;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

mod statics;
#[macro_use]
mod macros;
mod cashflow_generator;
mod configuration_parameters;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;
fn main() {
    let start_generate_timer = SystemTime::now();
    let app_name = "biu-stamper";

    let cnfg_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        cnfg_params.log_file_path(),
        cnfg_params.diagnostics_file_path(),
    );
    cnfg_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(cnfg_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(cnfg_params.is_perf_diagnostics_enabled());

    cashflow_generator::generate(&cnfg_params, &log, &diagnostics_log);
    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for cashflow generation.");
    info!(
        log,
        "Total duration for BIU Stamped cashflow generation: `{:?}`.", total_duration
    );
    println!(
        "Total Duration for BIU Stamped cashflow generation: {:2?}",
        total_duration
    );
}
