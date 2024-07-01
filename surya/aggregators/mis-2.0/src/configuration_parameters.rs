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
    report_id: String,
    home_currency: String,
    display_currency: String,
    consol_currency: String,
    config_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "report_id: {}", self.report_id());
        info!(logger, "home_currency: {}", self.home_currency());
        info!(logger, "display_currency: {}", self.display_currency());
        info!(logger, "consol_currency: {}", self.consol_currency());
        info!(logger, "config_file_path: {}", self.config_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let report_id = matches
            .value_of("report_id")
            .expect("Error getting `report_id`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let home_currency = matches
            .value_of("home_currency")
            .expect("Error getting `home_currency`.")
            .to_string();
        let display_currency = matches
            .value_of("display_currency")
            .expect("Error getting `display_currency`.")
            .to_string();
        let consol_currency = matches
            .value_of("consol_currency")
            .expect("Error getting `consol_currency`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );

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
            report_id,
            home_currency,
            display_currency,
            consol_currency,
            config_file_path,
            output_file_path,
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
    pub fn report_id(&self) -> &str {
        &self.report_id
    }
    pub fn home_currency(&self) -> &str {
        &self.home_currency
    }
    pub fn display_currency(&self) -> &str {
        &self.display_currency
    }
    pub fn consol_currency(&self) -> &str {
        &self.consol_currency
    }
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("MIS-2.0")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .version("2.0.3751")
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
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
            Arg::with_name("report_id")
                .long("report-id")
                .value_name("Report ID")
                .help("Unique ID of Report.")
                .required(true)
        )
        .arg(
            Arg::with_name("home_currency")
                .long("home-currency")
                .value_name("Home Currncy")
                .help("Local Curency.")
                .required(false)
        )
        .arg(
            Arg::with_name("display_currency")
                .long("display-currency")
                .value_name("Display Currncy")
                .help("Display Curency.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("consol_currency")
                .long("consol-currency")
                .value_name("consol Currncy")
                .help("Consol Curency.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file")
                .value_name("Config File")
                .help("Path to the config file.")
                .required(true)
        )
        .get_matches()
}
