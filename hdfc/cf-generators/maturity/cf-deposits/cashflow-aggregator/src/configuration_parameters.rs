use clap;
use clap::{Arg, App};
use slog::Logger;
use rbdate::NaiveDate;
use rbdate::DateParser;

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
    currency_conversion_file_path: String,
    log_file_path: String,
    known_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "currency_conversion_file_path: {}", self.currency_conversion_file_path());
        info!(logger, "known_fields_file_path: {}", self.known_fields_file_path());
        info!(logger, "account_metadata_file_path: {}", self.account_metadata_file_path());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        // TODO: `unwrap()`s need proper error messages.
        let input_file_path = matches.value_of("input_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();

        let date_parser = DateParser::new(
            "%Y-%m-%d".to_string(),
            true
        );
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
                matches.value_of("as_on_date")
                        .unwrap_or("this string will convert to today")
                );
        let known_fields_file_path = matches.value_of("known_fields_file").unwrap().to_string();

        let base_currency = matches.value_of("base_currency").unwrap().to_string();
        let currency_conversion_file_path = matches.value_of("exchange_rate_file").unwrap().to_string();
        let diagnostics_file_path = matches.value_of("diagnostics_log_file").unwrap().to_string();
        let account_metadata_file_path = matches.value_of("account_metadata_file_path").unwrap().to_string();
        let rules_file_path = matches.value_of("rules_file_path").unwrap().to_string();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches.value_of("perf_diag_flag").unwrap().parse::<bool>().unwrap();



        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            base_currency,
            currency_conversion_file_path,
            log_file_path,
            known_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled
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
    pub fn base_currency(&self) -> &str {
        return &self.base_currency;
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        return &self.currency_conversion_file_path;
    }
    pub fn log_file_path(&self) -> &str {
        return &self.log_file_path;
    }
    pub fn known_fields_file_path(&self) -> &str {
        return &self.known_fields_file_path;
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
                .short("e")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .short("c")
                .long("currency")
                .value_name("CURRENCY")
                .help("The base currency.")
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
        .get_matches()
}