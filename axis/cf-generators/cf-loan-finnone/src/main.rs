#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy::unwrap_used, clippy::panicking_unwrap, clippy::unwrap_used)
)]
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate clap;
extern crate health_report;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
mod macros;
mod cashflow_generator;
mod configuration_parameters;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

use std::time::SystemTime;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    // initialize loggers
    let app_name = "cf-loans-finnone-0.1.3571";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());

    cashflow_generator::generate(
        config_param.input_file_path(),
        config_param.output_file_path_principal(),
        config_param.as_on_date(),
        config_param.calc_ir_from_ason().to_string(),
        &log,
        &diagnostics_log,
    );
    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
