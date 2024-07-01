#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate sdb_day_convention;

mod statics;
#[macro_use]
mod macros;
mod configuration_parameters;
mod log;
mod params;
mod process;
pub mod stamp_ftp;

use crate::configuration_parameters::ConfigurationParameters;
use bm_reader::*;
use ftp_method::*;
use health_report::HealthReport;
use macros::{LOG_PARAMS, PERF_PARAMS};
use params::*;
use process::calculate_ftp;
use protobuf::Clear;
use rbdate::*;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRulesAdj;
use sdb_dyn_proto_rdr::reader::{account_with_cfs::AccountWithCFs, Reader};
use sdb_io::*;
use serde_json;
use slog::Logger;
use stamp_ftp::*;
use statics::*;
pub use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    time::SystemTime,
};

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "ftp-maturity-stamper";

    // Initialization
    let config_params = configuration_parameters::get_configuration_parameters(app_name);

    // Total adjustment count (fixed, variable) must be less than 6
    if config_params.fixed_adj_count() + config_params.var_adj_count() > 6 {
        panic!("Adjustmets count is more than expected. Kindly check the count and rerun.");
    }

    let (log, diag_log) = log::setup_loggers(
        &config_params.log_file_path(),
        &config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);

    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    calculate_ftp(&log, &diag_log, &config_params);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main timer.");
    let log_dur = format!(
        "Total Duration taken by FTP Stamper Maturity close: {:?}",
        total_duration
    );
    info!(log, "{}", log_dur);
    println!("{}", log_dur);
}
