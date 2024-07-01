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
extern crate rbdate;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
mod macros;
mod configuration_parameters;
mod init;
mod log;
mod td_liquidity_process;

use init::init_loggers;
use std::time::SystemTime;
use td_liquidity_process::data_read_writer;

fn main() {
    let start_main_timer = SystemTime::now();

    // initialize loggers
    let app_name = "td-liquidity";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    // process
    data_read_writer(&config_params, &log, &diagnostics_log);

    let end_main_timer = SystemTime::now();
    let total_duration = end_main_timer
        .duration_since(start_main_timer)
        .expect("Could not calculate total duration for TD_Liquidity.");
    info!(log, "Total Duration for TD_Liquidity: {:?}", total_duration);
    println!(
        "Total Duration for .txt file generator: {:2?}",
        total_duration
    );
    config_params.log_parameters(&log);
}
