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
extern crate dbpool;
extern crate oracle;
extern crate r2d2;
extern crate serde;
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
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
struct DBConfig {
    db_username: String,
    db_password: String,
    db_servicename: String,
}

fn main() {
    let start_aggregation_timer = SystemTime::now();

    // initialize loggers
    let app_name = "rules-extractor";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    let config_file = config_params.db_config_file_path();

    let file = File::open(&config_file).expect("Cannot open config file");
    let reader = BufReader::new(file);
    let dbconfig: DBConfig = serde_json::from_reader(reader).expect("Cannot read config file");

    let manager = dbpool::OracleConnectionManager::new(
        dbconfig.db_username,
        dbconfig.db_password,
        dbconfig.db_servicename,
    );

    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool of connections");

    // process
    process(&config_params, &log, &diagnostics_log, pool);

    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
