use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    as_on_date: NaiveDate,
    input_file_path: String,
    input_sheet_name: String,
    amt_col_pos: usize,
    config_file_path: String,
    denomination: f64,
    output_file_path: String,
    days_range: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "input_sheet_name: {}", self.input_sheet_name());
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "amt_col_pos: {}", self.amt_col_pos());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "denomination: {}", self.denomination());
        info!(logger, "days_range: {}", self.days_range());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let input_sheet_name = matches
            .value_of("input_sheet_name")
            .expect("Error getting `input_sheet_name`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let amt_col_pos: usize = matches
            .value_of("amt_col_pos")
            .expect("Error getting `amt_col_pos`.")
            .parse()
            .expect("Cannot parse `amt_col_pos` as usize.");
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let denomination: f64 = matches
            .value_of("denomination")
            .expect("Error getting `denomination`.")
            .parse()
            .expect("Cannot parse `denomination` as i64.");
        let days_range = matches
            .value_of("days_range")
            .expect("Error getting `days_range`.")
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
            as_on_date,
            input_file_path,
            input_sheet_name,
            config_file_path,
            amt_col_pos,
            denomination,
            output_file_path,
            days_range,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn input_sheet_name(&self) -> &str {
        &self.input_sheet_name
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn amt_col_pos(&self) -> &usize {
        &self.amt_col_pos
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn denomination(&self) -> &f64 {
        &self.denomination
    }
    pub fn days_range(&self) -> &str {
        &self.days_range
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
        .about("to convert MABS data to cashflow!!")
        .arg(
            Arg::with_name("as_on_date")
                .long("as_on_date")
                .value_name("as on date")
                .help("as on date")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("input file path")
                .help("path to read data")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sheet_name")
                .long("input-sheet-name")
                .value_name("input sheet name")
                .help("name of sheet to process")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .value_name("config file path")
                .help("path to config file path")
                .required(true)
        )
        .arg(
            Arg::with_name("amt_col_pos")
                .long("amt-col-pos")
                .value_name("amount column position")
                .help("position of amount starting column")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output file path")
                .help("path to write data")
                .required(true)
        )
        .arg(
            Arg::with_name("denomination")
                .long("denomination")
                .value_name("denomination")
                .help("Reprsentation of amount in multiple of")
                .default_value("100000")
                .required(false)
        )
        .arg(
            Arg::with_name("days_range")
                .long("days_range")
                .value_name("all the range dates")
                .help("give the ranges ex(1D,2D,7D,1,) etc")
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
