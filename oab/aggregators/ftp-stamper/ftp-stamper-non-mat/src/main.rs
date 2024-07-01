extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate health_report;
extern crate protobuf;
extern crate rbconcurrency;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_agg_rules_adj;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate sdb_util;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

mod statics;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod stamp_ftp;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let app_name = "ftp_stmp_non_mat";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diag_log) = log::setup_loggers(
        &config_params.log_file_path(),
        &config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    stamp_ftp::process_records(config_params, &log, &diag_log);

    let duration = start.elapsed();
    println!("Total time to process: {:?}", duration);
    let log_str = format!("Total time to process: {:?}", duration);
    log_info!(&log, "{}", log_str);
}
