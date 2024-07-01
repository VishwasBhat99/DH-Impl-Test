use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);

    let parameters = ConfigurationParameters::new_from_matches(matches);
    return parameters;
}

pub struct ConfigurationParameters {
    input_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    base_currency: String,
    exchange_rate_file: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_consolidated: bool,
    is_perf_diagnostics_enabled: bool,
    local_ccy: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
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
            "is_perf_diagnostics_enabled: {}",
            self.is_perf_diagnostics_enabled()
        );
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "local_ccy: {}", self.local_ccy());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        // TODO: `unwrap()`s need proper error messages.
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
        let exchange_rate_file = matches
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
        let rules_file_path = matches.value_of("rules_file_path").unwrap().to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base currency`.")
            .to_string();
            let local_ccy = matches
            .value_of("local_ccy")
            .expect("Error getting `local currency`.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            base_currency,
            exchange_rate_file,
            req_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_consolidated,
            is_perf_diagnostics_enabled,
            local_ccy,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        return &self.input_file_path;
    }
    pub fn output_file_path(&self) -> &str {
        return &self.output_file_path;
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        return &self.as_on_date;
    }
    pub fn exchange_rate_file(&self) -> &str {
        return &self.exchange_rate_file;
    }
    pub fn log_file_path(&self) -> &str {
        return &self.log_file_path;
    }
    pub fn req_fields_file_path(&self) -> &str {
        return &self.req_fields_file_path;
    }
    pub fn account_metadata_file_path(&self) -> &str {
        return &self.account_metadata_file_path;
    }
    pub fn rules_file_path(&self) -> &str {
        return &self.rules_file_path;
    }
    pub fn diagnostics_file_path(&self) -> &str {
        return &self.diagnostics_file_path;
    }
    pub fn log_level(&self) -> &str {
        return &self.log_level;
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        return self.is_perf_diagnostics_enabled;
    }
    pub fn is_consolidated(&self) -> bool {
        return self.is_consolidated;
    }
    pub fn default_llg_code(&self) -> i32 {
        return self.default_llg_code;
    }
    pub fn base_currency(&self) -> &str {
        return &self.base_currency;
    }
    pub fn local_ccy(&self) -> &str {
        return &self.local_ccy;
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app helps to aggregate cashflows for non maturity  programs!")
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
                .help("Path to write diagnostics log to")
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
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("CONSOLIDATION FLAG")
                .possible_values(&["true", "false"])
                .help("This flag decides whether currency is consolidated or not.")
                .default_value("true")
                .required(true)
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
                .long("base-ccy")
                .value_name("BASE CURRENCY")
                .help("The base currency.")
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
            Arg::with_name("local_ccy")
                .long("local-ccy")
                .value_name("LOCAL CURRENCY")
                .help("This is the local currency. Default RUP")
                .default_value("RUP")
                .required(false)
        )
        .get_matches()
}
