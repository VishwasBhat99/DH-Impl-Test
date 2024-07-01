extern crate clap;
#[macro_use]
extern crate slog;
extern crate calamine;
extern crate chrono;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate float_cmp;
extern crate sdb_agg_rules;
extern crate sdb_io;

#[macro_use]
mod macros;
mod aggregator;
mod configuration_parameters;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;
fn main() {
    let start_time = SystemTime::now();
    let app_name = "ia-aggregator";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, _diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    aggregator::aggregate_cashflows(config_params, &log);
    let end_time = SystemTime::now();
    println!(
        "Total time taken by IA-Aggregator : {:?}",
        end_time.duration_since(start_time).unwrap()
    );
}
