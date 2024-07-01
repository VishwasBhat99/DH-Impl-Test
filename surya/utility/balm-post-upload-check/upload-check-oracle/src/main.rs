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
extern crate clap;
extern crate odbc;
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

extern crate actix_rt;
extern crate actix_web;
extern crate chrono;
extern crate curl;
extern crate dbpool;
extern crate env_logger;
extern crate oracle;
extern crate r2d2;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

use actix_web::web;
use actix_web::web::{Json, Path};
use actix_web::{Error, HttpResponse, Result};

use std::env;
use std::io;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    let app_name = "balm_upload_check";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
	let connection_details: Vec<&str> = config_params.connection_string().split("|").collect();
	
    let manager = dbpool::OracleConnectionManager::new(
        connection_details[0].to_string(),
        connection_details[1].to_string(),
        connection_details[2].to_string(),
    );
    let pool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .unwrap();
    process(pool,&config_params, &log, &diagnostics_log);
}
