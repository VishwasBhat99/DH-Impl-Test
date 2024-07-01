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
    metadata_file_path: String,
    req_fields_file_path: String,
    balm_rule_file_path: String,
    basel_rule_file_path: String,
    rules_scheme_file_path: String,
    bucket_scheme_file_path: String,
    tbl_computation_file_path: String,
    exchange_rate_file: String,
    is_apply_custom_bkt: bool,
    is_tbl_def_req: bool,
    is_aggr_on_basel_llg: bool,
    is_consolidated: bool,
    is_amt_abs: bool,
    default_basel_llg: i32,
    default_balm_llg: i32,
    country: String,
    base_currency: String,
    source_name: String,
    scheme_id: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    as_on_date: NaiveDate,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path);
        info!(logger, "output_file_path: {}", self.output_file_path);
        info!(logger, "metadata_file_path: {}", self.metadata_file_path);
        info!(
            logger,
            "tbl_computation_file_path: {}", self.tbl_computation_file_path
        );
        info!(logger, "scheme_id: {}", self.scheme_id);
        info!(logger, "source_name: {}", self.source_name);
        info!(logger, "default_balm_llg: {}", self.default_balm_llg);
        info!(logger, "default_basel_llg: {}", self.default_basel_llg);
        info!(
            logger,
            "req_fields_file_path: {}", self.req_fields_file_path
        );
        info!(logger, "balm_rule_file_path: {}", self.balm_rule_file_path);
        info!(
            logger,
            "basel_rule_file_path: {}", self.basel_rule_file_path
        );
        info!(
            logger,
            "rules_scheme_file_path: {}", self.rules_scheme_file_path
        );
        info!(logger, "exchange rate file: {}", self.exchange_rate_file);
        info!(
            logger,
            "bucket_scheme_file_path: {}", self.bucket_scheme_file_path
        );
        info!(
            logger,
            "is_apply_custom_bucket: {}", self.is_apply_custom_bkt
        );

        info!(logger, "is_tbl_def_req: {}", self.is_tbl_def_req);

        info!(logger, "is_tbl_def_req: {}", self.is_aggr_on_basel_llg);
        info!(logger, "is_amt_abs: {}", self.is_amt_abs);
        info!(logger, "log_file: {}", self.log_file_path);
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path);
        info!(logger, "log_level: {}", self.log_level);
        info!(
            logger,
            "diagnostics_flag: {}", self.is_perf_diagnostics_enabled
        );
        info!(logger, "as_on_date: {}", self.as_on_date);
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let balm_rule_file_path = matches
            .value_of("balm_rule_file_path")
            .expect("Error getting `balm_rule_file_path`.")
            .to_string();
        let basel_rule_file_path = matches
            .value_of("basel_rule_file_path")
            .expect("Error getting `basel_rule_file_path`.")
            .to_string();
        let rules_scheme_file_path = matches
            .value_of("rules_scheme_file_path")
            .expect("Error getting `rules_scheme_file_path`.")
            .to_string();
        let bucket_scheme_file_path = matches
            .value_of("bucket_scheme_file_path")
            .expect("Error getting `bucket_scheme_file_path`.")
            .to_string();
        let tbl_computation_file_path = matches
            .value_of("tbl_computation_file_path")
            .expect("Error getting `tbl_computation_file_path`.")
            .to_string();
        let scheme_id = matches
            .value_of("scheme_id")
            .expect("Error getting `scheme_id`.")
            .to_string();
        let source_name = matches
            .value_of("source_name")
            .expect("Error getting `source_name`.")
            .to_string();
        let default_balm_llg = matches
            .value_of("default_balm_llg")
            .expect("Error getting `default_balm_llg`.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_balm_llg` as i32.");
        let default_basel_llg = matches
            .value_of("default_basel_llg")
            .expect("Error getting `default_basel_llg`.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_basel_llg` as i32.");
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let country = matches
            .value_of("country")
            .expect("Error getting `Country`.")
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
        let is_apply_custom_bkt = matches
            .value_of("is_apply_custom_bkt")
            .expect("Error getting `is_apply_custome_bkt`.")
            .parse::<bool>()
            .expect("Cannot parse `is_apply_custom_bkt` as bool.");
        let is_tbl_def_req = matches
            .value_of("is_tbl_def_req")
            .expect("Error getting `is_tbl_def_req`.")
            .parse::<bool>()
            .expect("Cannot parse `is_tbl_def_req` as bool.");
        let is_amt_abs = matches
            .value_of("is_amt_abs")
            .expect("Error getting `is_amt_abs`.")
            .parse::<bool>()
            .expect("Cannot parse `is_amt_abs` as bool.");
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let is_aggr_on_basel_llg = matches
            .value_of("is_aggr_on_basel_llg")
            .expect("Error getting `is_aggr_on_basel_llg`.")
            .parse::<bool>()
            .expect("Cannot parse `is_aggr_on_basel_llg` as bool.");
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            output_file_path,
            metadata_file_path,
            rules_scheme_file_path,
            req_fields_file_path,
            balm_rule_file_path,
            basel_rule_file_path,
            is_amt_abs,
            bucket_scheme_file_path,
            is_aggr_on_basel_llg,
            tbl_computation_file_path,
            source_name,
            default_balm_llg,
            is_apply_custom_bkt,
            default_basel_llg,
            country,
            base_currency,
            scheme_id,
            as_on_date,
            is_tbl_def_req,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            exchange_rate_file,
            is_consolidated,
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
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn balm_rule_file_path(&self) -> &str {
        &self.balm_rule_file_path
    }
    pub fn basel_rule_file_path(&self) -> &str {
        &self.basel_rule_file_path
    }
    pub fn rules_scheme_file_path(&self) -> &str {
        &self.rules_scheme_file_path
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn tbl_computation_file_path(&self) -> &str {
        &self.tbl_computation_file_path
    }
    pub fn is_amt_abs(&self) -> bool {
        self.is_amt_abs
    }
    pub fn bucket_scheme_file_path(&self) -> &str {
        &self.bucket_scheme_file_path
    }
    pub fn scheme_id(&self) -> &str {
        &self.scheme_id
    }
    pub fn source_name(&self) -> &str {
        &self.source_name
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
    pub fn is_tbl_def_req(&self) -> bool {
        self.is_tbl_def_req
    }

    pub fn is_aggr_on_basel_llg(&self) -> bool {
        self.is_aggr_on_basel_llg
    }

    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }

    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }

    pub fn default_balm_llg(&self) -> i32 {
        self.default_balm_llg
    }
    pub fn default_basel_llg(&self) -> i32 {
        self.default_basel_llg
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("BALM TO BASEL CF")
        .version("1.0.4057")
        .author("Saurabh Singh <saurabh.s@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
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
            Arg::with_name("metadata_file_path")
                .long("metadata-file")
                .value_name("Metadata File")
                .help("Path to the metadata file.")
                .required(true),
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file")
                .value_name("Req Fields File")
                .help("Path to the req fields file.")
                .required(true),
        )
        .arg(
            Arg::with_name("balm_rule_file_path")
                .long("balm-rule-file")
                .value_name("Balm Rule File")
                .help("Path to the BALM rule file.")
                .required(true),
        )
        .arg(
            Arg::with_name("basel_rule_file_path")
                .long("basel-rule-file")
                .value_name("Basel Rule File")
                .help("Path to the Basel rule file.")
                .required(true),
        )
        .arg(
            Arg::with_name("tbl_computation_file_path")
                .long("tbl-comp-file")
                .value_name("Tbl Computation File")
                .help("Path to the Tbl Computation file.")
                .required(true),
        )
        .arg(
            Arg::with_name("rules_scheme_file_path")
                .long("rules-scheme-file")
                .value_name("Rules Scheme File")
                .help("Path to the rules scheme file.")
                .required(true),
        )
        .arg(
            Arg::with_name("bucket_scheme_file_path")
                .long("bucket-scheme-file")
                .value_name("Bucket Scheme File")
                .help("Path to the bucket scheme file.")
                .required(true),
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("Exchange Rate File")
                .help("Path to the Exchange Rate File")
                .required(true),
        )
        .arg(
            Arg::with_name("scheme_id")
                .long("scheme-id")
                .value_name("Scheme ID")
                .help("Scheme ID value to be picked from rules scheme file.")
                .required(true),
        )
        .arg(
            Arg::with_name("source_name")
                .long("src-name")
                .value_name("Source Name")
                .help("Source Name which has to be picked from tbl computation file.")
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
            Arg::with_name("is_apply_custom_bkt")
                .long("is-apply-custom-bkt")
                .value_name("CUSTOM BUCKET FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to apply custome bucketing for aggregation")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_amt_abs")
                .long("is-amt-abs")
                .value_name("AMOUNT ABSOLUTE FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to apply take absolute amount value.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("is_aggr_on_basel_llg")
                .long("is-aggr-on-basel-llg")
                .value_name("BASEL LLG FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to apply custome bucketing for aggregation")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_tbl_def_req")
                .long("is-tbl-def-req")
                .value_name("TBL DEPOSIT COMPUTATION FLAG")
                .possible_values(&["true", "false"])
                .help("This flag decides whether to check tbl computation file for skipping records")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consoldated")
                .value_name("CONSOLIDATION FLAG")
                .possible_values(&["true", "false"])
                .help("This flag decides whether the amount is equivalent to base currency")
                .default_value("true")
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
            Arg::with_name("default_balm_llg")
                .long("default-balm-llg")
                .value_name("DEFAULT BALM LLG")
                .help("This is the default balm llg.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("base_currency")
                .help("base_currency value")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("Country")
                .help("Country Value")
                .required(true)
        )
        .arg(
            Arg::with_name("default_basel_llg")
                .long("default-basel-llg")
                .value_name("DEFAULT BASEL LLG")
                .help("This is the default basel llg.")
                .required(true)
        )
        .get_matches()
}
