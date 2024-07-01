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
    src_local_ccy: String,
    display_local_ccy: String,
    consol_ccy: String,
    currency_conversion_file_path: String,
    log_file_path: String,
    known_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    diagnostics_file_path: String,
    log_level: String,
    dim_id: String,
    is_perf_diagnostics_enabled: bool,
    is_consolidated: bool,
    neg_cf_type: String,
    is_neg: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "src_local_ccy: {}", self.src_local_ccy());
        info!(logger, "display_local_ccy: {}", self.display_local_ccy());
        info!(logger, "consol_ccy: {}", self.consol_ccy());
        info!(logger, "dim_id: {}", self.dim_id());
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
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "neg_cf_type: {}", self.neg_cf_type());
        info!(logger, "is_neg: {}", self.is_neg());
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
        let known_fields_file_path = matches
            .value_of("known_fields_file")
            .expect("Error while getting required fields file path.")
            .to_string();

        let src_local_ccy = matches
            .value_of("src_local_ccy")
            .expect("Error while getting `src_local_ccy`.")
            .to_string();
        let display_local_ccy = matches
            .value_of("display_local_ccy")
            .expect("Error while getting `display_local_ccy`.")
            .to_string();
        let consol_ccy = matches
            .value_of("consol_ccy")
            .expect("Error while getting `consol ccy`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error while getting Exchange Rate file path.")
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
        let dim_id = matches
            .value_of("dim_id")
            .expect("Error while getting `dim_id`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting `is perfect diagnostics`.")
            .parse::<bool>()
            .expect("Error while parsing `is_perf_diagnostics_enabled` as bool.");
        let is_consolidated = matches
            .value_of("is_consolidated")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let neg_cf_type = matches
            .value_of("neg_cf_type")
            .expect("Error while getting `neg_cf_type`.")
            .to_string();
        let is_neg = matches
            .value_of("is_neg")
            .unwrap()
            .parse::<bool>()
            .expect("Could not read is negetive flag from configuration");
        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            src_local_ccy,
            display_local_ccy,
            consol_ccy,
            currency_conversion_file_path,
            log_file_path,
            known_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            diagnostics_file_path,
            log_level,
            dim_id,
            is_perf_diagnostics_enabled,
            is_consolidated,
            neg_cf_type,
            is_neg,
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
    pub fn src_local_ccy(&self) -> &str {
        &self.src_local_ccy
    }
    pub fn display_local_ccy(&self) -> &str {
        &self.display_local_ccy
    }
    pub fn consol_ccy(&self) -> &str {
        &self.consol_ccy
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
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
    pub fn neg_cf_type(&self) -> &str {
        &self.neg_cf_type
    }
    pub fn is_neg(&self) -> bool {
        self.is_neg
    }
    pub fn dim_id(&self) -> &str {
        &self.dim_id
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Non Maturity GL Aggregator!")
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
                .help("Path to the output file")
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
            Arg::with_name("neg_cf_type")
                .long("neg-cf-type")
                .value_name("neg_cf_type")
                .possible_values(&["I", "O"])
                .help("This flag that decides whether the amount is absolute or not (default `O`).")
                .default_value("O")
                .required(false)
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
            Arg::with_name("dim_id")
                .long("dim-id")
                .value_name("Dimentional Id")
                .help("Dimentional Id")
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
            Arg::with_name("src_local_ccy")
                .long("src-local-ccy")
                .value_name("Source Local Currency")
                .help("The src local currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("display_local_ccy")
                .long("display-local-ccy")
                .value_name("Display Local Currency")
                .help("The local display currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("consol_ccy")
                .long("consol-ccy")
                .value_name("Consolidated Currency")
                .help("The Consol currency")
                .required(true)
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
            Arg::with_name("is_neg")
                .long("is-neg")
                .value_name("is_neg")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether the currency is consolidated or not.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("known_fields_file")
                .long("known-fields-file")
                .value_name("Required Fields")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
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
