extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate health_report;
extern crate protobuf;
extern crate rbconcurrency;
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
    let start_timer = SystemTime::now();
    let app_name = "td-cf";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());

    cashflow_generator::generate(&config_param, &log, &diagnostics_log);
    let end_timer = SystemTime::now();
    let total_duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total duration");
    info!(log, "Total Duration for CF-Generator: {:?}", total_duration);
    println!("Total Duration for CF-Generator: {:?}", total_duration);
}
