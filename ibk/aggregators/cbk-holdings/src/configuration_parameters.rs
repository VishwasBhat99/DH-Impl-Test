use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);

    let parameters = ConfigurationParameters::new_from_matches(matches);
    return parameters;
}

pub struct ConfigurationParameters {
    as_on_date: NaiveDate,
    log_file_path: String,
    config_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "is_perf_diagnostics_enabled: {}",
            self.is_perf_diagnostics_enabled()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches.value_of("config_file_path").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );

        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .unwrap()
            .to_string();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("diag_flag")
            .unwrap_or("false")
            .parse::<bool>()
            .unwrap();

        ConfigurationParameters {
            config_file_path,
            as_on_date,
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
    pub fn config_file_path(&self) -> &str {
        return &self.config_file_path;
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        return &self.as_on_date;
    }
    pub fn log_file_path(&self) -> &str {
        return &self.log_file_path;
    }
    pub fn diagnostics_file_path(&self) -> &str {
        return &self.diagnostics_file_path;
    }
    pub fn log_level(&self) -> &str {
        return &self.log_level;
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        return self.is_perf_diagnostics_enabled;
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This program generates output for MIS REPORTS CBK HOLDINGS")
        .version("1.0.3575")
        .arg(
            Arg::with_name("config_file_path")
                .short("c")
                .long("config-file")
                .value_name("FILE")
                .help("Path to the config file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .short("e")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
