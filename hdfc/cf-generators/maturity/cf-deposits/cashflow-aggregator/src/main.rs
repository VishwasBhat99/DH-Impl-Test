extern crate clap;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate sdb_io;
extern crate sdb_agg_rules;

mod statics;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod aggregator;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

fn main() {
    let app_name = "depostis-aggregator";

    let p = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(p.log_file_path(), p.diagnostics_file_path());
    p.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(p.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(p.is_perf_diagnostics_enabled());
    let report = aggregator::aggregate_cashflows(
        p.input_file_path(),
        p.as_on_date(),
        p.output_file_path(),
        p.base_currency(),
        p.currency_conversion_file_path(),
        p.known_fields_file_path(),
        p.account_metadata_file_path(),
        p.rules_file_path(),
        &log,
        &diagnostics_log
    );

    report.serialise_to_path(p.output_file_path());
}