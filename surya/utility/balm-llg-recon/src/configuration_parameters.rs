use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    output_file_path: String,
    metadata_file_path: String,
    required_fields_file_path: String,
    field_delimiter: String,
    balm_rule_file_path: String,
    balm_default_llg: i32,
    acc_currency: String,
    base_currency: String,
    exchange_rate_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    output_option: String,
    llg_ids: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
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
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "output_option: {}", self.output_option());
        info!(logger, "llg_ids: {}", self.llg_ids());
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
        let output_option = matches
            .value_of("output_option")
            .expect("Error getting `output_option`.")
            .to_string();
        let llg_ids = matches
            .value_of("llg_ids")
            .expect("Error getting `llg_ids`.")
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
            output_option,
            llg_ids,
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
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn output_option(&self) -> &str {
        &self.output_option
    }
    pub fn llg_ids(&self) -> &str {
        &self.llg_ids
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
            Arg::with_name("output_option")
                .long("output-option")
                .value_name("Output Option")
                .help("Output Option Value.")
                .default_value("ALL")
                .required(false)
        )
        .arg(
            Arg::with_name("llg_ids")
                .long("llg-ids")
                .value_name("LLG IDs Values")
                .help("List of LLG IDs")
                .default_value(" ")
                .required(false)
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
