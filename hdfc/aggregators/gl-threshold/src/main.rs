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
mod statics;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod process;

extern crate calamine;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let app_name = "gl_threshold-1.0.1";
    let start_generate_timer = SystemTime::now();
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());
    process::process(config_params, &log, &diagnostics_log);
    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration .");
    info!(
        log,
        "Total duration for Summery generation: `{:?}`.", total_duration
    );
    println!(
        "Total duration for Summery generation: {:2?}",
        total_duration
    );
}