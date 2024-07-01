use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    config_file_path: String,
    as_on_date: NaiveDate,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger){
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "diagnostics_file_path: {}", self.diagnostics_file_path());
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "is_perf_diagnostics_enabled: {}", self.is_perf_diagnostics_enabled());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `ouput_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file_path")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_file_path")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
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
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            config_file_path,
            as_on_date,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .author("Ravindar singh<ravindar.sr@surya-soft.com>")
        .version("1.0.0")
        .about("This program generates output for aggregated value of average balance and eop_balance on the basis of Division and PSL Catagory")
        .arg(
            Arg::with_name("output_file_path")
                .short("o")
                .long("output-file-path")
                .value_name("output file path")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("log file path")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_file_path")
                .long("diagnostics-file-path")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file-path")
                .value_name("config file name")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false),
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
