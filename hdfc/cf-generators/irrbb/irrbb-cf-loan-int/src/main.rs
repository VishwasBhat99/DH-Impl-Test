extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate protobuf;
extern crate rbconcurrency;
extern crate rbdate;
extern crate sdb_day_convention;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;

mod statics;
#[macro_use]
mod macros;
mod batch;
mod configuration_parameters;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

fn main() {
    let app_name = "cf_loan_irrbb_int";

    let p = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(p.log_file_path(), p.diagnostics_file_path());
    p.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(p.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(p.is_perf_diagnostics_enabled());

    let batch_processing_report = batch::process(
        p.batch_size(),
        p.num_threads(),
        p.input_file_path(),
        p.output_file_path(),
        p.day_convention(),
        &log,
        &diagnostics_log,
        p.as_on_date(),
        p.is_contractual(),
    );

    batch_processing_report.print_report(&log);
    batch_processing_report.serialise_to_file(p.output_file_path());
}
