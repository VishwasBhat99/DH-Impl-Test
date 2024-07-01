use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    biu_master_file_path: String,
    ca_ret_file_path: String,
    sa_ret_file_path: String,
    td_ret_file_path: String,
    rd_ret_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    base_currency: String,
    bkt_file_path: String,
    max_stable_amount: f64,
    is_casa_stable_amount: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_lcy_or_ccy: String,
    lien_file_path: String,
    lien_input_date_format: String,
    residual_maturity_days: i64,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "is_lcy_or_ccy: {}", self.is_lcy_or_ccy());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "bkt_file_path: {}", self.bkt_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "max stable amount: {}", self.max_stable_amount());
        info!(logger, "biu_file: {}", self.biu_master_file_path());
        info!(logger, "ca_ret_file_path: {}", self.ca_ret_file_path());
        info!(logger, "sa_ret_file_path: {}", self.sa_ret_file_path());
        info!(logger, "td_ret_file: {}", self.td_ret_file_path());
        info!(logger, "rd_ret_file: {}", self.rd_ret_file_path());
        info!(
            logger,
            "is_casa_stable_amount: {}",
            self.is_casa_stable_amount()
        );
        info!(logger, "lien_file_path: {}", self.lien_file_path());
        info!(
            logger,
            "lien_input_date_format: {}",
            self.lien_input_date_format()
        );
        info!(
            logger,
            "residual_maturity_days: {}",
            self.residual_maturity_days()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_value`.")
            .to_string();
        let biu_master_file_path = matches
            .value_of("biu_file")
            .expect("Error getting `biu_file_value`.")
            .to_string();
        let td_ret_file_path = matches
            .value_of("td_file")
            .expect("Error getting `td_file_value`.")
            .to_string();
        let rd_ret_file_path = matches
            .value_of("rd_file")
            .expect("Error getting `rd_file_value`.")
            .to_string();
        let max_stable_amount: f64 = matches
            .value_of("max_stable_amt")
            .expect("Error getting `max_stable_amt`.")
            .parse()
            .expect("Invalid amount passed for `max_stable_amt`.");
        let ca_ret_file_path = matches
            .value_of("ca_file")
            .expect("Error getting `ca_file_value`.")
            .to_string();
        let sa_ret_file_path = matches
            .value_of("sa_file")
            .expect("Error getting `sa_file_value`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error while getting `as_on_date`."),
        );

        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let bkt_file_path = matches
            .value_of("bkt_file_path")
            .expect("Error getting `bkt_file_path`.")
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
        let is_casa_stable_amount = matches
            .value_of("is_casa_stable_amount")
            .expect("Error getting `is_casa_stable_amount` flag.")
            .parse::<bool>()
            .expect("Cannot parse `is_casa_stable_amount` flag as bool.");
        let is_lcy_or_ccy = matches
            .value_of("is_lcy_or_ccy")
            .expect("Error getting `is_lcy_or_ccy`.")
            .to_string();
        let lien_file_path = matches
            .value_of("lien_file_path")
            .expect("Error getting `lien_file_path`.")
            .to_string();
        let lien_input_date_format = match matches
            .value_of("lien_input_date_format")
            .expect("Error getting `lien_input_date_format` value.")
        {
            "ddmmyyyy" => "%d%m%Y",
            "ddmmmyyyy" => "%d%b%Y",
            "ddmmyy" => "%d%m%y",
            "yymmdd" => "%y%m%d",
            "dd-mm-yyyy" => "%d-%m-%Y",
            "dd-mmm-yyyy" => "%d-%b-%Y",
            "yyyymmdd" => "%Y%m%d",
            "yyyymmmdd" => "%Y%b%d",
            "yyyy-mm-dd" => "%Y-%m-%d",
            "yyyy-mmm-dd" => "%Y-%b-%d",
            "dd-mmm-yy" => "%d-%b-%y",
            "dd-mm-yy" => "%d-%m-%y",
            "yy-mmm-dd" => "%y-%b-%d",
            "yy-mm-dd" => "%y-%m-%d",
            "dd/mm/yyyy" => "%d/%m/%Y",
            "dd/mmm/yyyy" => "%d/%b/%Y",
            "yyyy/mm/dd" => "%Y/%m/%d",
            "yyyy/mmm/dd" => "%Y/%b/%d",
            "dd/mmm/yy" => "%d/%b/%y",
            "dd/mm/yy" => "%d/%m/%y",
            "yy/mmm/dd" => "%y/%b/%d",
            "yy/mm/dd" => "%y/%m/%d",
            _ => panic!("Invalid Date Format!"),
        }
        .to_string();
        let residual_maturity_days = matches
            .value_of("residual_maturity_days")
            .expect("Error getting `residual_maturity_days`.")
            .parse::<i64>()
            .expect("Cannot parse `residual_maturity_days` as i64.");

        ConfigurationParameters {
            input_file_path,
            biu_master_file_path,
            ca_ret_file_path,
            sa_ret_file_path,
            td_ret_file_path,
            rd_ret_file_path,
            as_on_date,
            output_file_path,
            base_currency,
            is_casa_stable_amount,
            bkt_file_path,
            max_stable_amount,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_lcy_or_ccy,
            lien_file_path,
            lien_input_date_format,
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
    pub fn biu_master_file_path(&self) -> &str {
        &self.biu_master_file_path
    }
    pub fn ca_ret_file_path(&self) -> &str {
        &self.ca_ret_file_path
    }
    pub fn sa_ret_file_path(&self) -> &str {
        &self.sa_ret_file_path
    }
    pub fn td_ret_file_path(&self) -> &str {
        &self.td_ret_file_path
    }
    pub fn rd_ret_file_path(&self) -> &str {
        &self.rd_ret_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn bkt_file_path(&self) -> &str {
        &self.bkt_file_path
    }
    pub fn max_stable_amount(&self) -> f64 {
        self.max_stable_amount
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
    pub fn is_casa_stable_amount(&self) -> bool {
        self.is_casa_stable_amount
    }
    pub fn is_lcy_or_ccy(&self) -> &str {
        &self.is_lcy_or_ccy
    }
    pub fn lien_file_path(&self) -> &str {
        &self.lien_file_path
    }
    pub fn lien_input_date_format(&self) -> &str {
        &self.lien_input_date_format
    }
    pub fn residual_maturity_days(&self) -> &i64 {
        &self.residual_maturity_days
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("BASEL CUSTOMER BALANCE AGGREGATOR LCY")
        .version("1.0.4358")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("biu_file")
                .long("biu-file")
                .value_name("BIU File")
                .help("Path to the biu file.")
                .required(true)
        )
        .arg(
            Arg::with_name("td_file")
                .long("td-file")
                .value_name("TD File")
                .help("Path to the td file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rd_file")
                .long("rd-file")
                .value_name("RD File")
                .help("Path to the rd file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ca_file")
                .long("ca-file")
                .value_name("CA File")
                .help("Path to the ca file.")
                .required(true)
        )
        .arg(
            Arg::with_name("sa_file")
                .long("sa-file")
                .value_name("SA File")
                .help("Path to the sa file.")
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
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("bkt_file_path")
                .long("bkt-file")
                .value_name("BKT File")
                .help("Path to the bkt schema file.")
                .required(true)
        )
        .arg(
            Arg::with_name("max_stable_amt")
                .long("max-stable-amt")
                .value_name("MAX STABLE AMT")
                .help("Max amount to be assigned as stable.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("is_casa_stable_amount")
                .long("is-casa-stable-amount")
                .value_name("IS CASA STABLE AMOUNT")
                .possible_values(&["true", "false"])
                .help("This Flag determines whether stable and unstable classification of CASA is required or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("is_lcy_or_ccy")
                .long("is-lcy-or-ccy")
                .value_name("IS LCY OR CCY")
                .help("Type of Amounts to be processed (Either LCY or CCY).")
                .possible_values(&["LCY","CCY"])
                .required(true)
        )
        .arg(
            Arg::with_name("lien_input_date_format")
                .long("lien-date-format")
                .value_name("Lien Input Date Format")
                .help("Lien Input Date Format.")
                .possible_values(&["ddmmyyyy","ddmmmyyyy","ddmmyy","yymmdd","dd-mm-yyyy","dd-mmm-yyyy", "yyyymmdd","yyyymmmdd","yyyy-mm-dd","yyyy-mmm-dd","dd-mmm-yy","dd-mm-yy","yy-mmm-dd","yy-mm-dd","dd/mm/yyyy","dd/mmm/yyyy","yyyy/mm/dd","yyyy/mmm/dd","dd/mmm/yy","dd/mm/yy","yy/mmm/dd","yy/mm/dd"])                .default_value("dd-mm-yyyy")
                .required(false)
        )
        .arg(
            Arg::with_name("residual_maturity_days")
                .long("residual-maturity-days")
                .value_name("Residual Maturity in Days")
                .help("The residual days to be compared with tenor.")
                .default_value("30")
                .required(false)
        )
        .arg(
            Arg::with_name("lien_file_path")
                .long("lien-file")
                .value_name("LIEN File")
                .help("Path to the lien input file.")
                .required(true)
        )
        .get_matches()
}
