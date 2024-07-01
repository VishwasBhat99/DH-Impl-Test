use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

use crate::log;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    metadata_file_path: String,
    base_currency: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    bucket_scheme_id: String,
    by_bucket_struct: String,
    non_by_bucket_id: i64,
    req_field_path: String,
    maturity_bkt_def_file_path: String,
    rules_file_path: String,
    exchange_rate_file_path: String,
    is_consolidated: bool,
    default_llg_code: i32,
    llg_id_list: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "is_consolidated: {}", self.is_consolidated);
        info!(
            logger,
            "exchange_rate_file_path: {}", self.exchange_rate_file_path
        );
        info!(logger, "output_file: {}", self.output_file_path());

        info!(logger, "bucket_scheme_id: {}", self.bucket_scheme_id());
        info!(logger, "by_bucket_struct: {:?}", self.by_bucket_struct());
        info!(logger, "non_by_bucket_id: {}", self.non_by_bucket_id());
        info!(logger, "req_field_path: {}", self.req_field_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(
            logger,
            "maturity_bkt_def_file_path: {}",
            self.maturity_bkt_def_file_path()
        );
        info!(
            logger,
            "exchange_rate_file_path: {}", self.exchange_rate_file_path
        );
        info!(logger, "llg_id_list: {}", self.llg_id_list);
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let bucket_scheme_id = matches
            .value_of("bucket_scheme_id")
            .expect("Error getting `bucket_scheme_id`.")
            .to_string();
        let by_bucket_struct = matches
            .value_of("by_bucket_struct")
            .expect("Error getting `by_bucket_struct`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");
        let non_by_bucket_id = matches
            .value_of("non_by_bucket_id")
            .expect("Error getting `non_by_bucket_id`.")
            .parse::<i64>()
            .unwrap_or(0);
        let req_field_path = matches
            .value_of("req_field_path")
            .expect("Error getting `req_field_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `Output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `General log file path`.")
            .to_string();
        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file_path")
            .expect("Error getting `exchange_rate_file_path`.")
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

        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `Diagnostics log file path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting ``.")
            .parse::<bool>()
            .expect("Error while parsing `is perf diagnostics enabled` as bool.");
        let maturity_bkt_def_file_path = matches
            .value_of("maturity_bkt_def_file_path")
            .expect("Error getting `maturity_bkt_def_file_path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error while getting rules file path.")
            .to_string();
        let llg_id_list = matches
            .value_of("llg_id_list")
            .expect("Error while getting `llg_id_list`.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            bucket_scheme_id,
            as_on_date,
            by_bucket_struct,
            non_by_bucket_id,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            metadata_file_path,
            req_field_path,
            default_llg_code,
            base_currency,
            maturity_bkt_def_file_path,
            rules_file_path,
            is_consolidated,
            exchange_rate_file_path,
            llg_id_list,
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
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn req_field_path(&self) -> &str {
        &self.req_field_path
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn bucket_scheme_id(&self) -> &str {
        &self.bucket_scheme_id
    }
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn by_bucket_struct(&self) -> &str {
        &self.by_bucket_struct
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn non_by_bucket_id(&self) -> i64 {
        self.non_by_bucket_id
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
    pub fn maturity_bkt_def_file_path(&self) -> &str {
        &self.maturity_bkt_def_file_path
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn llg_id_list(&self) -> &str {
        &self.llg_id_list
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Dimensinal Aggregator.")
        .version("1.0.4674")
        .author("Tanuj Singh Rathore<tanuj.s@surya-soft.com>")        
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("FILE")
                .help("Path to the input cust agg file")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file_path")
                .long("exchange-rate-file-path")
                .value_name("Exchange Rate File")
                .help("Path to the Exchange Rate File.")
                .required(true),
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file_path")
                .long("metadata-file-path")
                .value_name("metadata file path")
                .help("Path to metadata file")
                .required(true)
        )
        .arg(
            Arg::with_name("maturity_bkt_def_file_path")
                .long("mat-bkt-def-file-path")
                .value_name("Maturity Bucket Definition File")
                .help("Path to the Maturity Bucket Definition File.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
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
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
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
            Arg::with_name("req_field_path")
                .long("req-fields-path")
                .value_name("FILE")
                .help("Path to the required fields file.")
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
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("bucket_scheme_id")
                .long("bucket-scheme-id")
                .value_name("Bucket Scheme Id")
                .help("Bucket Scheme Id")
                .required(true)
        )
        .arg(
            Arg::with_name("non_by_bucket_id")
                .long("non-by-bucket-id")
                .value_name("Non By Bucket ID")
                .help("Which bucket need to pick,if input file does not follows 203 structure.")
                .required(false)
        )
        .arg(
            Arg::with_name("by_bucket_struct")
                .long("by-bucket-struct")
                .value_name("By Bucket Structure")
                .help("To check whether input file follows 203 structure or not.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
                .required(true)
        )
        .arg(
            Arg::with_name("llg_id_list")
                .long("llg-id")
                .value_name("LLG ID LIST")
                .help("This will contains list of llg id for which amount will come in negative.")
                .default_value("")
                .required(false)
        )
        .get_matches()
}
