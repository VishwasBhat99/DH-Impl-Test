use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    cust_id_column: usize,
    residual_mat_days: i64,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    as_on_date: NaiveDate,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path);
        info!(logger, "output_file_path: {}", self.output_file_path);
        info!(logger, "log_file: {}", self.log_file_path);
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path);
        info!(logger, "log_level: {}", self.log_level);
        info!(
            logger,
            "diagnostics_flag: {}", self.is_perf_diagnostics_enabled
        );
        info!(logger, "cust_id_column: {}", self.cust_id_column);
        info!(logger, "as_on_date: {}", self.as_on_date);
        info!(logger, "residual_mat_days: {}", self.residual_mat_days);
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error while getting `as_on_date`."),
        );
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
        let residual_mat_days = matches
            .value_of("residual_mat_days")
            .expect("Error getting `residual_mat_days`.")
            .parse::<i64>()
            .expect("Cannot parse `residual_mat_days` as integer.");
        let cust_id_column = matches
            .value_of("cust_id_column")
            .expect("Error getting `cust_id_column`.")
            .parse::<usize>()
            .expect("Cannot parse `cust_id_column` as usize.");
        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            cust_id_column,
            residual_mat_days
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
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
    pub fn cust_id_column(&self) -> usize {
        self.cust_id_column
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn residual_mat_days(&self) -> i64 {
        self.residual_mat_days
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("LIEN PREPROCESSOR")
        .version("1.0.4337")
        .author("Saurabh Singh <saurabh.s@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
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
            Arg::with_name("residual_mat_days")
                .long("res-mat-days")
                .value_name("RESIDUAL MATURITY DAYS")
                .help("The residual days to be compared with tenor.")
                .default_value("30")
                .required(false)
        )
        .arg(
            Arg::with_name("cust_id_column")
                .long("cust-id-column")
                .value_name("CUSTOMER ID COLUMN")
                .help("This flag that help you pick the column for customer id.")
                .required(true)
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
