extern crate calamine;
extern crate chrono;
extern crate clap;
extern crate csv;
extern crate health_report;
extern crate rbdate;
extern crate sdb_agg_rules_txt;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_derive;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod pre_processor;
mod statics;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "pp-fwd-contracts-1.0.3244";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());

    pre_processor::process(config_param, &log, &diagnostics_log);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main timer.");
    info!(
        log,
        "Total Duration taken by Forward pre-processor: {:?}", total_duration
    );
    println!(
        "Total Duration taken by Forward pre-processor: {:?}",
        total_duration
    );
}
