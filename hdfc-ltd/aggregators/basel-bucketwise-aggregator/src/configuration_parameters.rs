use clap;
use clap::{Arg, Command};
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
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    diagnostics_file_path: String,
    amt_type: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_consolidated: bool,
    is_overdue_req: bool,
    is_custom_bucket_req: bool,
    is_edate_req: bool,
    default_overdue_llg_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "country: {}", self.country());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "amt_type: {}", self.amt_type());
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
        info!(logger, "is_overdue_req: {}", self.is_overdue_req());
        info!(
            logger,
            "is_custom_bucket_req: {}",
            self.is_custom_bucket_req()
        );
        info!(logger, "is_edate_req: {}", self.is_edate_req());
        info!(
            logger,
            "default_overdue_llg_path: {}",
            self.default_overdue_llg_path()
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
        let amt_type = matches
            .value_of("amt_type")
            .expect("Error getting `amount type`.")
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
        let is_overdue_req = matches
            .value_of("is_overdue_req")
            .expect("Error getting `is_overdue_req`.")
            .parse::<bool>()
            .expect("Cannot parse `is_overdue_req` as bool.");
        let is_custom_bucket_req = matches
            .value_of("is_custom_bucket_req")
            .expect("Error getting `is_custom_bucket_req`.")
            .parse::<bool>()
            .expect("Cannot parse `is_custom_bucket_req` as bool.");
        let is_edate_req = matches
            .value_of("is_edate_req")
            .expect("Error getting `is_edate_req`.")
            .parse::<bool>()
            .expect("Cannot parse `is_edate_req` as bool.");
        let default_overdue_llg_path = matches
            .value_of("default_overdue_llg_path")
            .unwrap_or("NA")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            base_currency,
            country,
            currency_conversion_file_path,
            log_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            diagnostics_file_path,
            log_level,
            amt_type,
            default_overdue_llg_path,
            is_perf_diagnostics_enabled,
            is_consolidated,
            is_overdue_req,
            is_custom_bucket_req,
            is_edate_req,
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
    pub fn amt_type(&self) -> &str {
        &self.amt_type
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
    pub fn is_overdue_req(&self) -> bool {
        self.is_overdue_req
    }
    pub fn is_custom_bucket_req(&self) -> bool {
        self.is_custom_bucket_req
    }
    pub fn is_edate_req(&self) -> bool {
        self.is_edate_req
    }
    pub fn default_overdue_llg_path(&self) -> &str {
        &self.default_overdue_llg_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("Basel-bucket-wise-aggregator.")
        .version("1.1.4970")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("country")
                .long("country")
                .value_name("Country")
                .help("Country instance name.")
                .required(true)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
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
            Arg::new("amt_type")
                .long("amt-type")
                .value_name("AMT TYPE")
                .possible_values(["PRIN","INT"])
                .help("Amount type required in output.")
                .default_value("PRIN")
                .required(false)
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
            Arg::new("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(["true", "false"])
                .help("This flag that decides whether amount in input is consolidated or not.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::new("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::new("base_currency")
                .long("base-currency")
                .value_name("BASE CURRENCY")
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::new("req_fields_file")
                .long("req-fields-file")
                .value_name("REQUIRED_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::new("account_metadata_file_path")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::new("rules_file_path")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
                .required(true)
        )
        .arg(
            Arg::new("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .arg(
            Arg::new("is_overdue_req")
                .long("is-overdue-req")
                .value_name("IS OVERDUE FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether overdue account amounts to be considered or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("is_custom_bucket_req")
                .long("is-custom-bucket-req")
                .value_name("IS CUSTOM BUCKET FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether to bucket amount with one day less than residual days.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("is_edate_req")
                .long("is-edate-req")
                .value_name("IS EDATE FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether to consider edate function or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("default_overdue_llg_path")
                .long("default-overdue-llg-path")
                .value_name("DEFAULT OVERDUE LLG CODE")
                .help("This is the default overdue llg path.")
                .default_value("NA")
                .required(false)
        )
        .get_matches()
}
