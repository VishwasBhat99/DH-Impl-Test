#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]

extern crate clap;
#[macro_use]
extern crate slog;
extern crate chrono;
extern crate protobuf;
extern crate rbconcurrency;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_json;
extern crate slog_async;
extern crate slog_term;
extern crate health_report;

mod statics;
#[macro_use]
mod macros;
mod cp;
mod log;

use aggregate::cfinput::AccFieldNames;
use ftp_parameters::FtpParameters;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use sdb_dyn_proto_rdr::reader;
pub mod aggregate;
mod ftp_parameters;
use aggregate::process_records;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    //Initialization
    let mut ftp_parameters = initialize();

    //Process input records
    let log_output = process_records(&mut ftp_parameters);

    let duration = start.elapsed();

    println!("Total time to process RD accounts: {:?}", duration);
    let log_str = format!("Total time to process RD accounts: {:?}", duration);
    log_info!(&ftp_parameters.log, "{}", log_str);
}

//Initialize all command line parameters , Read input cashflow file.
fn initialize() -> FtpParameters {
    let app_name = "ftp-rd-bal-aggr";

    //Initializing all configuration parameters
    let cp = cp::get_cp(app_name);

    let (log, diag_log) = log::setup_loggers(&cp.log_file_path(), &cp.diagnostics_file_path());
    cp.log_parameters(&log);

    let input_field_names = AccFieldNames::get_input_fields_names();

    let input_data = reader::Reader::new_at_path(&cp.meta_data_file_path(), &cp.input_file_path());

    LOG_PARAMS.set_once_diagnostic_level(cp.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(cp.is_perf_diagnostics_enabled());

    return FtpParameters {
        cp,
        log,
        diag_log,
        input_data,
        input_field_names,
    };
}
