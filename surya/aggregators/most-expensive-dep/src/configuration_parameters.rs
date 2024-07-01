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
    input_file: String,
    output_file: String,
    metadata_file: String,
    req_field_file: String,
    mat_config_file: String,
    as_on_date: NaiveDate,
    exchange_rate_file: String,
    base_currency: String,
    is_consolidated: bool,
    is_account_level_exchange_rate: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "metadata_file: {}", self.metadata_file());
        info!(logger, "req_field_file: {}", self.req_field_file());
        info!(logger, "mat_config_file: {}", self.mat_config_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(
            logger,
            "is_account_level_exchange_rate: {}",
            self.is_account_level_exchange_rate()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let metadata_file = matches
            .value_of("metadata_file")
            .expect("Error getting `metadata_file`.")
            .to_string();
        let req_field_file = matches
            .value_of("req_field_file")
            .expect("Error getting `req_field_file`.")
            .to_string();
        let mat_config_file = matches
            .value_of("mat_config_file")
            .expect("Error getting `mat_config_file`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let is_account_level_exchange_rate = matches
            .value_of("is_account_level_exchange_rate")
            .expect("Error getting `is_account_level_exchange_rate`.")
            .parse::<bool>()
            .expect("Cannot parse `is_account_level_exchange_rate` as bool.");
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
            input_file,
            output_file,
            metadata_file,
            req_field_file,
            mat_config_file,
            as_on_date,
            exchange_rate_file,
            base_currency,
            is_consolidated,
            is_account_level_exchange_rate,
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
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn metadata_file(&self) -> &str {
        &self.metadata_file
    }
    pub fn req_field_file(&self) -> &str {
        &self.req_field_file
    }
    pub fn mat_config_file(&self) -> &str {
        &self.mat_config_file
    }
    pub fn as_on_date(&self) -> NaiveDate {
        self.as_on_date
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn is_account_level_exchange_rate(&self) -> bool {
        self.is_account_level_exchange_rate
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
        .about("Program to generate most expensive dep report!!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file")
                .long("metadata-file")
                .value_name("Metadata File Path")
                .help("Path to metadata file.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_field_file")
                .long("req-field-file")
                .value_name("Req Field File Path")
                .help("Path to req field file.")
                .required(true)
        )
        .arg(
            Arg::with_name("mat_config_file")
                .long("mat-config-file")
                .value_name("Maturity File Path")
                .help("Path to maturity file.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On Date")
                .help("Program run date.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("Exchange Rate File Path")
                .help("Path to exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("Base Currency.")
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
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount is consolidated of native.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_account_level_exchange_rate")
                .long("is-account-level-exchange-rate")
                .value_name("IS ACCOUNT LEVEL EXCHANGE RATE")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether exchange rate to be picked from account level or file level.")
                .default_value("false")
                .required(false)
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
