extern crate chrono;
extern crate clap;
extern crate health_report;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
mod macros;
#[macro_use]
extern crate slog;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;
mod config_params;
mod log;
mod processing;

fn main() {
    let start_main_timer = SystemTime::now();
    let app_name = "top-n-deposits";
    // Get application parameters.
    let config_param = config_params::get_configuration_parameters(app_name);
    // Initialising log files.
    let (log, diagnostics_log) =
        log::setup_loggers(config_param.log_file(), config_param.diag_log_file());
    config_param.log_parameters(&log);
    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());
    info!(diagnostics_log, "Start of processing  data.");
    processing::process_data(&config_param, &log, &diagnostics_log);
    info!(diagnostics_log, "End of processing  data.");
    let end_main_timer = SystemTime::now();
    let total_duration = end_main_timer
        .duration_since(start_main_timer)
        .expect("Could not calculate total duration.");
    info!(
        log,
        "Total Duration taken for pre-processor: {:?}", total_duration
    );
    println!(
        "Total Duration taken for  pre-processor: {:?}",
        total_duration
    );
}
