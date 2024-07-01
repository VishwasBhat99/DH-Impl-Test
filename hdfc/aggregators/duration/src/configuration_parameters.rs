use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    as_on_date: String,
    input_file_path: String,
    output_file_path: String,
    delimiter: String,
    account_metadata_file_path: String,
    req_fields_file_path: String,
    rules_file_path: String,
    default_llg: String,
    consolidated_currency: String,
    local_consolidation_currency: String,
    foreign_consolidation_currency: String,
    exchange_rate_file: String,
    is_consolidated: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(
            logger,
            "consolidated_currency: {}",
            self.consolidated_currency()
        );
        info!(
            logger,
            "local_consolidation_currency: {}",
            self.local_consolidation_currency()
        );
        info!(
            logger,
            "foreign_consolidation_currency: {}",
            self.foreign_consolidation_currency()
        );
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "default_llg: {}", self.default_llg());
        info!(logger, "delimiter: {}", self.delimiter());
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let as_on_date = matches
            .value_of("as_on_date")
            .expect("Error getting `as_on_date`.")
            .to_string();
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let delimiter = matches
            .value_of("delimiter")
            .expect("Error getting `delimiter`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `account_metadata_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error getting `rules_file_path`.")
            .to_string();
        let default_llg = matches
            .value_of("default_llg")
            .expect("Error getting `default_llg`.")
            .to_string();
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let consolidated_currency = matches
            .value_of("consolidated_currency")
            .expect("Error getting `Consolidation currency`.")
            .to_string();
        let local_consolidation_currency = matches
            .value_of("local_consolidation_currency")
            .expect("Error getting `Local consolidation currency`.")
            .to_string();
        let foreign_consolidation_currency = matches
            .value_of("foreign_consolidation_currency")
            .expect("Error getting `Foreign consolidation currency`.")
            .to_string();
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
            input_file_path,
            output_file_path,
            delimiter,
            account_metadata_file_path,
            req_fields_file_path,
            rules_file_path,
            default_llg,
            consolidated_currency,
            local_consolidation_currency,
            foreign_consolidation_currency,
            exchange_rate_file,
            is_consolidated,
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
    pub fn as_on_date(&self) -> &str {
        &self.as_on_date
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }
    pub fn default_llg(&self) -> &str {
        &self.default_llg
    }
    pub fn consolidated_currency(&self) -> &str {
        &self.consolidated_currency
    }
    pub fn local_consolidation_currency(&self) -> &str {
        &self.local_consolidation_currency
    }
    pub fn foreign_consolidation_currency(&self) -> &str {
        &self.foreign_consolidation_currency
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
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
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Brief description of program to be added here!!")
        .arg(
            Arg::with_name("as_on_date")
                .long("as_on_date")
                .value_name("as on date")
                .help("as on date")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("input file path")
                .help("path to read data")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output file path")
                .help("path to write data")
                .required(true)
        )
        .arg(
            Arg::with_name("delimiter")
                .long("delimiter")
                .value_name("delimiter")
                .help("delimiter for separation")
                .required(false)
                .default_value("|")
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file-path")
                .value_name("account metadata file path")
                .help("path to account metadata")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file-path")
                .value_name("req fields file path")
                .help("path to req fields")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg")
                .long("default-llg")
                .value_name("default_llg")
                .help("value of default_llg")
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
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount is consolidated of native.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("consolidated_currency")
                .long("currency")
                .value_name("CURRENCY")
                .help("The consolidated currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("local_consolidation_currency")
                .long("local-consolidation-currency")
                .value_name("LOCAL CONSOLIDATION CURRENCY")
                .help("If the currency is INR we use this as consolidation currency")
                .required(true)
        )
        .arg(
            Arg::with_name("foreign_consolidation_currency")
                .long("foreign-consolidation-currency")
                .value_name("FOREIGN CONSOLIDATION CURRENCY")
                .help("If the currency is a foreign currency we use this as consolidation currency along with consolidated currency")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("rules file path")
                .help("path to rules data")
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
