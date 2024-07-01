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
    pub as_on_date: NaiveDate,
    input_file_path: String,
    output_file_path: String,
    nwd_file_path: String,
    exclude_file_path: String,
    bkt_file_path: String,
    currency_conversion_file_path: String,
    base_currency: String,
    cust_master_file_path: String,
    account_metadata_file_path: String,
    req_fields_file_path: String,
    ret_cust_types: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    check_wd: bool,
    is_consolidated: bool,
    is_acc_cust_type: String,
    residual_maturity_days: i64,
    abs_value_flag: Option<bool>,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "ret_cust_types: {:?}", self.ret_cust_types());
        info!(logger, "is_acc_cust_type: {:?}", self.is_acc_cust_type());
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(logger, "base_currency: {}", self.base_currency());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(
            logger,
            "cust_master_file_path: {}",
            self.cust_master_file_path()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "nwd_file_path: {}", self.nwd_file_path());
        info!(logger, "exclude_file_path: {}", self.exclude_file_path());
        info!(logger, "bkt_file_path: {}", self.bkt_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "check_wd: {}", self.check_wd());
        info!(
            logger,
            "residual_maturity_days: {}",
            self.residual_maturity_days()
        );
        info!(logger, "abs_value_flag: {:#?}", self.abs_value_flag());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser
            .parse_opt(
                matches
                    .value_of("as_on_date")
                    .expect("Error getting `as_on_date` value."),
            )
            .expect("Cannot parse `as_on_date` value as `DD-MM-YYYY` format");
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error while getting `input file path`.")
            .to_string();
        let cust_master_file_path = matches
            .value_of("cust_master_file")
            .expect("Error while getting `cust master file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error while getting `output file path`.")
            .to_string();
        let nwd_file_path = matches
            .value_of("nwd_file_path")
            .expect("Error while getting `nwd file path`.")
            .to_string();
        let exclude_file_path = matches
            .value_of("exclude_file_path")
            .expect("Error while getting `exclude file path`.")
            .to_string();
        let bkt_file_path = matches
            .value_of("bkt_file_path")
            .expect("Error while getting `bkt file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error while getting `log file path.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error while getting required fields file path.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error while getting `diagnostics file path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error while getting `account metadata file`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting `is perfect diagnostics`.")
            .parse::<bool>()
            .expect("Error while parsing `is_perf_diagnostics_enabled` as bool.");
        let check_wd = matches
            .value_of("check_wd")
            .expect("Error while getting `check_wd`.")
            .parse::<bool>()
            .expect("Error while parsing `check_wd` as bool.");
        let ret_cust_types = matches
            .value_of("ret_cust_types")
            .expect("Error while parsing `ret_cust_types")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error while getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Error while parsing `is_consolidated` as bool.");
        let is_acc_cust_type = matches
            .value_of("is_acc_cust_type")
            .expect("Error while getting `is_acc_cust_type`.")
            .to_string();
        let residual_maturity_days = matches
            .value_of("residual_maturity_days")
            .expect("Error while getting `residual_maturity_days`.")
            .parse::<i64>()
            .expect("Error while parsing `residual_maturity_days` as bool.");
        let abs_value_flag = matches
            .value_of("abs_value_flag")
            .map(|s| s.parse().ok())
            .flatten();

        ConfigurationParameters {
            as_on_date,
            input_file_path,
            output_file_path,
            nwd_file_path,
            exclude_file_path,
            bkt_file_path,
            currency_conversion_file_path,
            base_currency,
            cust_master_file_path,
            account_metadata_file_path,
            req_fields_file_path,
            ret_cust_types,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            check_wd,
            is_consolidated,
            is_acc_cust_type,
            residual_maturity_days,
            abs_value_flag,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn cust_master_file_path(&self) -> &str {
        &self.cust_master_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn nwd_file_path(&self) -> &str {
        &self.nwd_file_path
    }
    pub fn exclude_file_path(&self) -> &str {
        &self.exclude_file_path
    }
    pub fn bkt_file_path(&self) -> &str {
        &self.bkt_file_path
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
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
    pub fn check_wd(&self) -> bool {
        self.check_wd
    }
    pub fn ret_cust_types(&self) -> Vec<&str> {
        self.ret_cust_types.split(',').collect()
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn is_acc_cust_type(&self) -> &str {
        &self.is_acc_cust_type
    }
    pub fn residual_maturity_days(&self) -> &i64 {
        &self.residual_maturity_days
    }
    pub fn abs_value_flag(&self) -> Option<bool> {
        self.abs_value_flag
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("cust-classify-1")
        .version("1.1.5388")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
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
            Arg::with_name("nwd_file_path")
                .long("nwd-file")
                .value_name("NWD File Path")
                .help("Path to the NWD file")
                .required(true)
        )
        .arg(
            Arg::with_name("exclude_file_path")
                .long("exclude-file")
                .value_name("EXCLUDE File Path")
                .help("Path to the EXCLUDE file")
                .required(true)
        )
        .arg(
            Arg::with_name("bkt_file_path")
                .long("bkt-file")
                .value_name("BKT Schema File Path")
                .help("Path to the Bucket schema file")
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
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("BASE CURRENCY")
                .help("The base currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_master_file")
                .long("cust-master-file")
                .value_name("Customer File")
                .help("Path to customer master file.")
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
                .default_value("none")
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
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
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
            Arg::with_name("ret_cust_types")
                .long("ret-cust-types")
                .value_name("Retail Customer Types")
                .help("These types determines if an account is Retail or Not.")
                .required(true)
        )
        .arg(
            Arg::with_name("check_wd")
                .long("check-wd")
                .value_name("CHECK WD")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to check for wd or not.")
                .default_value("false")
                .required(true)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to check for wd or not.")
                .default_value("false")
                .required(true)
        )
        .arg(
            Arg::with_name("is_acc_cust_type")
                .long("is-acc-cust-type")
                .value_name("IS ACC CUST TYPE")
                .help("This flag that decides whether to derive cust type or use an existing pass through.")
                .default_value("NA")
                .required(true)
        )
        .arg(
            Arg::with_name("residual_maturity_days")
                .long("residual-maturity-days")
                .value_name("Residual maturity days")
                .help("The residual maturity days that are compared with tenor.")
                .default_value("30")
                .required(false)
        )
        .arg(
            Arg::with_name("abs_value_flag")
                .long("abs-value-flag")
                .value_name("IS ABSOLUTE VALUE  FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides weather the outstanding amount should be displayed as absolute value  or not.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
