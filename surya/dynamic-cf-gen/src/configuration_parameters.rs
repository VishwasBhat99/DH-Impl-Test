use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    prj_coa: String,
    as_on_date: NaiveDate,
    new_business_value: f64,
    currency: String,
    interest_basis: String,
    cf_type: String,
    coa_master_file_path: String,
    pre_payment_rates_file: String,
    disbursement_by_bm_file_path: String,
    disbursement_by_day_file_path: String,
    disbursement_by_tenor_file_path: String,
    bm_rates_file_path: String,
    bm_rates_type: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "prj_coa: {}", self.prj_coa());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "new_business_value: {}", self.new_business_value());
        info!(logger, "currency: {}", self.currency());
        info!(logger, "interest_basis: {}", self.interest_basis());
        info!(logger, "cf_type: {}", self.cf_type());
        info!(
            logger,
            "coa_master_file_path: {}",
            self.coa_master_file_path()
        );
        info!(
            logger,
            "pre_payment_rates_file: {}",
            self.pre_payment_rates_file()
        );
        info!(
            logger,
            "disbursement_by_bm_file_path: {}",
            self.disbursement_by_bm_file_path()
        );
        info!(
            logger,
            "disbursement_by_day_file_path: {}",
            self.disbursement_by_day_file_path()
        );
        info!(
            logger,
            "disbursement_by_tenor_file_path: {}",
            self.disbursement_by_tenor_file_path()
        );
        info!(logger, "bm_rates_file_path: {}", self.bm_rates_file_path());
        info!(logger, "bm_rates_type: {}", self.bm_rates_type());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let prj_coa = matches
            .value_of("prj_coa")
            .expect("Error getting `prj_coa`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let new_business_value = matches
            .value_of("new_business_value")
            .expect("Error getting `new_business_value`.")
            .parse::<f64>()
            .expect("Cannot parse `new_business_value` as f64.");
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let interest_basis = matches
            .value_of("interest_basis")
            .expect("Error getting `interest_basis`.")
            .to_string();
        let cf_type = matches
            .value_of("cf_type")
            .expect("Error getting `cf_type`.")
            .to_string();
        let pre_payment_rates_file = matches
            .value_of("pre_payment_rates_file")
            .expect("Error getting `pre_payment_rates_file`.")
            .to_string();
        let coa_master_file_path = matches
            .value_of("coa_master_file_path")
            .expect("Error getting `coa_master_file_path`.")
            .to_string();
        let disbursement_by_bm_file_path = matches
            .value_of("disbursement_by_bm_file_path")
            .expect("Error getting `disbursement_by_bm_file_path`.")
            .to_string();
        let disbursement_by_day_file_path = matches
            .value_of("disbursement_by_day_file_path")
            .expect("Error getting `disbursement_by_day_file_path`.")
            .to_string();
        let disbursement_by_tenor_file_path = matches
            .value_of("disbursement_by_tenor_file_path")
            .expect("Error getting `disbursement_by_tenor_file_path`.")
            .to_string();
        let bm_rates_file_path = matches
            .value_of("bm_rates_file_path")
            .expect("Error getting `bm_rates_file_path`.")
            .to_string();
        let bm_rates_type = matches
            .value_of("bm_rates_type")
            .expect("Error getting `bm_rates_type`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
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
            prj_coa,
            as_on_date,
            new_business_value,
            currency,
            interest_basis,
            cf_type,
            coa_master_file_path,
            pre_payment_rates_file,
            disbursement_by_bm_file_path,
            disbursement_by_day_file_path,
            disbursement_by_tenor_file_path,
            bm_rates_file_path,
            bm_rates_type,
            output_file_path,
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
    pub fn prj_coa(&self) -> &str {
        &self.prj_coa
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn new_business_value(&self) -> &f64 {
        &self.new_business_value
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn interest_basis(&self) -> &str {
        &self.interest_basis
    }
    pub fn cf_type(&self) -> &str {
        &self.cf_type
    }
    pub fn coa_master_file_path(&self) -> &str {
        &self.coa_master_file_path
    }
    pub fn pre_payment_rates_file(&self) -> &str {
        &self.pre_payment_rates_file
    }
    pub fn disbursement_by_bm_file_path(&self) -> &str {
        &self.disbursement_by_bm_file_path
    }
    pub fn disbursement_by_day_file_path(&self) -> &str {
        &self.disbursement_by_day_file_path
    }
    pub fn disbursement_by_tenor_file_path(&self) -> &str {
        &self.disbursement_by_tenor_file_path
    }
    pub fn bm_rates_file_path(&self) -> &str {
        &self.bm_rates_file_path
    }
    pub fn bm_rates_type(&self) -> &str {
        &self.bm_rates_type
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
        .about("Generate Hypothetical Account with cashflows!!")
        .version("1.0.2061")
        .arg(
            Arg::with_name("prj_coa")
                .long("prj-coa")
                .value_name("Projection COA")
                .help("COA Code for Projection.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("new_business_value")
                .long("new-business-value")
                .value_name("New Business Value")
                .help("Value of new business to be generated.")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("Currency")
                .help("Currency of New Business.")
                .required(true)
        )
            .arg(
                Arg::with_name("interest_basis")
                    .long("interest-basis")
                    .value_name("Interest Basis")
                    .help("Day Convention to be used.")
                    .required(true)
            )
        .arg(
            Arg::with_name("cf_type")
                .long("cf-type")
                .value_name("CF TYPE")
                .help("Cashflow Type")
                .required(true)
        )
        .arg(
            Arg::with_name("pre_payment_rates_file")
                .long("pre-payment-rates-file")
                .value_name("Pre Payment File Path")
                .help("Path to Pre Payment File")
                .required(true)
        )
        .arg(
            Arg::with_name("coa_master_file_path")
                .long("coa-master-file-path")
                .value_name("COA Master File Path")
                .help("Path to COA Master File")
                .required(true)
        )
        .arg(
            Arg::with_name("disbursement_by_bm_file_path")
                .long("disbursement-by-bm-file-path")
                .value_name("Disbursement by BM File Path")
                .help("Path to Disbursement by BM File")
                .required(true)
        )
        .arg(
            Arg::with_name("disbursement_by_day_file_path")
                .long("disbursement-by-day-file-path")
                .value_name("Disbursement by Day File Path")
                .help("Path to Disbursement by Day File.")
                .required(true)
        )
        .arg(
            Arg::with_name("disbursement_by_tenor_file_path")
                .long("disbursement-by-tenor-file-path")
                .value_name("Disbursement by Tenor File Path")
                .help("Path to Disbursement by Tenor File.")
                .required(true)
        )
        .arg(
            Arg::with_name("bm_rates_file_path")
                .long("bm-rates-file-path")
                .value_name("BM Rates File Path")
                .help("Path to BM Rates File.")
                .required(true)
        )
        .arg(
            Arg::with_name("bm_rates_type")
                .long("bm-rates-type")
                .value_name("BM Rates Type")
                .help("Convention for BM Rate selection")
                .possible_values(&["Daily", "Monthly"])
                .default_value("Monthly")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to Output File.")
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
