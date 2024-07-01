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
    as_on_mandatory: bool,
    metadata_file_path: String,
    required_fields_file_path: String,
    field_delimiter: String,
    balm_rule_file_path: String,
    balm_default_llg: i32,
    acc_currency: String,
    base_currency: String,
    exchange_rate_file: String,
    default_overdue_llg_code: i32,
    req_overdue: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "as_on_mandatory: {}", self.as_on_mandatory());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(
            logger,
            "required_fields_file_path: {}",
            self.required_fields_file_path()
        );
        info!(logger, "field_delimiter: {}", self.field_delimiter());
        info!(
            logger,
            "balm_rule_file_path: {}",
            self.balm_rule_file_path()
        );
        info!(logger, "balm_default_llg: {}", self.balm_default_llg());
        info!(logger, "acc_currency: {}", self.acc_currency());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(
            logger,
            "default_overdue_llg_code: {}",
            self.default_overdue_llg_code()
        );
        info!(logger, "req_overdue: {}", self.req_overdue());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let as_on_mandatory = matches
            .value_of("as_on_mandatory")
            .expect("Error getting `as_on_mandatory`.")
            .parse::<bool>()
            .expect("Cannot parse `as_on_mandatory` as bool.");
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let required_fields_file_path = matches
            .value_of("required_fields_file_path")
            .expect("Error getting `required_fields_file_path`.")
            .to_string();
        let field_delimiter = matches
            .value_of("field_delimiter")
            .expect("Error getting `field_delimiter`.")
            .to_string();
        let balm_rule_file_path = matches
            .value_of("balm_rule_file_path")
            .expect("Error getting `balm_rule_file_path`.")
            .to_string();
        let balm_default_llg = matches
            .value_of("balm_default_llg")
            .expect("Error getting `balm_default_llg`.")
            .parse()
            .unwrap_or(0);
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        let default_overdue_llg_code = matches
            .value_of("default_overdue_llg_code")
            .unwrap_or("0")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default overdue llg code` as integer.");
        let req_overdue = matches
            .value_of("req_overdue")
            .expect("Error getting `req_overdue`.")
            .parse::<bool>()
            .expect("Cannot parse `req_overdue` as bool.");
        let acc_currency = matches
            .value_of("acc_currency")
            .expect("Error getting `acc_currency`.")
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
            input_file_path,
            output_file_path,
            as_on_date,
            as_on_mandatory,
            metadata_file_path,
            required_fields_file_path,
            field_delimiter,
            balm_rule_file_path,
            balm_default_llg,
            acc_currency,
            base_currency,
            exchange_rate_file,
            default_overdue_llg_code,
            req_overdue,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn as_on_mandatory(&self) -> bool {
        self.as_on_mandatory
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn required_fields_file_path(&self) -> &str {
        &self.required_fields_file_path
    }
    pub fn field_delimiter(&self) -> &str {
        &self.field_delimiter
    }
    pub fn balm_rule_file_path(&self) -> &str {
        &self.balm_rule_file_path
    }
    pub fn balm_default_llg(&self) -> i32 {
        self.balm_default_llg
    }
    pub fn acc_currency(&self) -> &String {
        &self.acc_currency
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn default_overdue_llg_code(&self) -> i32 {
        self.default_overdue_llg_code
    }
    pub fn req_overdue(&self) -> bool {
        self.req_overdue
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
        .about("Converts fields from .cf file to text file!!")
        .version("2.2.4100")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to output file.")
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
            Arg::with_name("as_on_mandatory")
                .long("as-on-mandatory")
                .value_name("As On Mandatory")
                .help("As On Mandatory.")
                .default_value("false")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file_path")
                .long("metadata-file-path")
                .value_name("Metadata File Path")
                .help("Path to metadata file.")
                .required(true)
        )
        .arg(
            Arg::with_name("required_fields_file_path")
                .long("required-fields-file-path")
                .value_name("Req Fields File Path")
                .help("Path to req fields file.")
                .required(true)
        )
        .arg(
            Arg::with_name("field_delimiter")
                .long("field-delimiter")
                .value_name("Field Delimiter")
                .help("Field separator in output.")
                .default_value("|")
                .required(true)
        )
        .arg(
            Arg::with_name("balm_rule_file_path")
                .long("balm-rule-file-path")
                .value_name("BALM Rule File Path")
                .help("Path to balm rule file.")
                .required(true)
        )
        .arg(
            Arg::with_name("balm_default_llg")
                .long("balm-default-llg")
                .value_name("BALM Default LLG")
                .help("Path to BALM Default LLG.")
                .default_value("1999")
                .required(true)
        )
        .arg(
            Arg::with_name("acc_currency")
                .long("acc-currency")
                .value_name("Account Currency")
                .help("Name of cuurency field.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("Instance local Currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("Exchange Rate File Path")
                .help("Path to exchange rate file.")
                .required(true)
        ) .arg(
            Arg::with_name("req_overdue")
                .long("req-overdue")
                .value_name("Req Overdue")
                .help("This decides whether Overdue LLG should be applied or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("default_overdue_llg_code")
                .long("default-overdue-llg")
                .value_name("Default Overdue LLG")
                .help("Value of Default Overdue LLG.")
                .required(false)
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
