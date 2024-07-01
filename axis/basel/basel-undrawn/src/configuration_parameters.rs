use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    rtl_aggr_input_file_path: String,
    nrtl_aggr_input_file_path: String,
    output_file_path: String,
    lcr_undrawn_file_path: String,
    log_file_path: String,
    exchange_rate_file_path: String,
    base_currency: String,
    country: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_consolidated: bool,
    as_on_date: NaiveDate,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "rtl_aggr_input_file_path: {}", self.rtl_aggr_input_file_path
        );
        info!(
            logger,
            "nrtl_aggr_input_file_path: {}", self.nrtl_aggr_input_file_path
        );
        info!(logger, "output_file_path: {}", self.output_file_path);
        info!(
            logger,
            "lcr_undrawn_file_path: {}", self.lcr_undrawn_file_path
        );
        info!(logger, "log_file: {}", self.log_file_path);
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path);
        info!(logger, "log_level: {}", self.log_level);
        info!(
            logger,
            "diagnostics_flag: {}", self.is_perf_diagnostics_enabled
        );
        info!(logger, "is_consolidated: {}", self.is_consolidated);
        info!(
            logger,
            "exchange_rate_file_path: {}", self.exchange_rate_file_path
        );
        info!(logger, "base_currency: {}", self.base_currency);
        info!(logger, "country: {}", self.country);
        info!(logger, "as_on_date: {}", self.as_on_date);
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let rtl_aggr_input_file_path = matches
            .value_of("rtl_aggr_input_file_path")
            .expect("Error getting `rtl_aggr_input_file_path`.")
            .to_string();
        let nrtl_aggr_input_file_path = matches
            .value_of("nrtl_aggr_input_file_path")
            .expect("Error getting `nrtl_aggr_input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let lcr_undrawn_file_path = matches
            .value_of("lcr_undrawn_file_path")
            .expect("Error getting `lcr_undrawn_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error while getting `as_on_date`."),
        );
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file_path")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let country = matches
            .value_of("country")
            .expect("Error getting `country`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");
        ConfigurationParameters {
            rtl_aggr_input_file_path,
            nrtl_aggr_input_file_path,
            output_file_path,
            lcr_undrawn_file_path,
            as_on_date,
            log_file_path,
            country,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            base_currency,
            is_consolidated,
            exchange_rate_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn rtl_aggr_input_file_path(&self) -> &str {
        &self.rtl_aggr_input_file_path
    }
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
    }
    pub fn nrtl_aggr_input_file_path(&self) -> &str {
        &self.nrtl_aggr_input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn lcr_undrawn_file_path(&self) -> &str {
        &self.lcr_undrawn_file_path
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
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("UNDRAWN BIFURCATION")
        .version("1.3.3918")
        .author("Saurabh Singh <saurabh.s@surya-soft.com>")
        .arg(
            Arg::with_name("rtl_aggr_input_file_path")
                .long("rtl-aggr-input-file")
                .value_name("Retail Aggregator Input File")
                .help("Path to the retail aggregator input file.")
                .required(true),
        )
        .arg(
            Arg::with_name("nrtl_aggr_input_file_path")
                .long("nrtl-aggr-input-file")
                .value_name(" Non Retail Aggregator Input File")
                .help("Path to the non retail aggregator input file.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("lcr_undrawn_file_path")
                .long("lcr-undrawn-file")
                .value_name("Balm Rule File")
                .help("Path to the BALM rule file.")
                .required(true),
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
            Arg::with_name("exchange_rate_file_path")
                .long("exchange-rate-file-path")
                .value_name("Exchange Rate File")
                .help("Path to the Exchange Rate File.")
                .required(true),
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("CONSOLIDATION FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("Country")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
