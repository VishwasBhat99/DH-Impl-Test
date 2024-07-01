#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy::unwrap_used, clippy::panicking_unwrap,)
)]
#[macro_use]
extern crate slog;
extern crate clap;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
extern crate rbdate;
#[macro_use]
mod macros;
mod configuration_parameters;
mod init;
mod log;
mod process;

use init::init_loggers;
use process::convert_txt;
use std::time::SystemTime;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    // initialize loggers
    let app_name = "utf8-convert-1.0.0";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    // process to convert input to valid utf8 format 
    convert_txt(&config_params, &log, &diagnostics_log);

    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
