extern crate clap;
extern crate csv;
#[macro_use]
extern crate slog;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
extern crate sdb_agg_rules;
extern crate sdb_io;

mod statics;
#[macro_use]
mod macros;
mod aggregator;
mod configuration_parameters;
mod currency;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

fn main() {
    let app_name = "irrbb";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());
    aggregator::aggregate_cashflows(config_params, &log, &diagnostics_log);
}
