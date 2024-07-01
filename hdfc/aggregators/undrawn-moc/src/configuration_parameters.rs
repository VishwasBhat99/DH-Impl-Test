use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub summary_file_path: String,
    pub config_file_path: String,
    pub output_file_p1_path: String,
    pub output_file_p2_path: String,
    pub as_on_date: NaiveDate,
    pub country: String,
    pub currency: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "summary_file: {}", self.summary_file_path());
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "output_file_p1: {}", self.output_file_p1_path());
        info!(logger, "output_file_p2: {}", self.output_file_p2_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file`.")
            .to_string();
        let summary_file_path = matches
            .value_of("summary_file")
            .expect("Error getting `summary_file`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file")
            .expect("Error getting `config_file`.")
            .to_string();
        let output_file_p1_path = matches
            .value_of("output_file_p1")
            .expect("Error getting `output_file_p1`.")
            .to_string();
        let output_file_p2_path = matches
            .value_of("output_file_p2")
            .expect("Error getting `output_file_p2`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let country = matches
            .value_of("country")
            .expect("Error getting `Country`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `Currency`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file_path,
            summary_file_path,
            config_file_path,
            as_on_date,
            country,
            currency,
            output_file_p1_path,
            output_file_p2_path,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn summary_file_path(&self) -> &str {
        &self.summary_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file_p1_path(&self) -> &str {
        &self.output_file_p1_path
    }
    pub fn output_file_p2_path(&self) -> &str {
        &self.output_file_p2_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn currency(&self) -> &str {
        &self.currency
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
        .about("Undrawn MOC Automation")
        .version("1.0.4513")
        .author("SachinMulgir <sachin.m@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Input File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("summary_file")
                .long("summary-file")
                .value_name("SUMMARY File")
                .help("SUMMARY File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .value_name("CONFIG File")
                .help("CONFIG File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_p1")
                .long("output-file-p1")
                .value_name("Output File Process 1")
                .help("Path to Output file Process 1.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_p2")
                .long("output-file-p2")
                .value_name("Output File Process 2")
                .help("Path to Output file Process 2.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("COUNTRY")
                .help("The value to country variable")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("CURRENCY")
                .help("The value to currency variable.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
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
        .get_matches()
}
