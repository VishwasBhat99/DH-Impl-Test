extern crate clap;
#[macro_use]
extern crate slog;
extern crate rbdate;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
extern crate health_report;
extern crate protobuf;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;

#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod process;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let app_name = "cf-to-cf";

    let mut config_param = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_param.log_file_path(),
        config_param.diagnostics_file_path(),
    );
    config_param.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_param.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_param.is_perf_diagnostics_enabled());

    process::process(&mut config_param, &log, &diagnostics_log);
    let end_time = SystemTime::now();
    let total_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    info!(
        log,
        "Total Duration for CF TO CF Generic: {:?}", total_duration
    );
    println!("Total Duration for CF TO CF Generic: {:?}", total_duration);
}
