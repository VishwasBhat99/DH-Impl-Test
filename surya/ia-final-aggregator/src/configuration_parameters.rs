use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub principal_file_path: String,
    pub rate_file_path: String,
    pub summary_file_path: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub as_on_date: NaiveDate,
    pub diagnostics_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(
            logger,
            "principal_file_path: {}",
            self.principal_file_path()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "rate_file_path: {}", self.rate_file_path());
        info!(logger, "summary_file_path: {}", self.summary_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let principal_file_path = matches
            .value_of("principal_file_path")
            .expect("Error getting `principal_file_path` value.")
            .to_string();
        let rate_file_path = matches
            .value_of("rate_file_path")
            .expect("Error getting `rate_file_path` value.")
            .to_string();
        let summary_file_path = matches
            .value_of("summary_file_path")
            .expect("Error getting `summary_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        ConfigurationParameters {
            principal_file_path,
            rate_file_path,
            summary_file_path,
            output_file_path,
            log_file_path,
            as_on_date,
            diagnostics_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn principal_file_path(&self) -> &str {
        &self.principal_file_path
    }
    pub fn rate_file_path(&self) -> &str {
        &self.rate_file_path
    }
    pub fn summary_file_path(&self) -> &str {
        &self.summary_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        return &self.as_on_date;
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app Aggregates and prepares data for bulk insertion")
        .arg(
            Arg::with_name("principal_file_path")
                .long("principal-file-path")
                .value_name("FILE")
                .help("Path to principal file.")
                .required(true),
        )
        .arg(
            Arg::with_name("rate_file_path")
                .long("rate-file-path")
                .value_name("FILE")
                .help("Path to rate file.")
                .required(true),
        )
        .arg(
            Arg::with_name("summary_file_path")
                .long("summary-file-path")
                .value_name("FILE")
                .help("Path to summary file.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("FILE")
                .help("Path to output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs.")
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
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics log.")
                .required(true),
        )
        .get_matches()
}
