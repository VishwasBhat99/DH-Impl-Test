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
    report_id: String,
    input_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    known_fields_file_path: String,
    account_metadata_file_path: String,
    bkt_info: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    final_bkt_required: bool,
    amt_type: String,
    rules_file_path: String,
    default_llg: i32,
    is_nonmat_bucket_available: String,
    is_consolidated: bool,
    base_ccy: String,
    exchange_rate_file: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "report_id: {}", self.report_id());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "known_fields_file_path: {}",
            self.known_fields_file_path()
        );
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(logger, "Bucket Config: {}", self.bkt_info());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "Final bucket required: {}",
            self.final_bkt_required()
        );
        info!(logger, "Amount type: {}", self.amt_type());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "default_llg_code: {}", self.default_llg());
        info!(
            logger,
            "is_nonmat_bucket_available {}",
            self.is_nonmat_bucket_available()
        );
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "base_ccy {}", self.base_ccy());
        info!(logger, "exchange_rate_file {}", self.exchange_rate_file());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches.value_of("input_file").unwrap().to_string();
        let report_id = matches.value_of("report_id").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let known_fields_file_path = matches.value_of("known_fields_file").unwrap().to_string();

        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .unwrap()
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .unwrap()
            .to_string();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let bkt_info = matches.value_of("bkt_info").unwrap().to_string();
        let final_bkt_required = matches
            .value_of("final_bkt_required")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let amt_type = matches.value_of("amt_type").unwrap().to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error while getting rules file path.")
            .to_string();
        let default_llg = matches
            .value_of("default_llg")
            .expect("Error while getting `default llg`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default llg` as integer.");
        let is_nonmat_bucket_available = matches
            .value_of("is_nonmat_bucket_available")
            .expect("Error while getting `non maturity bucket`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let base_ccy = matches
            .value_of("base_ccy")
            .expect("Error getting `base_ccy`.")
            .to_string();
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        ConfigurationParameters {
            report_id,
            input_file_path,
            output_file_path,
            as_on_date,
            log_file_path,
            known_fields_file_path,
            account_metadata_file_path,
            bkt_info,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            final_bkt_required,
            amt_type,
            rules_file_path,
            default_llg,
            is_nonmat_bucket_available,
            is_consolidated,
            base_ccy,
            exchange_rate_file,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn report_id(&self) -> &str {
        &self.report_id
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn bkt_info(&self) -> &str {
        &self.bkt_info
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn known_fields_file_path(&self) -> &str {
        &self.known_fields_file_path
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
    pub fn final_bkt_required(&self) -> bool {
        self.final_bkt_required
    }
    pub fn amt_type(&self) -> &str {
        &self.amt_type
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn default_llg(&self) -> i32 {
        self.default_llg
    }
    pub fn is_nonmat_bucket_available(&self) -> &str {
        &self.is_nonmat_bucket_available
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn base_ccy(&self) -> &str {
        &self.base_ccy
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.1.4283")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .about("This program generates output for special bucketing for MIS!")
        .arg(
            Arg::with_name("report_id")
                .long("report-id")
                .value_name("REPORT ID")
                .help("Name of report to be generated")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .short("e")     // 'l', 'o' taken; 'g' doesn't make sense; 'v' stands for 'verbose'
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("known_fields_file")
                .short("k")
                .long("known-fields-file")
                .value_name("KNOWN_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .short("m")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("bkt_info")
                .long("bkt-info")
                .value_name("BUCKET INFO")
                .help("Config of Buckets Required.")
                .required(true)
        )
        .arg(
            Arg::with_name("final_bkt_required")
            .long("final-bkt-required")
            .value_name("FINAL BUCKET")
            .help("This flag decides if final bucket is required or not.")
            .default_value("false")
            .required(false)
        )
        .arg(
            Arg::with_name("amt_type")
            .long("amt-type")
            .value_name("AMOUNT TYPE")
            .help("This flag decides output to be generated for PRINCIPAL amount or INTEREST amount.")
            .possible_values(&["PRIN","INT"])
            .default_value("PRIN")
            .required(false)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
                .default_value("NONE")
                .required(false)
        )
        .arg(
            Arg::with_name("default_llg")
            .long("default-llg")
            .value_name("DEFAULT-LLG")
            .help("This flag has default llg value.")
            .default_value("0")
            .required(false)
        )
        .arg(
            Arg::with_name("is_nonmat_bucket_available")
            .long("is-nonmat-bkt-available")
            .value_name("is-nonmat-bkt-available")
            .help("This is nonmat bkt key name.")
            .default_value("NA")
            .required(false)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount in input is consolidated or not.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("base_ccy")
                .long("base-ccy")
                .value_name("BASE CURRENCY")
                .help("THE VALUE OF BASE CURRENCY")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .get_matches()
}
