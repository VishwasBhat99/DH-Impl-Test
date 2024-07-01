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
    output_file_path: String,
    as_on_date: NaiveDate,
    config_file_path: String,
    country_code: String,
    log_file_path: String,
    diagnostics_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "country_code: {}", self.country_code());
        info!(logger, "log_file: {}", self.log_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches.value_of("config_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );

        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let country_code = matches.value_of("country_code").unwrap().to_string();

        ConfigurationParameters {
            output_file_path,
            as_on_date,
            config_file_path,
            country_code,
            log_file_path,
            diagnostics_file_path,
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
    pub fn country_code(&self) -> &str {
        &self.country_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .version("1.0.2352")
        .about("This program generates output for LIEN-Summary-Report")
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true),
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false),
        )
        .arg(
            Arg::new("log_file")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true),
        )
        .arg(
            Arg::new("config_file")
                .long("config-file")
                .value_name("FILE")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false),
        )
        .arg(
            Arg::new("country_code")
                .long("country-code")
                .value_name("Country Code")
                .help("Code of the Country")
                .required(true),
        )
        .get_matches()
}
