use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub top_n_cust_count: usize,
    pub input_file: String,
    pub output_file: String,
    pub log_file: String,
    pub diag_log_file: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "top_n_cust_count: {}", self.top_n_cust_count());
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "log_file: {}", self.log_file());
        info!(logger, "diag_log_file: {}", self.diag_log_file());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let top_n_cust_count = (matches
            .value_of("top_n_cust_count")
            .expect("Error in getting `top_n_cust_count` value."))
        .parse::<usize>()
        .expect("Error in parsing top n cust count");
        let input_file = matches
            .value_of("input_file")
            .expect("Error in getting `input_file` value.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error in getting `output_file` value.")
            .to_string();
        let log_file = matches
            .value_of("log_file")
            .expect("Error in getting `log_file` value.")
            .to_string();
        let diag_log_file = matches
            .value_of("diag_log_file")
            .expect("Error in getting `diag_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error in getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error in getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        ConfigurationParameters {
            top_n_cust_count,
            input_file,
            output_file,
            log_file,
            diag_log_file,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn top_n_cust_count(&self) -> usize {
        self.top_n_cust_count
    }
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn log_file(&self) -> &str {
        &self.log_file
    }
    pub fn diag_log_file(&self) -> &str {
        &self.diag_log_file
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

pub fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .author("Paris BS. <paris.bs@surya-soft.com>")
        .about("Program for prepocessing the deposits file")
        .arg(
            Arg::with_name("top_n_cust_count")
                .long("top_n_cust_count")
                .help("Count for number of top n customers")
                .value_name("Top Customers Count")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .long("input_file")
                .help("Deposits data file path")
                .value_name("Deposits File")
                .required(true)
        )
               .arg(
            Arg::with_name("output_file")
                .long("output_file")
                .help("Output file path")
                .value_name("Output File")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log_file")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diag_log_file")
                .long("diag_log_file")
                .value_name("Diagnostic Log File")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log_level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics_flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
