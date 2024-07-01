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
    output_file_path: String,
    source: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    bucket_years: i32,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    source_sys_code: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "source: {}", self.source());
        info!(logger, "Bucket years: {}", self.bucket_years());
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
        info!(logger, "source_sys_code: {}", self.source_sys_code());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error while getting `input file path`.")
            .to_string();
        let source = matches
            .value_of("source")
            .expect("Error while getting `source name`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error while getting `output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error while getting `log file path.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting as on date as DD-MM-YYYY."),
        );
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error while getting required fields file path.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error while getting `diagnostics file path`.")
            .to_string();
        let source_sys_code = matches
            .value_of("source_sys_code")
            .expect("Error while getting `source system code`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error while getting `account metadata file`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let bucket_years = matches
            .value_of("bucket_years")
            .expect("Error while getting `bucket_years`.")
            .to_string()
            .parse::<i32>()
            .unwrap_or(100);
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting `is perfect diagnostics`.")
            .parse::<bool>()
            .expect("Error while parsing `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            source,
            as_on_date,
            log_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            bucket_years,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            source_sys_code,
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
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn as_on_date(&self) -> String {
        self.as_on_date.format("%Y%m%d").to_string()
    }
    pub fn bucket_years(&self) -> i32 {
        self.bucket_years
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
    pub fn source_sys_code(&self) -> &str {
        &self.source_sys_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("ECL cashflow extractor.")
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
            Arg::with_name("source")
                .long("source")
                .value_name("Source")
                .help("File source name.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On Date")
                .help("The date for which the program has to be processed.")
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
            Arg::with_name("bucket_years")
                .long("bucket_years")
                .value_name("Bucket Years")
                .help("Number of Bucket Years")
                .default_value("100")
                .required(false)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("source_sys_code")
                .long("source-sys-code")
                .value_name("Source System Code")
                .help("Value of Source System Code.")
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
        .get_matches()
}
