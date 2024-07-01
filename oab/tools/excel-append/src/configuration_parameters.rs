use clap;
use clap::{Arg, Command};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file1: String,
    sheet_name1: String,
    input_file2: String,
    sheet_name2: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file1: {}", self.input_file1());
        info!(logger, "sheet_name1: {}", self.sheet_name1());
        info!(logger, "input_file2: {}", self.input_file2());
        info!(logger, "sheet_name2: {}", self.sheet_name2());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file1 = matches
            .value_of("input_file1")
            .expect("Error getting `input_file1`.")
            .to_string();
        let input_file2 = matches
            .value_of("input_file2")
            .expect("Error getting `input_file2`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
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
        let sheet_name1 = matches
            .value_of("sheet_name1")
            .expect("Error getting `sheet_name1`.")
            .to_string();
        let sheet_name2 = matches
            .value_of("sheet_name2")
            .expect("Error getting `sheet_name2`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file1,
            input_file2,
            as_on_date,
            sheet_name1,
            sheet_name2,
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
    pub fn input_file1(&self) -> &str {
        &self.input_file1
    }
    pub fn input_file2(&self) -> &str {
        &self.input_file2
    }
    pub fn sheet_name1(&self) -> &str {
        &self.sheet_name1
    }
    pub fn sheet_name2(&self) -> &str {
        &self.sheet_name2
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
    Command::new(app_name)
        .about("Program to append one excel file content to another excel file.")
        .arg(
            Arg::new("input_file1")
                .long("input-file-1")
                .value_name("Input File 1")
                .help("Input file 1 path.")
                .required(true)
        )
        .arg(
            Arg::new("input_file2")
                .long("input-file-2")
                .value_name("Input File 2")
                .help("Input file 2 path.")
                .required(true)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("sheet_name1")
                .long("sheet-name-1")
                .value_name("Input File 1 Sheet Name.")
                .help("Input File 1 Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("sheet_name2")
                .long("sheet-name-2")
                .value_name("Input File 2 Sheet Name.")
                .help("Input File 2 Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
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
        .get_matches()
}
