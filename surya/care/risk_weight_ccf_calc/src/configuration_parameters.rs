use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    prov_file_path: String,
    col_file_path: String,
    output_file_path: String,
    metadata_file_path: String,
    rules_file_path: String,
    req_fields_file_path: String,
    as_on_date: NaiveDate,
    src_file_name: String,
    default_risk_weight: i32,
    default_sub_claim_id: i32,
    ccy_mm_hc_prcnt: f64,
    mat_mm_hc_prcnt: f64,
    exchange_rate_file: String,
    base_currency: String,
    is_consolidated: bool,
    default_ccf_prcnt: f64,
    is_negative: bool,
    neg_crm_check: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "prov_file_path: {}", self.prov_file_path());
        info!(logger, "col_file_path: {}", self.col_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "neg_crm_check: {}", self.neg_crm_check());
        info!(logger, "src_file_name: {}", self.src_file_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "default_risk_weight: {}",
            self.default_risk_weight()
        );
        info!(
            logger,
            "default_sub_claim_id: {}",
            self.default_sub_claim_id()
        );
        info!(logger, "ccy_mm_prcnt: {}", self.ccy_mm_hc_prcnt());
        info!(logger, "mat_mm_prcnt: {}", self.mat_mm_hc_prcnt());
        info!(logger, "default_ccf_prcnt: {}", self.default_ccf_prcnt());
        info!(logger, "is_negative: {}", self.is_negative());
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
        let col_file_path = matches
            .value_of("col_file_path")
            .expect("Error getting `col_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let prov_file_path = matches
            .value_of("prov_file_path")
            .expect("Error getting `prov_file_path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error getting `rules_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let src_file_name = matches
            .value_of("src_file_name")
            .expect("Error getting `src_file_name`.")
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
        let default_risk_weight: i32 = matches
            .value_of("default_risk_weight")
            .expect("Error getting `default_risk_weight`.")
            .parse()
            .expect("Cannot parse default risk weight");
        let default_sub_claim_id: i32 = matches
            .value_of("default_sub_claim_id")
            .expect("Error getting `default_sub_claim_id`.")
            .parse()
            .expect("Cannot parse default_sub_claim_id");
        let ccy_mm_hc_prcnt: f64 = matches
            .value_of("ccy_mm_hc_prcnt")
            .expect("Error getting `ccy_mm_hc_prcnt`.")
            .parse()
            .expect("Cannot parse ccy_mm_hc_prcnt");
        let mat_mm_hc_prcnt: f64 = matches
            .value_of("mat_mm_hc_prcnt")
            .expect("Error getting `mat_mm_hc_prcnt`.")
            .parse()
            .expect("Cannot parse mat_mm_hc_prcnt");
        let default_ccf_prcnt: f64 = matches
            .value_of("default_ccf_prcnt")
            .expect("Error getting `default_ccf_prcnt`.")
            .parse()
            .expect("Cannot parse default_ccf_prcnt");
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let is_negative = matches
            .value_of("is_negative")
            .expect("Error getting `is_negative`.")
            .parse::<bool>()
            .expect("Cannot parse `is_negative` as bool.");
        let neg_crm_check = matches
            .value_of("neg_crm_check")
            .expect("Error getting `neg_crm_check`.")
            .parse::<bool>()
            .expect("Cannot parse `neg_crm_check` as bool.");
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
            prov_file_path,
            col_file_path,
            output_file_path,
            metadata_file_path,
            rules_file_path,
            req_fields_file_path,
            as_on_date,
            src_file_name,
            default_risk_weight,
            default_sub_claim_id,
            ccy_mm_hc_prcnt,
            mat_mm_hc_prcnt,
            exchange_rate_file,
            base_currency,
            is_consolidated,
            default_ccf_prcnt,
            is_negative,
            neg_crm_check,
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
    pub fn prov_file_path(&self) -> &str {
        &self.prov_file_path
    }
    pub fn col_file_path(&self) -> &str {
        &self.col_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn src_file_name(&self) -> &str {
        &self.src_file_name
    }
    pub fn default_risk_weight(&self) -> &i32 {
        &self.default_risk_weight
    }
    pub fn default_sub_claim_id(&self) -> &i32 {
        &self.default_sub_claim_id
    }
    pub fn ccy_mm_hc_prcnt(&self) -> &f64 {
        &self.ccy_mm_hc_prcnt
    }
    pub fn mat_mm_hc_prcnt(&self) -> &f64 {
        &self.mat_mm_hc_prcnt
    }
    pub fn default_ccf_prcnt(&self) -> &f64 {
        &self.default_ccf_prcnt
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }
    pub fn neg_crm_check(&self) -> bool {
        self.neg_crm_check
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
        .about("This program derived risk weight using rules lib!!")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .version("1.0.4744")
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("prov_file_path")
                .long("prov-file-path")
                .value_name("Provisional File Path")
                .help("Path to Provisional File Path.")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::with_name("default_sub_claim_id")
                .long("default-sub-claim-id")
                .value_name("DEFAULT SUB CLAIM ID")
                .help("The default sub claim id")
                .required(true)
        )
        .arg(
            Arg::with_name("default_risk_weight")
                .long("default-risk-weight")
                .value_name("DEFAULT RISK WEIGHT")
                .help("The default risk weight")
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
            Arg::with_name("mat_mm_hc_prcnt")
                .long("mat-mm-hc-prcnt")
                .value_name("MATURITY MISMATCH HAIRCUT PERCENTAGE")
                .help("The MATURITY MISMATCH HAIRCUT PERCENTAGE")
                .required(true)
        )
        .arg(
            Arg::with_name("default_ccf_prcnt")
                .long("default-ccf-prcnt")
                .value_name("DEFAULT CCF PERCENTAGE")
                .help("The DEFAULT CCF PERCENTAGE")
                .default_value("100.0")
                .required(false)
        )
        .arg(
            Arg::with_name("src_file_name")
                .long("src-file-name")
                .value_name("SRC FILE NAME")
                .help("Name of source file")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to Output File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to Input File Path.")
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
            Arg::with_name("metadata_file_path")
                .long("metadata-file-path")
                .value_name("Metadata File Path")
                .help("Path to Metadata File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("Rules File Path")
                .help("Path to Rules File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file-path")
                .value_name("Req Fields File Path")
                .help("Path to Req Fields File Path.")
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
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount is consolidated of native.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_negative")
                .long("is-negative")
                .value_name("IS NEGATIVE")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount is to be multiplied by -1 or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("neg_crm_check")
                .long("neg-crm-check")
                .value_name("NEGATIVE CRM CHECK")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to check CRM Negative amount or not.")
                .default_value("true")
                .required(false)
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
