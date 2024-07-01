use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub input_file_path: String,
    pub output_file_path: String,
    pub master_file_path: String,
    pub sheet_name: String,
    pub input_date_format: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "input_date_format: {}", self.input_date_format());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
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
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let master_file_path = matches
            .value_of("master_file")
            .expect("Error getting `master_file_path`.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let input_date_format = match matches
            .value_of("input_date_format")
            .expect("Error getting `input_date_format` value.")
        {
            "ddmmyyyy" => "%d%m%Y",
            "dd-mm-yyyy" => "%d-%m-%Y",
            "dd-mmm-yyyy" => "%d-%b-%Y",
            "yyyymmdd" => "%Y%m%d",
            "yyyy-mm-dd" => "%Y-%m-%d",
            "yyyy-mmm-dd" => "%Y-%b-%d",
            "dd-mmm-yy" => "%d-%b-%y",
            _ => panic!("Invalid Date Format!"),
        }
        .to_string();
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            output_file_path,
            master_file_path,
            sheet_name,
            log_level,
            is_perf_diagnostics_enabled,
            input_date_format,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn input_date_format(&self) -> &str {
        &self.input_date_format
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("Pre Processor for Bills (SLK and GFC)")
        .author("Saurabh Singh <Saurabh.s@surya-soft.com>")
        .version("1.0.3988")
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
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
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to read Input.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to write Output.")
                .required(true)
        )
        .arg(
            Arg::new("master_file")
                .long("master-file")
                .value_name("Master File Path")
                .help("Path to write Master.")
                .required(true)
        )
        .arg(
            Arg::new("sheet_name")
                .long("sheet-name")
                .value_name("Sheet Name")
                .help("Sheet Name")
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
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::new("input_date_format")
                .long("input-date-format")
                .value_name("Date Format")
                .help("Expected Date Format from Input File for Date Fields.")
                .possible_values(&["ddmmyyyy", "dd-mm-yyyy", "dd-mmm-yyyy", "yyyymmdd", "yyyy-mm-dd", "yyyy-mmm-dd", "dd-mmm-yy"])
                .default_value("dd-mm-yyyy")
                .required(false)
        )
        .get_matches()
}
