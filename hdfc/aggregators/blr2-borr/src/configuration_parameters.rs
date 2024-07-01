use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    country_id: String,
    base_ccy: String,
    exchange_rate_file_path: String,
    config_file_path: String,
    cust_code_master: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    delimiter: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "cust_code_master: {}", self.cust_code_master());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "country_id: {}", self.country_id());
        info!(logger, "base_ccy: {}", self.base_ccy());
        info!(logger, "delimiter: {}", self.delimiter());
        info!(
            logger,
            "exchange_rate_file_path: {}",
            self.exchange_rate_file_path()
        );
        info!(logger, "config_file_path: {}", self.config_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let cust_code_master = matches
            .value_of("cust_code_master")
            .expect("Error getting `cust_code_master`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let country_id = matches
            .value_of("country_id")
            .expect("Error getting `country_id`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let base_ccy = matches
            .value_of("base_ccy")
            .expect("Error getting `base_ccy`.")
            .to_string();
        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file_path")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
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
        let delimiter = matches
            .value_of("delimiter")
            .expect("Error getting `delimeter`.")
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
            country_id,
            base_ccy,
            exchange_rate_file_path,
            config_file_path,
            cust_code_master,
            output_file_path,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            delimiter,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn country_id(&self) -> &str {
        &self.country_id
    }
    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }
    pub fn base_ccy(&self) -> &str {
        &self.base_ccy
    }
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn cust_code_master(&self) -> &str {
        &self.cust_code_master
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
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("BLR-2 Borrowings!")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("cust_code_master")
                .long("cust-code-master")
                .value_name("Custumer Code Master File")
                .help("Path to the customer code master file.")
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
                .value_name("DATE")
                .help("The date for which the program has to run.")
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
            Arg::with_name("delimiter")
                .long("delimiter")
                .value_name("DELIMITER")
                .help("Delimiter to be used for cust master file.")
                .default_value("~#~")
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
            Arg::with_name("country_id")
                .long("country-id")
                .value_name("Country ID")
                .help("Unique ID of Country.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_ccy")
                .long("base-ccy")
                .value_name("Base Currncy")
                .help("Local Curency.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file_path")
                .long("exchange-rate-file")
                .value_name("Exchange Rate File Path")
                .help("Path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file")
                .value_name("Config File")
                .help("Path to the config file.")
                .required(true)
        )
        .get_matches()
}
