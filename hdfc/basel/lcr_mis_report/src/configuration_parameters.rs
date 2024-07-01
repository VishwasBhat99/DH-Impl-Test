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
    pub ret_cust_path: String,
    pub master_file_path: String,
    pub dis_smry_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub as_on_date: NaiveDate,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "ret_cust_path: {}", self.ret_cust_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "dis_smry_path: {}", self.dis_smry_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let ret_cust_path = matches
            .value_of("ret_cust_path")
            .expect("Error getting `ret_cust_path`.")
            .to_string();
        let master_file_path = matches
            .value_of("master_file_path")
            .expect("Error getting `master_file_path`.")
            .to_string();
        let dis_smry_path = matches
            .value_of("dis_smry_path")
            .expect("Error getting `dis_smry_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
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
            ret_cust_path,
            master_file_path,
            dis_smry_path,
            log_file_path,
            diagnostics_file_path,
            as_on_date,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn ret_cust_path(&self) -> &str {
        &self.ret_cust_path
    }
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn dis_smry_path(&self) -> &str {
        &self.dis_smry_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
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
        .about("Program to Find the Summary of Retail Customer")
        .arg(
            Arg::new("ret_cust_path")
                .long("ret-cust-path")
                .value_name("Cust File Path")
                .help("Path to cust file path.")
                .required(true)
        )
        .arg(
            Arg::new("master_file_path")
                .long("master-file-path")
                .value_name("master  File Path")
                .help("Path to Master file.")
                .required(true)
        )
        .arg(
            Arg::new("dis_smry_path")
                .long("dis-smry-path")
                .value_name("Disclosure summary Output File Path")
                .help("Path to Disclosure summary output file.")
                .required(true)
        )
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
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
        .get_matches()
}
