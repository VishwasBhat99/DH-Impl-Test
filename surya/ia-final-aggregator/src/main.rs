// extern crate chrono;
extern crate clap;
extern crate rbdate;
extern crate sdb_io;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;

use std::time::SystemTime;

mod final_aggregator;

fn main() {
    let start_main_timer = SystemTime::now();
    let app_name = "ia_final_aggregator";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    final_aggregator::aggregate(&config_param, &log);

    let end_main_timer = SystemTime::now();
    let total_duration = end_main_timer
        .duration_since(start_main_timer)
        .expect("Could not calculate total duration.");
    info!(log, "Total Duration: {:?}", total_duration);
    println!("Total Duration: {:?}", total_duration);
}