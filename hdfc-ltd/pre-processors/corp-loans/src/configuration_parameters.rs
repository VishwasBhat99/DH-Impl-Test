use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    as_on_date: NaiveDate,
    input_master_file_path: String,
    input_cashflow_file_path: String,
    input_ledger_file_path: String,
    master_header_count: usize,
    cf_header_count: usize,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
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
            "input_cashflow_file_path: {}",
            self.input_cashflow_file_path()
        );
        info!(
            logger,
            "input_ledger_file_path: {}",
            self.input_ledger_file_path()
        );
        info!(logger, "master_header_count: {}", self.master_header_count());
        info!(logger, "cf_header_count: {}", self.cf_header_count());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
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
        let input_master_file_path = matches
            .value_of("input_master_file_path")
            .expect("Error getting `input_master_file_path`.")
            .to_string();
        let input_cashflow_file_path = matches
            .value_of("input_cashflow_file_path")
            .expect("Error getting `input_cashflow_file_path`.")
            .to_string();
        let input_ledger_file_path = matches
            .value_of("input_ledger_file_path")
            .expect("Error getting `input_ledger_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let master_header_count = matches
            .value_of("master_header_count")
            .expect("Error getting `master_header_count`.")
            .to_string()
            .parse::<usize>()
            .expect("Cannot parse `master_header_count` as usize.");
        let cf_header_count = matches
            .value_of("cf_header_count")
            .expect("Error getting `cf_header_count`.")
            .to_string()
            .parse::<usize>()
            .expect("Cannot parse `cf_header_count` as usize.");
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
            as_on_date,
            input_master_file_path,
            input_cashflow_file_path,
            input_ledger_file_path,
            output_file_path,
            cf_header_count,
            master_header_count,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn input_master_file_path(&self) -> &str {
        &self.input_master_file_path
    }
    pub fn input_cashflow_file_path(&self) -> &str {
        &self.input_cashflow_file_path
    }
    pub fn input_ledger_file_path(&self) -> &str {
        &self.input_ledger_file_path
    }
    pub fn master_header_count(&self) -> &usize {
        &self.master_header_count
    }
    pub fn cf_header_count(&self) -> &usize {
        &self.cf_header_count
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.0.2558")
        .about("Corporate loans pre-processors!!")
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_master_file_path")
                .long("input-master-file-path")
                .value_name("Input Master File Path")
                .help("Path to input master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_cashflow_file_path")
                .long("input-cashflow-file-path")
                .value_name("Input Cashflow File Path")
                .help("Path to input cashflow file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_ledger_file_path")
                .long("input-ledger-file-path")
                .value_name("Input ledger File Path")
                .help("Path to input ledger file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("master_header_count")
                .long("master-header-count")
                .value_name("Master Header Count")
                .help("Count of headers in Master File.")
                .default_value("0")
                .required(false)
        )
        .arg(
            Arg::with_name("cf_header_count")
                .long("cf-header-count")
                .value_name("CF Header Count")
                .help("Count of headers in CF File.")
                .default_value("0")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to write output.")
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
        .get_matches()
}
