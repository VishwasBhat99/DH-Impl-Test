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
    output_file_path: String,
    as_on_date: NaiveDate,
    base_currency: String,
    country: String,
    currency_conversion_file_path: String,
    log_file_path: String,
    req_fields_file_path: String,
    llg_neg: String,
    llg_abs: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_negative: bool,
    is_consolidated: bool,
    is_account_level_exchange_rate: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "country: {}", self.country());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "llg_neg: {}", self.llg_neg());
        info!(logger, "llg_abs: {}", self.llg_abs());
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "is_negative: {}", self.is_negative());
        info!(
            logger,
            "is_account_level_exchange_rate: {}",
            self.is_account_level_exchange_rate()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let country = matches
            .value_of("country")
            .expect("Error getting `country`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );

        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `req_file_path`.")
            .to_string();
        let llg_neg = matches
            .value_of("llg_neg")
            .expect("Error getting llg_neg")
            .to_string();
        let llg_abs = matches
            .value_of("llg_abs")
            .expect("Error getting llg_abs")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `account metadata file path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error getting `rules_file_path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error getting `default_llg_code`.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_llg_code` as i64.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let is_negative = matches
            .value_of("is_negative")
            .expect("Error getting `is_negative`.")
            .parse::<bool>()
            .expect("Cannot parse `is_negative` as bool.");
        let is_account_level_exchange_rate = matches
            .value_of("is_account_level_exchange_rate")
            .expect("Error getting `is_account_level_exchange_rate`.")
            .parse::<bool>()
            .expect("Cannot parse `is_account_level_exchange_rate` as bool.");

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            base_currency,
            country,
            currency_conversion_file_path,
            log_file_path,
            req_fields_file_path,
            llg_neg,
            llg_abs,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_negative,
            is_consolidated,
            is_account_level_exchange_rate,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn llg_neg(&self) -> &str {
        &self.llg_neg
    }
    pub fn llg_abs(&self) -> &str {
        &self.llg_abs
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
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
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }
    pub fn is_account_level_exchange_rate(&self) -> bool {
        self.is_account_level_exchange_rate
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("BASEL-3 T1 Aggregator")
        .version("1.2.5111")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("Country")
                .help("Country instance name.")
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
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount in input is consolidated or not.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_negative")
                .long("is-negative")
                .value_name("IS NEGATIVE")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount in input is consolidated or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("is_account_level_exchange_rate")
                .long("is-acc-level-exrt")
                .value_name("IS ACCOUNT LEVEL EXCHANGE RATE")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether exchange rate in input is picked or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("BASR CURRENCY")
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("REQUIRED_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
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
            Arg::with_name("llg_neg")
            .long("llg-neg")
            .help("List of llgs which have to be multiplied by -1.")
            .default_value("")
            .required(false)
        )
        .arg(
            Arg::with_name("llg_abs")
            .long("llg-abs")
            .help("List of llgs for which absolute values are to be considered.")
            .default_value("")
            .required(false)
        )
        .get_matches()
}
