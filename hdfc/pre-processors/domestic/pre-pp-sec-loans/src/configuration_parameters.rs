use chrono::NaiveDate;
use clap::{App, Arg};
use rbdate::DateParser;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub master_file_path: String,
    pub yield_file_path: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub as_on_date: NaiveDate,
    pub sec_loan_master_sheet_name: String,
    pub sec_loan_cashflows_sheet_name: String,
    pub non_concat_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "yield_file_path: {}", self.yield_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "sec_loan_master_sheet_name: {}",
            self.sec_loan_master_sheet_name()
        );
        info!(
            logger,
            "sec_loan_cashflows_sheet_name: {}",
            self.sec_loan_cashflows_sheet_name()
        );
        info!(
            logger,
            "non_concat_file_path: {}",
            self.non_concat_file_path()
        )
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let master_file_path = matches
            .value_of("master_file_path")
            .expect("Error getting `master_file` value.")
            .to_string();
        let yield_file_path = matches
            .value_of("yield_file_path")
            .expect("Error getting `master_file` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file` value.")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let mut diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let sec_loan_master_sheet_name = matches
            .value_of("sec_loan_master_sheet_name")
            .expect("Error getting `sec_loan_master_sheet_name` value.")
            .to_string();
        let sec_loan_cashflows_sheet_name = matches
            .value_of("sec_loan_cashflows_sheet_name")
            .expect("Error getting `sec_loan_cashflow_sheet_name` value.")
            .to_string();
        let non_concat_file_path = matches
            .value_of("non_concat_file_path")
            .expect("Error getting `non_concat_file_path` value.")
            .to_string();

        ConfigurationParameters {
            master_file_path,
            yield_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            as_on_date,
            sec_loan_master_sheet_name,
            sec_loan_cashflows_sheet_name,
            non_concat_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn yield_file_path(&self) -> &str {
        &self.yield_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn as_on_date(&self) -> NaiveDate {
        self.as_on_date
    }
    pub fn sec_loan_cashflows_sheet_name(&self) -> &str {
        &self.sec_loan_cashflows_sheet_name
    }
    pub fn sec_loan_master_sheet_name(&self) -> &str {
        &self.sec_loan_master_sheet_name
    }
    pub fn non_concat_file_path(&self) -> &str {
        &self.non_concat_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of pre pp sec loans")
        .version("1.0.4354")
        .author("sonali.s<sonali.s@surya-soft.com>")
        .arg(
            Arg::new("master_file_path")
                .long("master-file")
                .value_name("MASTER FILE")
                .help("Path to master file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::new("yield_file_path")
            .long("yield-file")
            .value_name("YIELD FILE")
            .help("Path to yield file that needs to be processed.")
            .required(true)
        )
        .arg(
            Arg::new("output_file_path")
                .long("output-file")
                .value_name("OUTPUT FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("LOG FILE")
                .help("Path to write logs file.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAGNOSTIC FILE")
                .help("Path to write diagnostics logs file.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("sec_loan_master_sheet_name")
            .long("sec-master-sheet")
            .value_name("MASTER SHEET NAME")
            .help("sec loan master sheet name")
            .required(true)
        )
        .arg(
            Arg::with_name("sec_loan_cashflows_sheet_name")
            .long("sec-cashflows-sheet")
            .value_name("CASHFLOW SHEET NAME")
            .help("sec loan cashflow sheet name")
            .required(true)
        )
        .arg(
            Arg::with_name("non_concat_file_path")
            .long("non-concat-file-path")
            .value_name("NON CONCAT")
            .help("non concat file path")
            .required(true)
        )
        .get_matches()
}
