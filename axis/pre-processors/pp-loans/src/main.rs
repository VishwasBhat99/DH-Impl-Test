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
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
mod macros;
mod configuration_parameters;
mod init;
mod log;
mod process;

use init::init_loggers;
use process::process;
use std::time::SystemTime;

fn main() {
    let start_processing_timer = SystemTime::now();

    // Initialize loggers
    let app_name = "pp-loans-1.0.3112";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    // Process
    process(&config_params, &log, &diagnostics_log);

    let total_duration = print_return_time_since!(start_processing_timer);
    log_info!(log, "Total time taken for processing: {:?}", total_duration);
}
