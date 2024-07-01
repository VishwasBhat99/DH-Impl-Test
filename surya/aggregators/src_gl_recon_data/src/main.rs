#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy::unwrap_used, clippy::panicking_unwrap, clippy::unwrap_used)
)]
#[macro_use]
extern crate slog;
extern crate calamine;
extern crate clap;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
extern crate serde_derive;
#[macro_use]
mod macros;
mod aggregator;
mod configuration_parameters;
mod init;
mod log;

use aggregator::aggregate;
use init::init_loggers;
use std::time::SystemTime;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    // initialize loggers
    let app_name = "src_gl_recon_data";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    // Aggregate based on both LLG and GL Code
    aggregate(&config_params, &log, &diagnostics_log);

    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
