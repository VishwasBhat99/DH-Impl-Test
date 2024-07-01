use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    tot_gsec_bv: f64,
    tot_gsec_mv: f64,
    ndtl_val: f64,
    excess_slr_val: f64,
    slr_pct: f64,
    as_on_date: NaiveDate,
    msf_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "tot_gsec_bv: {}", self.tot_gsec_bv());
        info!(logger, "tot_gsec_mv: {}", self.tot_gsec_mv());
        info!(logger, "ndtl_val: {}", self.ndtl_val());
        info!(logger, "excess_slr_val: {}", self.excess_slr_val());
        info!(logger, "slr_pct: {}", self.slr_pct());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "msf_file: {}", self.msf_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let tot_gsec_bv: f64 = matches
            .value_of("tot_gsec_bv")
            .expect("Error getting `tot_gsec_bv`.")
            .parse()
            .expect("Invalid total gsec bv.");
        let tot_gsec_mv: f64 = matches
            .value_of("tot_gsec_mv")
            .expect("Error getting `tot_gsec_mv`.")
            .parse()
            .expect("Invalid total gsec mv.");
        let ndtl_val: f64 = matches
            .value_of("ndtl_val")
            .expect("Error getting `ndtl_val`.")
            .parse()
            .expect("Invalid NDTL value.");
        let excess_slr_val: f64 = matches
            .value_of("excess_slr_val")
            .expect("Error getting `excess_slr_val`.")
            .parse()
            .expect("Invalid excess slr value.");
        let slr_pct: f64 = matches
            .value_of("slr_pct")
            .expect("Error getting `slr_pct`.")
            .parse()
            .expect("Invalid slr pct value.");

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error while getting `as_on_date`."),
        );

        let msf_file_path = matches
            .value_of("msf_file")
            .expect("Error getting `msf_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            tot_gsec_bv,
            tot_gsec_mv,
            ndtl_val,
            excess_slr_val,
            slr_pct,
            as_on_date,
            msf_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn tot_gsec_bv(&self) -> &f64 {
        &self.tot_gsec_bv
    }
    pub fn tot_gsec_mv(&self) -> &f64 {
        &self.tot_gsec_mv
    }
    pub fn ndtl_val(&self) -> &f64 {
        &self.ndtl_val
    }
    pub fn excess_slr_val(&self) -> &f64 {
        &self.excess_slr_val
    }
    pub fn slr_pct(&self) -> &f64 {
        &self.slr_pct
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn msf_file_path(&self) -> &str {
        &self.msf_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("CF Generator for NDTL")
        .version("1.0.3570")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .arg(
            Arg::with_name("tot_gsec_bv")
                .long("tot-gsec-bv")
                .value_name("Total Gsec BV")
                .help("Total Gsec BV.")
                .required(true)
        )
        .arg(
            Arg::with_name("tot_gsec_mv")
                .long("tot-gsec-mv")
                .value_name("Total Gsec MV")
                .help("Total Gsec MV.")
                .required(true)
        )
        .arg(
            Arg::with_name("ndtl_val")
                .long("ndtl-val")
                .value_name("NDTL Amount")
                .help("NDTL Amount.")
                .required(true)
        )
        .arg(
            Arg::with_name("excess_slr_val")
                .long("excess-slr-val")
                .value_name("Excess SLR Amount")
                .help("Excess SLR Amount.")
                .required(true)
        )
        .arg(
            Arg::with_name("slr_pct")
                .long("slr-pct")
                .value_name("SLR Percentage")
                .help("SLR Percentage.")
                .required(true)
        )
        .arg(
            Arg::with_name("msf_file")
                .long("msf-file")
                .value_name("MSF File")
                .help("Path to the MSF file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
