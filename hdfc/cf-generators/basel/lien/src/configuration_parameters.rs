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
    input_sheet_name: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    input_currency: String,
    base_currency: String,
    country: String,
    currency_conversion_file_path: String,
    cust_type_ref_path: String,
    act_ccy_col_id: usize,
    cust_typ_col_id: usize,
    exp_date_col_id: usize,
    os_col_id: usize,
    fd_amt_col_id: usize,
    acc_id_col_id: usize,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    residual_maturity_days: i64,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "input_sheet_name: {}", self.input_sheet_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "country: {}", self.country());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "input_currency: {}", self.input_currency());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "cust_type_ref_path: {}", self.cust_type_ref_path());
        info!(logger, "act_ccy_col_id: {}", self.act_ccy_col_id());
        info!(logger, "cust_typ_col_id: {}", self.cust_typ_col_id());
        info!(logger, "exp_date_col_id: {}", self.exp_date_col_id());
        info!(logger, "os_col_id: {}", self.os_col_id());
        info!(logger, "fd_amt_col_id: {}", self.fd_amt_col_id());
        info!(logger, "acc_id_col_id: {}", self.acc_id_col_id());
        info!(logger, "residual_maturity_days: {}", self.residual_maturity_days());
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
        let input_sheet_name = matches
            .value_of("input_sheet_name")
            .expect("Error getting `input_sheet_name`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let cust_type_ref_path = matches
            .value_of("cust_type_ref_path")
            .expect("Error getting `cust_type_ref_path`.")
            .to_string();
        let country = matches
            .value_of("country")
            .expect("Error getting `country`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );

        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let input_currency = matches
            .value_of("input_currency")
            .expect("Error getting `input_currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file_path`.")
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
        let act_ccy_col_id = matches
            .value_of("act_ccy_col_id")
            .expect("Error getting `act_ccy_col_id`.")
            .parse::<usize>()
            .expect("Cannot parse `act_ccy_col_id` as usize.");
        let cust_typ_col_id = matches
            .value_of("cust_typ_col_id")
            .expect("Error getting `cust_typ_col_id`.")
            .parse::<usize>()
            .expect("Cannot parse `cust_typ_col_id` as usize.");
        let exp_date_col_id = matches
            .value_of("exp_date_col_id")
            .expect("Error getting `exp_date_col_id`.")
            .parse::<usize>()
            .expect("Cannot parse `exp_date_col_id` as usize.");
        let os_col_id = matches
            .value_of("os_col_id")
            .expect("Error getting `os_col_id`.")
            .parse::<usize>()
            .expect("Cannot parse `os_col_id` as usize.");
        let acc_id_col_id = matches
            .value_of("acc_id_col_id")
            .expect("Error getting `acc_id_col_id`.")
            .parse::<usize>()
            .expect("Cannot parse `acc_id_col_id` as usize.");
        let fd_amt_col_id = matches
            .value_of("fd_amt_col_id")
            .expect("Error getting `fd_amt_col_id`.")
            .parse::<usize>()
            .expect("Cannot parse `fd_amt_col_id` as usize.");
        let residual_maturity_days = matches
            .value_of("residual_maturity_days")
            .expect("Error getting `residual_maturity_days`.")
            .parse::<i64>()
            .expect("Cannot parse `residual_maturity_days` as i64.");
        ConfigurationParameters {
            input_file_path,
            input_sheet_name,
            output_file_path,
            as_on_date,
            input_currency,
            base_currency,
            country,
            currency_conversion_file_path,
            cust_type_ref_path,
            act_ccy_col_id,
            cust_typ_col_id,
            exp_date_col_id,
            os_col_id,
            fd_amt_col_id,
            acc_id_col_id,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            residual_maturity_days,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn input_sheet_name(&self) -> &str {
        &self.input_sheet_name
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
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
    pub fn cust_type_ref_path(&self) -> &str {
        &self.cust_type_ref_path
    }
    pub fn input_currency(&self) -> &str {
        &self.input_currency
    }
    pub fn act_ccy_col_id(&self) -> &usize {
        &self.act_ccy_col_id
    }
    pub fn cust_typ_col_id(&self) -> &usize {
        &self.cust_typ_col_id
    }
    pub fn exp_date_col_id(&self) -> &usize {
        &self.exp_date_col_id
    }
    pub fn os_col_id(&self) -> &usize {
        &self.os_col_id
    }
    pub fn fd_amt_col_id(&self) -> &usize {
        &self.fd_amt_col_id
    }
    pub fn acc_id_col_id(&self) -> &usize {
        &self.acc_id_col_id
    }
    pub fn residual_maturity_days(&self) -> &i64 {
        &self.residual_maturity_days
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Lien Files Program")
        .version("1.0.4103")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_type_ref_path")
                .long("cust-type-ref-path")
                .value_name("Cust Type Ref Path")
                .help("Path to the Cust Type.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sheet_name")
                .long("input-sheet-name")
                .value_name("Input Sheet Name")
                .help("Input file sheet name.")
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
            Arg::with_name("country")
                .long("country")
                .value_name("Country")
                .help("Country instance name.")
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
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
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
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_currency")
                .long("input-currency")
                .value_name("INPUT CURRENCY")
                .help("The INPUT currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("act_ccy_col_id")
                .long("act-ccy-col-id")
                .value_name("Act CCY column Id")
                .help("The Actual CCY Column ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("acc_id_col_id")
                .long("acc-id-col-id")
                .value_name("Account Id column Id")
                .help("Account ID Column ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_typ_col_id")
                .long("cust-typ-col-id")
                .value_name("Cust type column Id")
                .help("The Cust Type Column ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("exp_date_col_id")
                .long("exp-date-col-id")
                .value_name("Exp Date column Id")
                .help("The Exp Date Column ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("os_col_id")
                .long("os-col-id")
                .value_name("OS column Id")
                .help("The OS Column ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("fd_amt_col_id")
                .long("fd-amt-col-id")
                .value_name("FD Amt column Id")
                .help("The FD Amt Column ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("residual_maturity_days")
                .long("residual-maturity-days")
                .value_name("Residual Maturity in Days")
                .help("The residual days to be compared with tenor.")
                .default_value("30")
                .required(false)
        )
        .get_matches()
}
