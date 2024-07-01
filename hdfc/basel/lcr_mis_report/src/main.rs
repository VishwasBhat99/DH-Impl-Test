#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate clap;
extern crate health_report;
extern crate rbdate;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
mod macros;
mod configuration_parameters;
mod init;
mod io;
mod log;
mod processor;
mod statics;
use init::init_loggers;
use std::time::SystemTime;
fn main() {
    let start_aggregation_timer = SystemTime::now();
    // initialize loggers
    let app_name = "LCR_MIS_REPORT";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);
    processor::process(config_params);
    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
