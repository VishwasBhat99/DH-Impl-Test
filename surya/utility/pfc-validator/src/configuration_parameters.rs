use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub file_format: String,
    pub input_sheet_name: String,
    pub input_delimeter: String,
    pub amt_field_pos: usize,
    pub header_count: usize,
    pub footer_count: usize,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "file_format: {}", self.file_format());
        info!(logger, "input_sheet_name: {}", self.input_sheet_name());
        info!(logger, "input_delimeter: {}", self.input_delimeter());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "header_count: {}", self.header_count());
        info!(logger, "footer_count: {}", self.footer_count());
        info!(logger, "amt_field_pos: {}", self.amt_field_pos());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let header_count = matches
            .value_of("header_count")
            .expect("Error getting `header_count` value.")
            .parse::<usize>()
            .expect("Cannot parse `header_count` value as usize.");
        let footer_count = matches
            .value_of("footer_count")
            .expect("Error getting `footer_count` value.")
            .parse::<usize>()
            .expect("Cannot parse `footer_count` value as usize.");
        let amt_field_pos = matches
            .value_of("amt_field_pos")
            .expect("Error getting `amt_field_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `amt_field_pos` value as usize.");
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
        let file_format = matches
            .value_of("file_format")
            .expect("Error getting `file_format` value.")
            .to_string();
        let input_sheet_name = matches
            .value_of("input_sheet_name")
            .expect("Error getting `input_sheet_name` value.")
            .to_string();
        let input_delimeter = matches
            .value_of("input_delimeter")
            .expect("Error getting `input_delimeter` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            file_format,
            input_sheet_name,
            input_delimeter,
            amt_field_pos,
            header_count,
            footer_count,
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
    pub fn header_count(&self) -> usize {
        self.header_count
    }
    pub fn footer_count(&self) -> usize {
        self.footer_count
    }
    pub fn amt_field_pos(&self) -> usize {
        self.amt_field_pos
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
    pub fn file_format(&self) -> &str {
        &self.file_format
    }
    pub fn input_sheet_name(&self) -> &str {
        &self.input_sheet_name
    }
    pub fn input_delimeter(&self) -> &str {
        &self.input_delimeter
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app validates the file!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("header_count")
                .long("head-count")
                .value_name("Header Count")
                .help("Header to be skipped.")
                .required(true)
        )
        .arg(
            Arg::with_name("footer_count")
                .long("foot-count")
                .value_name("Footer Count")
                .help("Footer to be skipped.")
                .required(true)
        )
        .arg(
            Arg::with_name("amt_field_pos")
                .long("amt-pos")
                .value_name("Amount Field Position")
                .help("Position of Amount Field.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Diagnostics log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
            Arg::with_name("file_format")
                .long("file-format")
                .value_name("File Format")
                .help("File Format.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("input_sheet_name")
                .long("input-sheet-name")
                .value_name("input_sheet_name")
                .help("Sheet name for Input file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("input_delimeter")
                .long("input-delimeter")
                .value_name("input_delimeter")
                .help("Delimeter used in Input file.")
                .required(false)
        )
        .get_matches()
}
