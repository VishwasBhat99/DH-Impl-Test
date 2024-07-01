use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub cust_master_file: String,
    pub ucic_master_file: String,
    pub ucic_field_delimiter: String,
    pub cust_field_delimiter: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "ucic_master_file: {}", self.ucic_master_file());
        info!(
            logger,
            "ucic_field_delimiter: {}",
            self.ucic_field_delimiter()
        );
        info!(logger, "cust_master: {}", self.cust_master_file());
        info!(
            logger,
            "cust_field_delimiter: {}",
            self.cust_field_delimiter()
        )
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
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let ucic_master_file = matches
            .value_of("ucic_master_file")
            .expect("Error getting `ucic_master_file`.")
            .to_string();
        let ucic_field_delimiter = matches
            .value_of("ucic_field_delimiter")
            .expect("Error getting ucic_field_delimiter")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let cust_master_file = matches
            .value_of("cust_master_file")
            .expect("Error getting `cust_master_file`.")
            .to_string();
        let cust_field_delimiter = matches
            .value_of("cust_field_delimiter")
            .expect("Error getting cust_field_delimiter")
            .to_string();

        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            ucic_master_file,
            ucic_field_delimiter,
            log_level,
            is_perf_diagnostics_enabled,
            output_file_path,
            cust_field_delimiter,
            cust_master_file,
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
    pub fn ucic_master_file(&self) -> &str {
        &self.ucic_master_file
    }
    pub fn ucic_field_delimiter(&self) -> &str {
        &self.ucic_field_delimiter
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn cust_field_delimiter(&self) -> &str {
        &self.cust_field_delimiter
    }
    pub fn cust_master_file(&self) -> &str {
        &self.cust_master_file
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("UCIC Mapping Program EDW")
        .author("Saurabh Singh <saurabh.s@surya-soft.com>")
        .version("1.1.4515")
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
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to read Output File.")
                .required(true)
        )
        .arg(
            Arg::new("ucic_master_file")
                .long("ucic-master-file")
                .value_name("ucic_master_file")
                .help("Path to read Mapping Master File.")
                .required(true)
        )
        .arg(
            Arg::new("cust_master_file")
                .long("cust-master-file")
                .value_name("cust_master_file")
                .help("Path to read Cust Master File.")
                .required(true)
        )
        .arg(
            Arg::new("ucic_field_delimiter")
                .long("ucic-field-delimiter")
                .value_name("ucic_field_delimiter")
                .help("Delimiter used in UCIC Master File")
                .required(true)
        )
        .arg(
            Arg::new("cust_field_delimiter")
                .long("cust-field-delimiter")
                .value_name("cust_field_delimiter")
                .help("Delimiter used in CUST Master File")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(["true", "false"])
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
        .get_matches()
}