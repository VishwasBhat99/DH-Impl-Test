extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate float_cmp;
extern crate health_report;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod splitter;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_generate_timer = SystemTime::now();
    let app_name = "cf-generic-splitter-1.0.2684";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());
    splitter::split_files(&config_params, &log, &diagnostics_log);
    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for execution.");
    info!(log, "Total duration for Splitting `{:?}`.", total_duration);
    println!("Total Duration for Splitting: {:2?}", total_duration);
}
