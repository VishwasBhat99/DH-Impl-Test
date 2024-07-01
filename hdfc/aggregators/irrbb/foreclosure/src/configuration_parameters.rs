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
    input_file_path: String,
    tl_ids_file_path: String,
    output_file_path: String,
    req_file_path: String,
    account_metadata_file: String,
    rules_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    default_llg_code: i32,
    as_on_date: NaiveDate,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "tl_ids_file_path: {}", self.tl_ids_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "req_file: {}", self.req_file_path());
        info!(
            logger,
            "account_metadata_file: {}",
            self.account_metadata_file()
        );
        info!(logger, "rules_file: {}", self.rules_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let tl_ids_file_path = matches
            .value_of("tl_ids_file")
            .expect("Error getting `tl_ids_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let req_file_path = matches
            .value_of("req_file")
            .expect("Error getting `req_file_path`.")
            .to_string();
        let account_metadata_file = matches
            .value_of("account_metadata_file")
            .expect("Error getting `account_metadata_file`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file")
            .expect("Error getting `rules_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error while getting `default llg code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default llg code` as integer.");
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
            input_file_path,
            tl_ids_file_path,
            output_file_path,
            req_file_path,
            account_metadata_file,
            rules_file_path,
            log_file_path,
            diagnostics_file_path,
            as_on_date,
            default_llg_code,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn tl_ids_file_path(&self) -> &str {
        &self.tl_ids_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn req_file_path(&self) -> &str {
        &self.req_file_path
    }
    pub fn account_metadata_file(&self) -> &str {
        &self.account_metadata_file
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
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
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
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
        .version("1.0.1")
        .author("srinivas-644 <srinivas.r@surya-soft.com>")
        .about("Program for IRRBB Foreclosure")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("tl_ids_file")
                .long("tl-ids-file")
                .value_name("TL IDs File Path")
                .help("Path to TL IDs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output File Path")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_file")
                .long("req-file")
                .value_name("required File Path")
                .help("Path to required file.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file")
                .long("account-metadata-file")
                .value_name("Account Metadata File Path")
                .help("Path to account metadata file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file")
                .long("rules-file")
                .value_name("Rules File Path")
                .help("Path to Rules file.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
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
