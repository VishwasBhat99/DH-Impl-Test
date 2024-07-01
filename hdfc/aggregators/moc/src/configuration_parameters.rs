use chrono::Local;
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
    log_file_path: String,
    known_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
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
            "known_fields_file_path: {}",
            self.known_fields_file_path()
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
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error while getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error while getting `output_file_path`.")
            .to_string();
        let timestamp = Local::now()
            .naive_local()
            .format("%d%m%Y_%H%M%S")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        log_file_path = log_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let mut diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        diagnostics_file_path = diagnostics_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let known_fields_file_path = matches
            .value_of("known_fields_file")
            .expect("Error while getting `known_fields_file`.")
            .to_string();

        let consolidated_currency = matches
            .value_of("consolidated_currency")
            .expect("Error while getting `consolidator_currency`.")
            .to_string();
        let local_consolidation_currency = matches
            .value_of("local_consolidation_currency")
            .expect("Error while getting `local_consolidation_currency`.")
            .to_string();
        let foreign_consolidation_currency = matches
            .value_of("foreign_consolidation_currency")
            .expect("Error while getting `foreign_consolidation_currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error while getting `exchange_rate_file`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error while getting `account_metadata_file_path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error while getting `rules_file_path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error while getting `default_llg_code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default_llg_code` as integer.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Error while parsing `perf_diag_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            consolidated_currency,
            local_consolidation_currency,
            foreign_consolidation_currency,
            currency_conversion_file_path,
            log_file_path,
            known_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            diagnostics_file_path,
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
    pub fn known_fields_file_path(&self) -> &str {
        &self.known_fields_file_path
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .short("e")     // 'l', 'o' taken; 'g' doesn't make sense; 'v' stands for 'verbose'
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .short("E")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("consolidated_currency")
                .short("c")
                .long("currency")
                .value_name("CURRENCY")
                .help("The consolidated currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("local_consolidation_currency")
                .short("C")
                .long("local-consolidation-currency")
                .value_name("LOCAL CONSOLIDATION CURRENCY")
                .help("If the currency is INR we use this as consolidation currency")
                .required(true)
        )
        .arg(
            Arg::with_name("foreign_consolidation_currency")
                .short("F")
                .long("foreign-consolidation-currency")
                .value_name("FOREIGN CONSOLIDATION CURRENCY")
                .help("If the currency is a foreign currency we use this as consolidation currency along with consolidated currency")
                .required(true)
        )
        .arg(
            Arg::with_name("known_fields_file")
                .short("k")
                .long("known-fields-file")
                .value_name("KNOWN_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .short("m")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .short("r")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .short("d")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .get_matches()
}