extern crate calamine;
extern crate clap;
#[macro_use]
extern crate slog;
extern crate rbdate;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate health_report;
extern crate sdb_io;

#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod process;

fn main() {
    let app_name = "topn_sig_borr";
    let config_params = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);
    process::process(config_params, &log, &diagnostics_log);
}
