extern crate clap;
#[macro_use]
extern crate slog;
extern crate rbdate;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate csv;
extern crate hashbrown;
extern crate health_report;
extern crate sdb_agg_rules;
extern crate sdb_io;
extern crate serde;

#[macro_use]
mod macros;
mod aggregator;
mod configuration_parameters;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

fn main() {
    let app_name = "Dimension-Wise-FTP-Stamper-Aggregator";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());
    aggregator::generatesummary(config_params, &log, &diagnostics_log);
}
