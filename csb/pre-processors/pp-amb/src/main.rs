#[macro_use]
extern crate slog;
extern crate chrono;
extern crate clap;
extern crate rbdate;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
mod macros;
mod configuration_parameters;
mod init;
mod log;
mod process;

use init::init_loggers;
use macros::*;
use process::process_name;
use std::time::SystemTime;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    // initializing loggers
    let app_name = "pp-amb";
    let (config_params, log, _diagnostics_log) = init_loggers(app_name);

    // process
    process_name(&config_params);

    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
