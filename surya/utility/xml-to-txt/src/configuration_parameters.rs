use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    is_header_present: bool,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    field_separator: String,
    date_format: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "is_header_present: {}", self.is_header_present());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "field seperator: {}", self.field_separator());
        info!(logger, "date format: {}", self.date_format());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let is_header_present = matches
            .value_of("is_header_present")
            .expect("Error getting `is_header_present`.")
            .parse::<bool>()
            .expect("Can not parse `is_header_present` as bool.");
        let field_separator = matches
            .value_of("field_separator")
            .expect("Error getting `field_separator`.")
            .to_string();
        let date_format = matches
            .value_of("date_format")
            .expect("Error getting `date_format`.")
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
            is_header_present,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            field_separator,
            date_format,
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
    pub fn is_header_present(&self) -> bool {
        self.is_header_present
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
    pub fn field_separator(&self) -> &str {
        &self.field_separator
    }
    pub fn date_format(&self) -> &str {
        &self.date_format
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
        .about("A Rust Program to convert xml file to text file!!")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .version( "1.0.3738")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to Input File.")
                .required(true)
        )
        .arg(
            Arg::with_name("is_header_present")
                .long("is-header-present")
                .value_name("Is Header Present")
                .help("Is Header Present?.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to Output File.")
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
            Arg::with_name("field_separator")
                .long("field-separator")
                .value_name("Input Field separator")
                .help("Input values field separator.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::with_name("date_format")
                .long("date-format")
                .value_name("Input Date format")
                .help("Input Date format.")
                .possible_values(&["%d-%m-%Y","%Y-%m-%d","%d-%b-%Y","%Y-%b-%d","%d %m %Y","%Y %m %d","%d-%m-%y","%y-%m-%d","%d %b %Y","%Y %b %d"])
                .default_value("%d-%m-%Y")
                .required(false)
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
