#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]
#[macro_use]
extern crate slog;
extern crate clap;
extern crate health_report;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
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
    let start_aggregation_timer = SystemTime::now();
    // init
    let app_name = "sls-aggregator";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);

    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);
    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());
    // process
    aggregator::aggregate_cashflows(&config_params, &log, &diagnostics_log);
    // cal
    let total_duration = print_return_time_since!(start_aggregation_timer);
    log_info!(log, "Total time for aggregation: {:?}", total_duration);
}
