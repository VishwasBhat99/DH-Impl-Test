#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy::unwrap_used, clippy::panicking_unwrap,)
)]
extern crate calamine;
extern crate chrono;
extern crate clap;
extern crate health_report;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
extern crate sdb_day_convention;

#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod stamper;
mod statics;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;
fn main() {
    let start_main_timer = SystemTime::now();
    let app_name = "pre-mature-ftp-stamper";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());

    stamper::process(config_param, &log, &diagnostics_log);
    let end_main_timer = SystemTime::now();
    let total_duration = end_main_timer
        .duration_since(start_main_timer)
        .expect("Could not calculate total duration for the Pre Mature FTP Stamper");
    info!(
        log,
        "Total Pre Mature FTP Stamper Duration: {:?}", total_duration
    );
    println!(
        "Total Pre Mature FTP Stamper Duration: {:?}",
        total_duration
    );
}
