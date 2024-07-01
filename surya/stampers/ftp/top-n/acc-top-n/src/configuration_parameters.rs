use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    top_n_input_file_path: String,
    output_file_path: String,
    config_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    tot_field_number: i32,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "top_n_input_file_path: {}",
            self.top_n_input_file_path()
        );
        info!(
            logger,
            "tot_field_number: {}",
            self.tot_field_number()
        )
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let top_n_input_file_path = matches
            .value_of("top_n_input_file_path")
            .expect("Error getting `top_n_input_file_path`")
            .to_string();
        let tot_field_number=matches
        .value_of("tot_field_number")
        .expect("Error getting `tot_field_number`")
        .parse::<i32>()
        .expect("Error parsing `tot_field_number` to integer");

        ConfigurationParameters {
            config_file_path,
            output_file_path,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            top_n_input_file_path,
            tot_field_number,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn top_n_input_file_path(&self) -> &str {
        &self.top_n_input_file_path
    }
    pub fn tot_field_number(&self)-> i32{
           self.tot_field_number
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program to print top n accounts on the basis on net profit")
        .author("sonali <sonali.s@surya-soft.com>")
        .version("1.0.4340")
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
            Arg::with_name("log_file")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
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
            Arg::with_name("config_file_path")
                .long("config-file")
                .value_name("CONFIG FILE")
                .help("Path to the config file")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("OUTPUT FILE")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true),
        )
        .arg(
            Arg::with_name("top_n_input_file_path")
                .long("top-n-input-file")
                .value_name("TOP N INPUT FILE")
                .help("Path to read top n input file")
                .required(true),
        )
        .arg(
            Arg::with_name("tot_field_number")
                .long("tot-field-number")
                .value_name("TOTAL FIELD NUMBER")
                .help("This variable decide the tot field in stamper file")
                .required(true),
        )
        .get_matches()
}
