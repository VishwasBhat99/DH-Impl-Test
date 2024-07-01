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
    log_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    rpt_id: String,
    incr_date: NaiveDate,
    threshold_bal: f64,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_consolidated: bool,
    home_currency: String,
    exchange_rate_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
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
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "rpt_id: {}", self.rpt_id());
        info!(logger, "incr_date: {}", self.incr_date());
        info!(logger, "threshold_bal: {}", self.threshold_bal());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "home_currency: {}", self.home_currency());
        info!(
            logger,
            "exchange_rate_file_path: {}",
            self.exchange_rate_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
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
        let rpt_id = matches
            .value_of("rpt_id")
            .expect("Error getting `rpt_id`.")
            .to_string();
        let home_currency = matches
            .value_of("home_currency")
            .expect("Error getting `home_currency`.")
            .to_string();
        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file_path")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let incr_date = date_parser.parse(
            matches
                .value_of("incr_date")
                .expect("Error getting `incr_date`."),
        );
        let threshold_bal = matches
            .value_of("threshold_bal")
            .expect("Error getting `threshold_bal`.")
            .parse::<f64>()
            .expect("Error parsing threshold_bal as f64");
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `required_fields_file`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `account_metadata_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `is_perf_diagnostics_enabled` flag.")
            .parse::<bool>()
            .expect("Error while parsing `is_perf_diagnostics_enabled` as bool.");
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated` flag.")
            .parse::<bool>()
            .expect("Error while parsing `is_consolidated` as bool.");
        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            log_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            rpt_id,
            incr_date,
            threshold_bal,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_consolidated,
            home_currency,
            exchange_rate_file_path,
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
    pub fn rpt_id(&self) -> &str {
        &self.rpt_id
    }
    pub fn incr_date(&self) -> &NaiveDate {
        &self.incr_date
    }
    pub fn threshold_bal(&self) -> &f64 {
        &self.threshold_bal
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn home_currency(&self) -> &str {
        &self.home_currency
    }
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
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
}

pub fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("mis_incr_td")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rpt_id")
                .long("rpt-id")
                .value_name("Reprot ID")
                .help("Path to the report id.")
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
            Arg::with_name("home_currency")
                .long("home-currency")
                .value_name("Home Currncy")
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
            Arg::with_name("incr_date")
                .long("incr-date")
                .value_name("INCR DATE")
                .help("INCREMENTAL DATE.")
                .required(true)
        )
        .arg(
            Arg::with_name("threshold_bal")
                .long("threshold-bal")
                .value_name("THRESHOLD BAL")
                .help("Max Balance")
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
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether is consolidated flag is ON/OFF")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("REQUIRED FIELDS FILE")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe req_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .get_matches()
}
