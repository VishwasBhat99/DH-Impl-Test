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
    pub ca_input_file_path: String,
    pub sa_input_file_path: String,
    pub output_file_path: String,
    pub td_input_file_path: String,
    pub log_level: String,
    pub is_cust_repeated: bool,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "ca_input_file_path: {}", self.ca_input_file_path());
        info!(logger, "sa_input_file_path: {}", self.sa_input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "is_cust_repeated: {}", self.is_cust_repeated());
        info!(logger, "td_input_file_path: {}", self.td_input_file_path());
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
        let ca_input_file_path = matches
            .value_of("ca_input_file")
            .expect("Error getting `ca_input_file_path`.")
            .to_string();
        let sa_input_file_path = matches
            .value_of("sa_input_file")
            .expect("Error getting `sa_input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let td_input_file_path = matches
            .value_of("td_input_file")
            .expect("Error getting `td_input_file_path`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let is_cust_repeated = matches
            .value_of("is_cust_repeated")
            .expect("Error getting `is_cust_repeated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_cust_repeated` as bool.");
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            sa_input_file_path,
            ca_input_file_path,
            output_file_path,
            td_input_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_cust_repeated,
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
    pub fn sa_input_file_path(&self) -> &str {
        &self.sa_input_file_path
    }
    pub fn ca_input_file_path(&self) -> &str {
        &self.ca_input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn td_input_file_path(&self) -> &str {
        &self.td_input_file_path
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
    pub fn is_cust_repeated(&self) -> bool {
        self.is_cust_repeated
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("MULTI DEPOSIT MASTER GENERATOR")
        .author("Saurabh Singh <Saurabh.s@surya-soft.com>")
        .version("1.0.5427")
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
            Arg::new("sa_input_file")
                .long("sa-input-file")
                .value_name("SA Input File Path")
                .help("Path to read SA Input.")
                .required(true)
        )
        .arg(
            Arg::new("ca_input_file")
                .long("ca-input-file")
                .value_name("CA Input File Path")
                .help("Path to read CA Input.")
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
            Arg::new("td_input_file")
                .long("td-input-file")
                .value_name("TD Input File Path")
                .help("Path to read TD Input.")
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
            Arg::new("is_cust_repeated")
                .long("is-cust-repeated")
                .value_name("CUST REPEATED FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether customer id is repeated in CA or SA file.")
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
        .get_matches()
}
