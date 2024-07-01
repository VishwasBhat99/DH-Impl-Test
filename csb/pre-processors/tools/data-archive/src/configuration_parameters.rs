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
    zip_file_path: String,
    preprocess_path: String,
    cfdata_path: String,
    summary_path: String,
    logs_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    date_format: String,
    as_on_date: NaiveDate,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_path: {}", self.input_file_path());
        info!(logger, "izip_file_path: {}", self.zip_file_path());
        info!(logger, "preprocess_path: {}", self.preprocess_path());
        info!(logger, "cfdata_path: {}", self.cfdata_path());
        info!(logger, "summary_path: {}", self.summary_path());
        info!(logger, "logs_path: {}", self.logs_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "date_format: {}", self.date_format());
        info!(logger, "as_on_date: {}", self.as_on_date());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let zip_file_path = matches
            .value_of("zip_file_path")
            .expect("Error getting `zip_file_path`.")
            .to_string();
        let preprocess_path = matches
            .value_of("preprocess_path")
            .expect("Error getting `preprocess_path`.")
            .to_string();
        let cfdata_path = matches
            .value_of("cfdata_path")
            .expect("Error getting `cfdata_path`.")
            .to_string();
        let summary_path = matches
            .value_of("summary_path")
            .expect("Error getting `summary_path`.")
            .to_string();
        let logs_path = matches
            .value_of("logs_path")
            .expect("Error getting `logs_path`.")
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

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );

        ConfigurationParameters {
            input_file_path,
            zip_file_path,
            preprocess_path,
            cfdata_path,
            summary_path,
            logs_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            date_format,
            as_on_date,
        }
    }
}

impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn zip_file_path(&self) -> &str {
        &self.zip_file_path
    }
    pub fn preprocess_path(&self) -> &str {
        &self.preprocess_path
    }
    pub fn cfdata_path(&self) -> &str {
        &self.cfdata_path
    }
    pub fn summary_path(&self) -> &str {
        &self.summary_path
    }
    pub fn logs_path(&self) -> &str {
        &self.logs_path
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
    pub fn date_format(&self) -> &str {
        &self.date_format
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("The program archives the previous 2 months folder files.")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input Folder Path")
                .help("Path of InputData.")
                .required(true)
        )
        .arg(
            Arg::with_name("zip_file_path")
                .long("zip-file-path")
                .value_name("Zip File Path")
                .help("Path of Zip Archive to be created.")
                .required(true)
        )
        .arg(
            Arg::with_name("preprocess_path")
                .long("preprocess-path")
                .value_name("Preprocess Folder Path")
                .help("Path of PreprocessData.")
                .required(true)
        )
        .arg(
            Arg::with_name("cfdata_path")
                .long("cfdata-path")
                .value_name("Cfdata Folder Path")
                .help("Path of CfData.")
                .required(true)
        )
        .arg(
            Arg::with_name("summary_path")
                .long("summary-path")
                .value_name("Summarydata Folder Path")
                .help("Path of SummaryData.")
                .required(true)
        )
        .arg(
            Arg::with_name("logs_path")
                .long("logs-path")
                .value_name("Logs Folder Path")
                .help("Path of Logs.")
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
        .get_matches()
}
