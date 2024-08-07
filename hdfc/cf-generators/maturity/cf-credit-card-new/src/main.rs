extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate health_report;
extern crate protobuf;
extern crate rbconcurrency;
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

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "credit_card_cf";

    let config_params = configuration_parameters::get_configuration_parameters(app_name);

    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    cashflow_derivator::derive(config_params, &log, &diagnostics_log);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main process.");
    info!(
        log,
        "Total Duration of Credit Card Cashflow generator: {:?}", total_duration
    );
    println!(
        "Total Duration of Credit Card Cashflow generator: {:?}",
        total_duration
    );
}
