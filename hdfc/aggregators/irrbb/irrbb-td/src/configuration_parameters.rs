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
    base_ccy: String,
    exchange_rate_file: String,
    log_file_path: String,
    output_fields_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    consol_ccy: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "base_ccy: {}", self.base_ccy());
        info!(logger, "consol_ccy: {}", self.consol_ccy());
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(
            logger,
            "output_fields_file_path: {}",
            self.output_fields_file_path()
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
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error while getting `input file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error while getting `output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error while getting `log file path.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting as on date as DD-MM-YYYY."),
        );
        let output_fields_file_path = matches
            .value_of("output_fields_file")
            .expect("Error while getting output fields file path.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error while getting required fields file path.")
            .to_string();
        let base_ccy = matches
            .value_of("base_ccy")
            .expect("Error while getting `base currency`.")
            .to_string();
        let consol_ccy = matches
            .value_of("consol_ccy")
            .expect("Error while getting `consol currency`.")
            .to_string();

        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error while getting `exchange_rate_file`.")
            .to_string();

        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error while getting `diagnostics file path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error while getting `account metadata file`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error while getting `rules file path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error while getting `default llg code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default llg code` as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting `is perfect diagnostics`.")
            .parse::<bool>()
            .expect("Error while parsing `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            base_ccy,
            exchange_rate_file,
            log_file_path,
            output_fields_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            consol_ccy,
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
    pub fn base_ccy(&self) -> &str {
        &self.base_ccy
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn output_fields_file_path(&self) -> &str {
        &self.output_fields_file_path
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
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn consol_ccy(&self) -> &str {
        &self.consol_ccy
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("IRRBB TD!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On Date")
                .help("The date for which the program has to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
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
            Arg::with_name("base_ccy")
                .long("base-ccy")
                .value_name("Base Currency")
                .help("The base currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("consol_ccy")
                .long("consol-ccy")
                .value_name("Consol Currency")
                .help("The consol currency.")
                .default_value("RUP")
                .required(false)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("Exchange Rate File Path")
                .help("Path to exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_fields_file")
                .long("output-fields-file")
                .value_name("Output Fields")
                .help("All fields required to process the file.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("Required Fields")
                .help("All fields required to process the file.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file")
                .value_name("Account Metadata File")
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
        .get_matches()
}
