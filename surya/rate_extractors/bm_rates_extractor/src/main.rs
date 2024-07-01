#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod configuration_parameters;
mod extract;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

fn main() {
    let start_main_timer = SystemTime::now();
    let app_name = "benchmark-rates-extractor";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    extract::extract_rates(&config_params, &log, &diagnostics_log);
    let end_main_timer = SystemTime::now();
    let total_duration = end_main_timer
        .duration_since(start_main_timer)
        .expect("Could not calculate total duration for Benchmark Rate Extractor.");
    info!(
        log,
        "Total duration for Benchmark Rate Extractor: `{:?}`.", total_duration
    );
    println!(
        "Total Duration for Benchmark Rate Extractor: {:2?}",
        total_duration
    );
}
