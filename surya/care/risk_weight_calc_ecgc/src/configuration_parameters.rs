use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    config_file_path: String,
    col_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    default_risk_weight_covered: f64,
    cover_thrs_bal_pre_shipment: f64,
    cover_thrs_bal_post_shipment: f64,
    cutoff_cover_prcnt: f64,
    default_cover_prcnt: f64,
    ccy_mm_hc_prcnt: f64,
    mat_mm_hc_prcnt: f64,
    exchange_rate_file: String,
    base_currency: String,
    ccf_prcnt: f64,
    amt_set_limit: f64,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "col_file_path: {}", self.col_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "default_risk_weight_covered: {}",
            self.default_risk_weight_covered()
        );
        info!(logger, "ccy_mm_prcnt: {}", self.ccy_mm_hc_prcnt());
        info!(
            logger,
            "cover_thrs_bal_pre_shipment: {}",
            self.cover_thrs_bal_pre_shipment()
        );
        info!(
            logger,
            "cover_thrs_bal_post_shipment: {}",
            self.cover_thrs_bal_post_shipment()
        );
        info!(logger, "cutoff_cover_prcnt: {}", self.cutoff_cover_prcnt());
        info!(
            logger,
            "default_cover_prcnt: {}",
            self.default_cover_prcnt()
        );
        info!(logger, "mat_mm_prcnt: {}", self.mat_mm_hc_prcnt());
        info!(logger, "ccf_prcnt: {}", self.ccf_prcnt());
        info!(logger, "amt_set_limit: {}", self.amt_set_limit());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let col_file_path = matches
            .value_of("col_file_path")
            .expect("Error getting `col_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let default_risk_weight_covered: f64 = matches
            .value_of("default_risk_weight_covered")
            .expect("Error getting `default_risk_weight_covered`.")
            .parse()
            .expect("Cannot parse default_risk_weight_covered risk weight");
        let ccy_mm_hc_prcnt: f64 = matches
            .value_of("ccy_mm_hc_prcnt")
            .expect("Error getting `ccy_mm_hc_prcnt`.")
            .parse()
            .expect("Cannot parse ccy_mm_hc_prcnt");
        let amt_set_limit: f64 = matches
            .value_of("amt_set_limit")
            .expect("Error getting `amt_set_limit`.")
            .parse()
            .expect("Cannot parse amt_set_limit");
        let cutoff_cover_prcnt: f64 = matches
            .value_of("cutoff_cover_prcnt")
            .expect("Error getting `cutoff_cover_prcnt`.")
            .parse()
            .expect("Cannot parse cutoff_cover_prcnt");
        let cover_thrs_bal_pre_shipment: f64 = matches
            .value_of("cover_thrs_bal_pre_shipment")
            .expect("Error getting `cover_thrs_bal_pre_shipment`.")
            .parse()
            .expect("Cannot parse cover_thrs_bal_pre_shipment");
        let cover_thrs_bal_post_shipment: f64 = matches
            .value_of("cover_thrs_bal_post_shipment")
            .expect("Error getting `cover_thrs_bal_post_shipment`.")
            .parse()
            .expect("Cannot parse cover_thrs_bal_post_shipment");
        let default_cover_prcnt: f64 = matches
            .value_of("default_cover_prcnt")
            .expect("Error getting `default_cover_prcnt`.")
            .parse()
            .expect("Cannot parse default_cover_prcnt");
        let mat_mm_hc_prcnt: f64 = matches
            .value_of("mat_mm_hc_prcnt")
            .expect("Error getting `mat_mm_hc_prcnt`.")
            .parse()
            .expect("Cannot parse mat_mm_hc_prcnt");
        let ccf_prcnt: f64 = matches
            .value_of("ccf_prcnt")
            .expect("Error getting `ccf_prcnt`.")
            .parse()
            .expect("Cannot parse ccf_prcnt");
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
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
            config_file_path,
            col_file_path,
            output_file_path,
            as_on_date,
            default_risk_weight_covered,
            cover_thrs_bal_pre_shipment,
            cover_thrs_bal_post_shipment,
            cutoff_cover_prcnt,
            default_cover_prcnt,
            ccy_mm_hc_prcnt,
            mat_mm_hc_prcnt,
            exchange_rate_file,
            base_currency,
            ccf_prcnt,
            amt_set_limit,
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
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn col_file_path(&self) -> &str {
        &self.col_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn default_risk_weight_covered(&self) -> &f64 {
        &self.default_risk_weight_covered
    }
    pub fn ccy_mm_hc_prcnt(&self) -> &f64 {
        &self.ccy_mm_hc_prcnt
    }
    pub fn cover_thrs_bal_pre_shipment(&self) -> &f64 {
        &self.cover_thrs_bal_pre_shipment
    }
    pub fn cover_thrs_bal_post_shipment(&self) -> &f64 {
        &self.cover_thrs_bal_post_shipment
    }
    pub fn cutoff_cover_prcnt(&self) -> &f64 {
        &self.cutoff_cover_prcnt
    }
    pub fn default_cover_prcnt(&self) -> &f64 {
        &self.default_cover_prcnt
    }
    pub fn mat_mm_hc_prcnt(&self) -> &f64 {
        &self.mat_mm_hc_prcnt
    }
    pub fn ccf_prcnt(&self) -> &f64 {
        &self.ccf_prcnt
    }
    pub fn amt_set_limit(&self) -> &f64 {
        &self.amt_set_limit
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
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

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This program derived risk weight for ECGC using rules lib!!")
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file-path")
                .value_name("Config File Path")
                .help("Path to Config File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_risk_weight_covered")
                .long("default-risk-weight-covered")
                .value_name("DEFAULT RISK WEIGHT COVERED")
                .help("The default risk weight for covered portion")
                .required(true)
        )
        .arg(
            Arg::with_name("ccy_mm_hc_prcnt")
                .long("ccy-mm-hc-prcnt")
                .value_name("CCY MISMATCH HAIRCUT PERCENTAGE")
                .help("The CCY MISMATCH HAIRCUT PERCENTAGE")
                .required(true)
        )
        .arg(
            Arg::with_name("cover_thrs_bal_post_shipment")
                .long("cover-thrs-bal-post-shipment")
                .value_name("MAX COVER AMOUNT POST SHIPMENT")
                .help("The MAX COVER AMOUNT POST SHIPMENT for ECGC accounts")
                .required(true)
        )
        .arg(
            Arg::with_name("cover_thrs_bal_pre_shipment")
                .long("cover-thrs-bal-pre-shipment")
                .value_name("MAX COVER AMOUNT PRE SHIPMENT")
                .help("The MAX COVER AMOUNT PRE SHIPMENT for ECGC accounts")
                .required(true)
        )
        .arg(
            Arg::with_name("amt_set_limit")
                .long("amt-set-limit")
                .value_name("Set Amount Limit")
                .help("The amount convered limit")
                .default_value("10200000.0")
                .required(false)
        )
        .arg(
            Arg::with_name("cutoff_cover_prcnt")
                .long("cutoff-cover-prcnt")
                .value_name("CUT-OFF COVER PERCENTAGE")
                .help("The CUT-OFF COVER PERCENTAGE for ECGC accounts")
                .required(true)
        )
        .arg(
            Arg::with_name("default_cover_prcnt")
                .long("default-cover-prcnt")
                .value_name("DEFAULT COVER PERCENTAGE")
                .help("The DEFAULT COVER PERCENTAGE for ECGC accounts")
                .required(true)
        )
        .arg(
            Arg::with_name("mat_mm_hc_prcnt")
                .long("mat-mm-hc-prcnt")
                .value_name("MATURITY MISMATCH HAIRCUT PERCENTAGE")
                .help("The MATURITY MISMATCH HAIRCUT PERCENTAGE")
                .required(true)
        )
        .arg(
            Arg::with_name("ccf_prcnt")
                .long("ccf-prcnt")
                .value_name("CCF PERCENTAGE")
                .help("The CCF PERCENTAGE")
                .default_value("100.0")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to Output File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("col_file_path")
                .long("col-file-path")
                .value_name("Collateral File Path")
                .help("Path to Collateral File Path.")
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
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("Base Currency.")
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
