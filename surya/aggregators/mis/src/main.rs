#[macro_use]
extern crate slog;
extern crate clap;
extern crate rbdate;
extern crate sdb_agg_rules;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate serde_derive;
extern crate health_report;
extern crate sdb_dyn_proto_rdr;

#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
use aggregate::aggregate;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use std::time::SystemTime;

mod aggregate;

fn main() {
    let start_process_timer = SystemTime::now();

    // Init configuration parameters and loggers
    let app_name = "mis-1.2.4534";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);
    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    aggregate(config_params, &log, &diagnostics_log);

    let ttl_proc_time = print_return_time_since!(start_process_timer);
    log_info!(log, "Total Time Taken: {:?}", ttl_proc_time);
}
