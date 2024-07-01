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
    input_file: String,
    master_llg_updated: String,
    exchange_rate_file: String,
    base_currency: String,
    alm_master_sheet_name: String,
    output_file_path: String,
    req_fields_file_path: String,
    metadata_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "master_llg_updated: {}", self.master_llg_updated());
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "req_fields_file: {}", self.req_fields_file_path());
        info!(logger, "metadata_file: {}", self.metadata_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file`.")
            .to_string();
        let master_llg_updated = matches
            .value_of("master_llg_updated")
            .expect("Error getting `master_llg_updated`.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
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
            input_file,
            master_llg_updated,
            exchange_rate_file,
            alm_master_sheet_name,
            base_currency,
            output_file_path,
            req_fields_file_path,
            metadata_file_path,
            as_on_date,
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
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn master_llg_updated(&self) -> &str {
        &self.master_llg_updated
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
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
        .about("Program For src recon loader")
        .version("1.0.2643")
        .author("Rahul Gangwar <rahul.gangwar@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("master_llg_updated")
                .long("master-file")
                .value_name("Master File")
                .help("Master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("alm_master_sheet_name")
                .help("Alm Master File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("Exchange Rate File")
                .help("Exchange Rate file path.")
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
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output File Path")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file")
                .long("metadata-file")
                .value_name("Metadata File Path")
                .help("Path to metadata file.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("Input Req fields File Path")
                .help("Path to input req fields file.")
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
        .get_matches()
}
