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
    metadata_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    customer_master_file: String,
    customer_master_metadata_file: String,
    required_fields_file_path: String,
    config_file_path: String,
    field_delimiter: String,
    balm_rule_file_path: String,
    balm_default_llg: i32,
    acc_currency: String,
    base_currency: String,
    exchange_rate_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(
            logger,
            "customer_master_file: {}",
            self.customer_master_file()
        );
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(
            logger,
            "customer_master_metadata_file: {}",
            self.customer_master_metadata_file()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
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
        let customer_master_file = matches
            .value_of("customer_master_file")
            .expect("Error getting `customer_master_file`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();

        let customer_master_metadata_file = matches
            .value_of("customer_master_metadata_file")
            .expect("Error getting `customer_master_metadata_file`.")
            .to_string();
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
            config_file_path,
            customer_master_metadata_file,
            as_on_date,
            customer_master_file,
            metadata_file_path,
            required_fields_file_path,
            field_delimiter,
            balm_rule_file_path,
            balm_default_llg,
            acc_currency,
            base_currency,
            exchange_rate_file,
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
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn customer_master_file(&self) -> &str {
        &self.customer_master_file
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
    pub fn customer_master_metadata_file(&self) -> &str {
        &self.customer_master_metadata_file
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
        .about("This is llg level analysis program!!")
        .version("1.1.4923")
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
            Arg::with_name("config_file_path")
                .long("config-file-path")
                .value_name("Config file path")
                .help("Path to get config file.")
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
            Arg::with_name("customer_master_file")
                .long("customer-master-file")
                .value_name("Customer master file")
                .help("Cust Master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("customer_master_metadata_file")
                .long("customer-master-metadata-file")
                .value_name("Customer master metadata file")
                .help("Customer master metadata file path.")
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
