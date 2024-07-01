use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_cf_file_path: String,
    metadata_file_path: String,
    req_fields_file_path: String,
    vp_npa_file_path: String,
    od_study_master_file_path: String,
    od_study_master_sheet_name: String,
    exchange_rate_file_path: String,
    is_consolidated: bool,
    base_currency: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    alm_line: Vec<String>,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "vp_npa_file_path: {}", self.vp_npa_file_path());
        info!(
            logger,
            "od_study_master_file_path: {}",
            self.od_study_master_file_path()
        );
        info!(
            logger,
            "od_study_master_sheet_name: {}",
            self.od_study_master_sheet_name()
        );
        info!(
            logger,
            "exchange_rate_file_path: {}",
            self.exchange_rate_file_path()
        );
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_cf_file: {}", self.input_cf_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "alm_line: {:?}", self.alm_line());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_cf_file_path = matches
            .value_of("input_cf_file_path")
            .expect("Error getting `input_cf_file_value`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );

        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let od_study_master_file_path = matches
            .value_of("od_study_master_file_path")
            .expect("Error getting `od_study_master_file_path`.")
            .to_string();
        let od_study_master_sheet_name = matches
            .value_of("od_study_master_sheet_name")
            .expect("Error getting `od_study_master_sheet_name`.")
            .to_string();
        let vp_npa_file_path = matches
            .value_of("vp_npa_file_path")
            .expect("Error getting `vp_npa_file_path`.")
            .to_string();
        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file_path")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated`.")
            .parse::<bool>()
            .expect("Cannot parse `is_consolidated` as bool.");
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let alm_line = matches
            .value_of("alm_line")
            .expect("Error getting `alm_line`.")
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
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

        ConfigurationParameters {
            input_cf_file_path,
            metadata_file_path,
            req_fields_file_path,
            vp_npa_file_path,
            od_study_master_file_path,
            od_study_master_sheet_name,
            exchange_rate_file_path,
            is_consolidated,
            base_currency,
            as_on_date,
            output_file_path,
            alm_line,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// pub lic getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_cf_file_path(&self) -> &str {
        &self.input_cf_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn vp_npa_file_path(&self) -> &str {
        &self.vp_npa_file_path
    }
    pub fn od_study_master_file_path(&self) -> &str {
        &self.od_study_master_file_path
    }
    pub fn od_study_master_sheet_name(&self) -> &str {
        &self.od_study_master_sheet_name
    }
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
    }
    pub fn is_consolidated(&self) -> &bool {
        &self.is_consolidated
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn alm_line(&self) -> &Vec<String> {
        &self.alm_line
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
        .about("Cashflow generation for Term deposits!!")
        .author("Ravindar Singh <ravindar.sr@surya-soft.com>")
        .version("1.0.3721")
        .arg(
            Arg::with_name("input_cf_file_path")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file_path")
                .long("metadata-file-path")
                .value_name("Metadata File Path")
                .help("Path to the metadata file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file-path")
                .value_name("Required Fields File Path")
                .help("Path to the required fields file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("vp_npa_file_path")
                .long("vp-npa-master-file-path")
                .value_name("VP Npa Master File Path")
                .help("Path to the VP Npa master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("od_study_master_file_path")
                .long("od-study-master-file-path")
                .value_name("od_study Master File Path")
                .help("Path to the od_study master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("od_study_master_sheet_name")
                .long("od-study-master-sheet-name")
                .value_name("od_study Master sheet name")
                .help("Path to the od_study master sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file_path")
                .long("exchange-rate-file-path")
                .value_name("exchange rate file path")
                .help("Path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("Is Consolidated")
                .help("Is Consolidated flag.")
                .possible_values(&["true","false"])
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("Base Currency")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_line")
                .long("alm-line")
                .value_name("Alm Line")
                .help("Alm Line")
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
        .get_matches()
}
