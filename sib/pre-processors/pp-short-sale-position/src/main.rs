#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(feature = "cargo-clippy", warn(clippy::panicking_unwrap,))]
#[macro_use]
extern crate slog;
extern crate clap;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
mod macros;
mod configuration_parameter;
mod init;
mod log;
mod preprocessor;

use init::init_loggers;
use preprocessor::process;
use std::time::SystemTime;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    let app_name = "SHORT SALE POSITION";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    process(&config_params, &log, &diagnostics_log);

    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
