extern crate clap;
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
extern crate chrono;
extern crate sdb_agg_rules;
extern crate sdb_io;

mod statics;
#[macro_use]
mod macros;
mod aggregator;
mod configuration_parameters;
mod log;

use macros::LOG_PARAMS;
use macros::PERF_PARAMS;

fn main() {
    let app_name = "gl-aggr";
    let p = configuration_parameters::get_configuration_parameters(app_name);
    let (log, diagnostics_log) = log::setup_loggers(p.log_file_path(), p.diagnostics_file_path());
    p.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(p.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(p.is_perf_diagnostics_enabled());
    aggregator::aggregate_cashflows(
        p.input_file_path(),
        p.as_on_date(),
        p.output_file_path(),
        p.consolidated_currency(),
        p.local_consolidation_currency(),
        p.currency_conversion_file_path(),
        p.known_fields_file_path(),
        p.account_metadata_file_path(),
        p.rules_file_path(),
        p.default_llg_code(),
        &log,
        &diagnostics_log,
    );
}
