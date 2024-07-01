use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_master_file_path: String,
    input_master_sheet_name: String,
    config_file_path: String,
    output_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    display_ccy: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "input_master_file_path: {}",
            self.input_master_file_path()
        );
        info!(
            logger,
            "input_master_sheet_name: {}",
            self.input_master_sheet_name()
        );
        info!(
            logger,
            "config_file_path: {}",
            self.config_file_path()
        );
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "display_ccy: {}", self.display_ccy());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_master_file_path = matches
            .value_of("input_master_file_path")
            .expect("Error getting `input_master_file_path`.")
            .to_string();
        let input_master_sheet_name = matches
        .value_of("input_master_sheet_name")
        .expect("Error getting `input_master_sheet_name`.")
        .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let display_ccy = matches
            .value_of("display_ccy")
            .expect("Error getting `display_ccy`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
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
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_master_file_path,
            input_master_sheet_name,
            config_file_path,
            output_file,
            as_on_date,
            display_ccy,
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
    pub fn input_master_file_path(&self) -> &str {
        &self.input_master_file_path
    }
    pub fn input_master_sheet_name(&self) -> &str {
        &self.input_master_sheet_name
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
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
    pub fn display_ccy(&self) -> &str {
        &self.display_ccy
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program for contingent liablity!!")
        .version("1.0.5068")
        .author("Sonali<sonali.s@surya-soft.com>")
        .arg(
            Arg::with_name("input_master_file_path")
                .long("input-master-file")
                .value_name("MASTER INPUT MAPPING FILE")
                .help("Path of input master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_master_sheet_name")
                .long("input-master-sheet-name")
                .value_name("INPUT MASTER SHEET NAME")
                .help("Input master sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file-path")
                .value_name("CONFIG FILE PATH")
                .help("Path to get config file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("OUTPUT")
                .help("Path to Output File.")
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
            Arg::with_name("display_ccy")
                .long("display-ccy")
                .value_name("DISPLAY CCY")
                .help("The currency that need to be displayed.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("LOG_FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAGLOG_FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG_LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS_FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
