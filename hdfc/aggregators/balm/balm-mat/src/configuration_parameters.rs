use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    consolidated_currency: String,
    local_consolidation_currency: String,
    foreign_consolidation_currency: String,
    currency_conversion_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    default_overdue_llg_code: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_account_level_exchange_rate: bool,
    is_consolidated: bool,
    is_npa: String,
    npa_values: Vec<String>,
    is_rep_mandatory: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
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
        info!(
            logger,
            "default_overdue_llg_code: {}",
            self.default_overdue_llg_code()
        );
        info!(
            logger,
            "is_account_level_exchange_rate: {}",
            self.is_account_level_exchange_rate()
        );
        info!(logger, "is_npa: {}", self.is_npa());
        info!(logger, "npa_values: {:#?}", self.npa_values());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "is_rep_mandatory: {}", self.is_rep_mandatory());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `Input file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `Output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `General log file path`.")
            .to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `Req fields file path`.")
            .to_string();

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
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `File level exchange rate file path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `Diagnostics log file path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `Account metadata file path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error while getting rules file path.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error while getting `default llg code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default llg code` as integer.");
        let default_overdue_llg_code = matches
            .value_of("default_overdue_llg_code")
            .unwrap_or("0")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default overdue llg code` as integer.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting ``.")
            .parse::<bool>()
            .expect("Error while parsing `is perf diagnostics enabled` as bool.");
        let is_account_level_exchange_rate = matches
            .value_of("account_level_exchange_rate")
            .expect("Error while getting ``.")
            .parse::<bool>()
            .expect("Error while parsing `account level exchange rate` as bool.");
        let is_consolidated = matches
            .value_of("is_consolidated")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let is_rep_mandatory = matches
            .value_of("is_rep_mandatory")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let is_npa = matches
            .value_of("is_npa")
            .expect("Error while getting is_npa field name.")
            .to_string();
        let npa_values: Vec<String> = matches
            .value_of("npa_values")
            .expect("Error while getting npa_values.")
            .to_string()
            .split(',')
            .map(|s| s.to_uppercase().to_string())
            .collect();

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            consolidated_currency,
            local_consolidation_currency,
            foreign_consolidation_currency,
            currency_conversion_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            default_overdue_llg_code,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_account_level_exchange_rate,
            is_consolidated,
            is_npa,
            npa_values,
            is_rep_mandatory,
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
    pub fn consolidated_currency(&self) -> &str {
        &self.consolidated_currency
    }
    pub fn local_consolidation_currency(&self) -> &str {
        &self.local_consolidation_currency
    }
    pub fn foreign_consolidation_currency(&self) -> &str {
        &self.foreign_consolidation_currency
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
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn is_account_level_exchange_rate(&self) -> bool {
        self.is_account_level_exchange_rate
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn is_npa(&self) -> &str {
        &self.is_npa
    }
    pub fn npa_values(&self) -> &Vec<String> {
        &self.npa_values
    }
    pub fn is_rep_mandatory(&self) -> bool {
        self.is_rep_mandatory
    }
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn default_overdue_llg_code(&self) -> i32 {
        self.default_overdue_llg_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("BALM Aggregator.")
        .version("1.1.3542")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
            Arg::with_name("account_level_exchange_rate")
                .long("account-level-exchange-rate")
                .value_name("Exchange Rate Flag")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether exchange rate will be taken from file or account level.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("is_consolidated")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether the currency is consolidated or not.")
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
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("REQ_FIELDS")
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
            Arg::with_name("default_overdue_llg_code")
                .long("default-overdue-llg-code")
                .value_name("DEFAULT OVERDUE LLG CODE")
                .help("This is the default overdue llg code.")
                .required(false)
        )
        .arg(
            Arg::with_name("is_rep_mandatory")
                .long("is-rep-mandatory")
                .value_name("is_rep_mandatory")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether repricing date is mandatory for IRS or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("is_npa")
                .long("is-npa")
                .value_name("IS NPA")
                .help("Name of NPA field is NPA to be considered")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("npa_values")
                .long("npa-values")
                .value_name("NPA VALUES")
                .help("List of NPA values to be considered, comma separated.")
                .default_value("NA")
                .required(false)
        )
        .get_matches()
}
