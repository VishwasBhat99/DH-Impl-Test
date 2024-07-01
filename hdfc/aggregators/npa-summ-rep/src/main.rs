extern crate calamine;
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
extern crate sdb_io;

#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod process;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

fn main() {
    let app_name = "npa_summ_rep";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());
    process::process(config_params, &log, &diagnostics_log);
}
