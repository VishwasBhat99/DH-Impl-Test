use chrono::NaiveDate;
use clap::{App, Arg};
use rbdate::DateParser;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub config_file_path: String,
    pub output_file_path: String,
    pub output_sheet_name: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub as_on_date: NaiveDate,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "output_sheet_name: {}", self.output_sheet_name());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "config_file_path: {}", self.config_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file` value.")
            .to_string();
        let output_sheet_name = matches
            .value_of("output_sheet_name")
            .expect("Error getting `output_sheet_name` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file` value.")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let mut diagnostics_file_path = matches
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

        ConfigurationParameters {
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            as_on_date,
            output_sheet_name,
            config_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
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
    pub fn as_on_date(&self) -> NaiveDate {
        self.as_on_date
    }
    pub fn output_sheet_name(&self) -> &str {
        &self.output_sheet_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This program merge the given excel into one final excel")
        .version("1.0.4706")
        .author("sonali.s<sonali.s@surya-soft.com>")
        .arg(
            Arg::new("output_file_path")
                .long("output-file")
                .value_name("OUTPUT FILE")
                .help("Path to Output file.")
                .required(true)
        )
        .arg(
            Arg::new("config_file_path")
            .long("config-file")
            .value_name("CONFIG FILE")
            .help("Path of config file")
            .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("LOG FILE")
                .help("Path to write logs file.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAGNOSTIC FILE")
                .help("Path to write diagnostics logs file.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_sheet_name")
            .long("output-sheet-name")
            .value_name("MASTER SHEET NAME")
            .help("sec loan master sheet name")
            .required(true)
        )
       
        .get_matches()
}
