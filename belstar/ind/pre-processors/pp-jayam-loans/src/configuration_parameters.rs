use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_master_file_path: String,
    pub input_cashflow_file_path: String,
    pub cf_delimiter: String,
    pub master_delimiter: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(
            logger,
            "input_master_file: {}",
            self.input_master_file_path()
        );
        info!(
            logger,
            "input_cashflow_file: {}",
            self.input_cashflow_file_path()
        );
        info!(logger, "cashflow file delimiter: {}", self.cf_delimiter());
        info!(logger, "master file delimiter: {}", self.master_delimiter());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_master_file_path = matches
            .value_of("input_master_file_path")
            .expect("Error getting `input_master_file_path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );

        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let input_cashflow_file_path = matches
            .value_of("input_cashflow_file_path")
            .expect("Error getting `input_cashflow_file_path` value.")
            .to_string();
        let cf_delimiter = matches
            .value_of("cf_delimiter")
            .expect("Error getting `cf_delimiter` value.")
            .to_string();
        let master_delimiter = matches
            .value_of("master_delimiter")
            .expect("Error getting `master_delimiter` value.")
            .to_string();
        ConfigurationParameters {
            input_master_file_path,
            input_cashflow_file_path,
            cf_delimiter,
            master_delimiter,
            as_on_date,
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
    pub fn input_master_file_path(&self) -> &str {
        &self.input_master_file_path
    }
    pub fn input_cashflow_file_path(&self) -> &str {
        &self.input_cashflow_file_path
    }
    pub fn cf_delimiter(&self) -> &str {
        &self.cf_delimiter
    }
    pub fn master_delimiter(&self) -> &str {
        &self.master_delimiter
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
        .about("Pre-processor for Jayam Loans.")
        .version("1.0.2963")
        .author("Bhargavi052 <bhargavi.n@surya-soft.com>")
        .arg(
            Arg::with_name("input_master_file_path")
                .long("input-master-file")
                .value_name("Input Master File")
                .help("Path to the input master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_cashflow_file_path")
                .long("input-cashflow-file")
                .value_name("input_cashflow_file_path")
                .help("Path to the input cashflow file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cf_delimiter")
                .long("cf-delimiter")
                .value_name("cf_delimiter")
                .help("delimiter of cashflow file accounts.")
                .default_value(",")
                .required(false)
        )
        .arg(
            Arg::with_name("master_delimiter")
                .long("master-delimiter")
                .value_name("master_delimiter")
                .help("delimiter of master file accounts.")
                .default_value(",")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
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
