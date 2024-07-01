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
    config_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    granularity_weight_file: String,
    is_granularity_perc: bool,
    compare_condition: String,
    total_out_bal: f64,
    base_currency: String,
    currency_conversion_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "granularity_weight_file: {}",
            self.granularity_weight_file()
        );
        info!(logger, "total_out_bal: {}", self.total_out_bal());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "is_granularity_perc: {}",
            self.is_granularity_perc()
        );
        info!(logger, "compare_condition: {}", self.compare_condition());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
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
        let granularity_weight_file = matches
            .value_of("granularity_weight_file")
            .expect("Error getting `granularity_weight_file`.")
            .to_string();
        let total_out_bal = matches
            .value_of("total_out_bal")
            .expect("Error getting `total_out_bal`.")
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let is_granularity_perc = matches
            .value_of("is_granularity_perc")
            .expect("Error getting `is_granularity_perc`.")
            .parse::<bool>()
            .expect("Cannot parse `is_granularity_perc` as bool.");
        let compare_condition = matches
            .value_of("compare_condition")
            .expect("Error getting `compare_condition`.")
            .to_string();

        ConfigurationParameters {
            config_file_path,
            output_file_path,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            granularity_weight_file,
            total_out_bal,
            base_currency,
            currency_conversion_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_granularity_perc,
            compare_condition,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn granularity_weight_file(&self) -> &str {
        &self.granularity_weight_file
    }
    pub fn total_out_bal(&self) -> f64 {
        self.total_out_bal
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn compare_condition(&self) -> &str {
        &self.compare_condition
    }
    pub fn is_granularity_perc(&self) -> bool {
        self.is_granularity_perc
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This Program Calculates the Total Exposure of each Customer")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .version("1.4.4532")
        .arg(
            Arg::new("config_file_path")
                .long("config-file-path")
                .value_name("CONFIG_FILE")
                .help("Path to the config file.")
                .required(true)
        )
        .arg(
            Arg::new("output_file_path")
                .long("output-file-path")
                .value_name("OUTPUT_FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("ASONDATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("LOG_FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAGLOG_FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG_LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS_FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE_RATE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::new("base_currency")
                .long("base-currency")
                .value_name("BASE_CURRENCY")
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::new("granularity_weight_file")
                .long("granularity-weight-file")
                .value_name("GRANULARITY_WEIGHT_FILE")
                .help("File containing Granularity Percentage/Amount to be applied on Total Outstanding Balance.")
                .required(true)
        )
        .arg(
            Arg::new("total_out_bal")
                .long("total-out-bal")
                .value_name("TOTAL_OUT_BAL")
                .help("Total Outstanding Balance.")
                .default_value("0.0")
                .required(false)
        )
        .arg(
            Arg::new("is_granularity_perc")
                .long("is-granularity-perc")
                .value_name("IS_GRANULARITY_PERC")
                .possible_values(["true", "false"])
                .help("This flag that decides whether granularity will be given in Percentage or Amount.")
                .default_value("false")
                .required(true)
        )
        .arg(
            Arg::new("compare_condition")
                .long("compare-condition")
                .value_name("COMPARE_CONDITION")
                .help("Amount to be considered for comparison with Exposure-Amt (Either Max or Min of TTL_BAL/TTL_LIM_BAL OR TTL_BAL/TTL_LIM_BAL Directly).")
                .possible_values(["MAX","MIN","TTL_BAL","TTL_LIM_BAL"])
                .default_value("TTL_BAL")
                .required(false)
        )
        .get_matches()
}
