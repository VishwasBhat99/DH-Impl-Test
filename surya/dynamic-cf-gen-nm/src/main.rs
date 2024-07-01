#[macro_use]
extern crate slog;
extern crate clap;
extern crate health_report;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_cf_gen;
extern crate sdb_day_convention;
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
use process::generate;
use std::time::SystemTime;

fn main() {
    let start_aggregation_timer = SystemTime::now();

    // initialize loggers
    let app_name = "dyn-cf-nm";
    let (config_params, log, diagnostics_log) = init_loggers(app_name);

    // process
    generate(&config_params, &log, &diagnostics_log);

    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time taken for execution: {:?}", total_duration);
}
