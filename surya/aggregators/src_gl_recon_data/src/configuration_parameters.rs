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
    llg_recon_file_path: String,
    alm_master_file_path: String,
    alm_master_sheet_name: String,
    gl_master_sheet_name: String,
    req_fields_file_path: String,
    exchange_rate_file_path: String,
    gl_master_file_path: String,
    output_file_path: String,
    metadata_file_path: String,
    as_on_date: NaiveDate,
    base_currency: String,
    is_consolidated: bool,
    default_gl_code: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "llg_recon_file_path: {}",
            self.llg_recon_file_path()
        );
        info!(
            logger,
            "alm_master_file_path: {}",
            self.alm_master_file_path()
        );
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(
            logger,
            "gl_master_sheet_name: {}",
            self.gl_master_sheet_name()
        );

        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(
            logger,
            "exchange_rate_file_path: {}",
            self.exchange_rate_file_path()
        );
        info!(
            logger,
            "gl_master_file_path: {}",
            self.gl_master_file_path()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "default_gl_code: {}", self.default_gl_code());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "is_perf_diagnostics_enabled: {}",
            self.is_perf_diagnostics_enabled()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let llg_recon_file_path = matches
            .value_of("llg_recon_file_path")
            .expect("Error getting `llg_recon_file_path`.")
            .to_string();
        let alm_master_file_path = matches
            .value_of("alm_master_file_path")
            .expect("Error getting `alm_master_file_path`.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name`.")
            .to_string();
        let gl_master_sheet_name = matches
            .value_of("gl_master_sheet_name")
            .expect("Error getting `gl_master_sheet_name`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `req_file_path`.")
            .to_string();
        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let gl_master_file_path = matches
            .value_of("gl_master_file")
            .expect("Error getting `gl_master_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `account metadata file path`.")
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
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let default_gl_code = matches
            .value_of("default_gl_code")
            .expect("Error getting `default_gl_code`.")
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
            input_file_path,
            llg_recon_file_path,
            alm_master_file_path,
            alm_master_sheet_name,
            gl_master_sheet_name,
            req_fields_file_path,
            exchange_rate_file_path,
            gl_master_file_path,
            output_file_path,
            metadata_file_path,
            as_on_date,
            base_currency,
            is_consolidated,
            default_gl_code,
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
    pub fn llg_recon_file_path(&self) -> &str {
        &self.llg_recon_file_path
    }
    pub fn alm_master_file_path(&self) -> &str {
        &self.alm_master_file_path
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn gl_master_sheet_name(&self) -> &str {
        &self.gl_master_sheet_name
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
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
    }
    pub fn gl_master_file_path(&self) -> &str {
        &self.gl_master_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
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
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn default_gl_code(&self) -> &str {
        &self.default_gl_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("src_gl_recon_data")
        .version("1.0.3167")
        .arg(
            Arg::new("input_file_path")
                .long("input-file-path")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::new("llg_recon_file_path")
                .long("llg-recon-file-path")
                .value_name("LLG GL Recon File Path")
                .help("Path to the LLG GL Recon File.")
                .required(true)
        )
        .arg(
            Arg::new("alm_master_file_path")
                .long("alm-master-file-path")
                .value_name("ALM Master File Path")
                .help("Path to the ALM Master File.")
                .required(true)
        )
        .arg(
            Arg::new("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("ALM Master Sheet Name")
                .help("ALM Master Sheet Name.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("gl_master_sheet_name")
                .long("gl-master-sheet-name")
                .value_name("GL Master Sheet Name")
                .help("GL Master Sheet Name.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("output_file_path")
                .long("output-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("is_consolidated")
                .long("is-consolidated")
                .value_name("IS CONSOLIDATED")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether amount in input is consolidated or not.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::new("default_gl_code")
                .long("default-gl-code")
                .value_name("DEFAULT GL CODE")
                .help("This flag that decides whether amount in input default gl code or not.")
                .default_value("999999")
                .required(false)
        )
        .arg(
            Arg::new("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::new("gl_master_file")
                .long("gl-master-file-path")
                .value_name("GL MASTER FILE")
                .help("The path to the GL MASTER file.")
                .required(false)
                .default_value("NA")
        )
        .arg(
            Arg::new("base_currency")
                .long("base-currency")
                .value_name("BASECURRENCY")
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::new("req_fields_file")
                .long("req-fields-file-path")
                .value_name("REQUIRED_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::new("metadata_file_path")
                .long("metadata-file-path")
                .value_name("METADATA")
                .help("The aggregator requires metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .get_matches()
}
