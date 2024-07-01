extern crate chrono;
extern crate clap;
extern crate protobuf;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

mod configuration_parameters;
mod log;
#[macro_use]
mod macros;
mod stamper;
mod statics;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let app_name = "stmp_investments";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, _diag_log) = log::setup_loggers(
        &config_params.log_file_path(),
        &config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    stamper::stamp_llg(config_params, &log);

    let duration = start.elapsed();
    println!("Total time to process: {:?}", duration);
    let log_str = format!("Total time to process: {:?}", duration);
    log_info!(&log, "{}", log_str);
}
