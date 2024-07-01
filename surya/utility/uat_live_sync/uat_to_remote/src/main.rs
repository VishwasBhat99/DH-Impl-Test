#![feature(seek_convenience)]
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
extern crate colored;
extern crate core;
extern crate crossterm_cursor;
extern crate fs_extra;
extern crate indicatif;
extern crate rbdate;
extern crate rprompt;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
extern crate ssh2;
extern crate term_size;
extern crate terminal_menu;
extern crate termion;
extern crate walkdir;
#[macro_use]
mod macros;
mod configuration_parameters;
mod init;
mod log;
mod process;
use init::init_loggers;
use process::process_name;
use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();

    // initialize loggers
    let app_name = "uat-to-remote";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    // process
    process_name(config_params, &log, &diagnostics_log);

    let _total_duration = print_return_time_since!(start_time_main);

    log_info!(log, "Total time taken for execution: {:?}", start_time_main);
}
