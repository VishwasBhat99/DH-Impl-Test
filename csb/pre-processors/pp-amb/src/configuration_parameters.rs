use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    req_cols: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    date_format: String,
    as_on_date: NaiveDate,
    op: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "Req columns in order: {}", self.req_cols());
        info!(logger, "date_format: {}", self.date_format());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "Task: {}", self.operation());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let req_cols = matches
            .value_of("req_cols")
            .expect("Error getting `req_cols`.")
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
        let date_format = match matches
            .value_of("date_format")
            .expect("Error getting `date_format` value.")
        {
            "DD-MM-YYYY" => "%d-%m-%Y",
            "DDMMYYYY" => "%d%m%Y",
            _ => panic!("Unidentified date format"),
        }
        .to_string();
        let op = matches
            .value_of("op")
            .expect("Error getting `task`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );

        ConfigurationParameters {
            input_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            req_cols,
            date_format,
            as_on_date,
            op,
        }
    }
}

impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
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
    pub fn req_cols(&self) -> &str {
        &self.req_cols
    }
    pub fn as_on_date(&self) -> NaiveDate {
        self.as_on_date
    }
    pub fn date_format(&self) -> &str {
        &self.date_format
    }
    pub fn operation(&self) -> &str {
        &self.op
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("The program extracts required columns.")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to write logs.")
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
        .arg(
            Arg::with_name("req_cols")
                .long("req-cols")
                .value_name("REQUIRED COLUMNS")
                .help("The required '|' seperated column numbers of acc_no|date|amt|int_rt.")
                .required(true)
        )
        .arg(
            Arg::with_name("date_format")
                .long("date-format")
                .value_name("Date Format")
                .help("Date Format")
                .default_value("DDMMYYYY")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On date")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("op")
                .long("task")
                .value_name("Task")
                .help("This flag determines to extract input files or to delete temporary files.")
                .possible_values(&["extract","delete"])
                .default_value("extract")
                .required(false)
        )
        .get_matches()
}
