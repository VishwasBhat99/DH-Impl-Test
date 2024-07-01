use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub ason_date: NaiveDate,
    pub currency_id: String,
    pub input_file: String,
    pub ucic_map_file: String,
    pub output_file: String,
    pub exrt_file: String,
    pub log_file: String,
    pub diag_log_file: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub country_code: String,
    pub top_cust_count: u32,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "ason_date: {}", self.ason_date());
        info!(logger, "currency_id: {}", self.currency_id());
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "ucic_map_file: {}", self.ucic_map_file());
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "log_file: {}", self.log_file());
        info!(logger, "diag_log_file: {}", self.diag_log_file());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "exrt_file: {}", self.exrt_file());
        info!(logger, "country_code: {}", self.country_code());
        info!(logger, "top_cust_count: {}", self.top_cust_count());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let ason_date = date_parser.parse(
            matches
                .value_of("ason")
                .expect("Error getting `ason_date` value."),
        );
        let country_code = matches
            .value_of("country_code")
            .expect("Error in getting `country_code` value.")
            .to_string();
        let currency_id = matches
            .value_of("currency_id")
            .expect("Error in getting `currency_id` value.")
            .to_string();
        let input_file = matches
            .value_of("input_file")
            .expect("Error in getting `input_file` value.")
            .to_string();
        let ucic_map_file = matches
            .value_of("ucic_map_file")
            .expect("Error in getting `ucic_map_file` value.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error in getting `output_file` value.")
            .to_string();
        let log_file = matches
            .value_of("log_file")
            .expect("Error in getting `log_file` value.")
            .to_string();
        let diag_log_file = matches
            .value_of("diag_log_file")
            .expect("Error in getting `diag_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error in getting `log_level` value.")
            .to_string();
        let exrt_file = matches
            .value_of("exrt_file")
            .expect("Error in getting `exrt_file` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error in getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let top_cust_count = matches
            .value_of("top_cust_count")
            .expect("Error in getting `top_cust_count` value.")
            .parse::<u32>()
            .expect("Cannot parse `top_cust_count` value as u32.");
        ConfigurationParameters {
            ason_date,
            currency_id,
            input_file,
            ucic_map_file,
            output_file,
            exrt_file,
            log_file,
            diag_log_file,
            log_level,
            is_perf_diagnostics_enabled,
            country_code,
            top_cust_count,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn ason_date(&self) -> &NaiveDate {
        &self.ason_date
    }
    pub fn exrt_file(&self) -> &str {
        &self.exrt_file
    }
    pub fn currency_id(&self) -> &str {
        &self.currency_id
    }
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn ucic_map_file(&self) -> &str {
        &self.ucic_map_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn log_file(&self) -> &str {
        &self.log_file
    }
    pub fn diag_log_file(&self) -> &str {
        &self.diag_log_file
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn country_code(&self) -> &str {
        &self.country_code
    }
    pub fn top_cust_count(&self) -> &u32 {
        &self.top_cust_count
    }
}

pub fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .author("Sougata Bhattacharjee. <sougata.b@surya-soft.com>")
        .about("Program for prepocessing the deposits file")
        .arg(
            Arg::with_name("ason")
                .long("ason")
                .help("ason date for processing")
                .value_name("Ason Date")
                .required(true)
        )
        .arg(
            Arg::with_name("exrt_file")
                .long("exrt_file")
                .help("path to exchange rate file")
                .value_name("Exchange Rate file")
                .required(true)
        )
        .arg(
            Arg::with_name("currency_id")
                .long("currency_id")
                .help("currency id for processing")
                .value_name("Currency id")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .long("input_file")
                .help("Deposits data file path")
                .value_name("Deposits File")
                .required(true)
        )
        .arg(
            Arg::with_name("ucic_map_file")
                .long("ucic_map_file")
                .help("ucic to customer id mapping file")
                .value_name("Ucic Mapping File")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output_file")
                .help("Output file path")
                .value_name("Output File")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log_file")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diag_log_file")
                .long("diag_log_file")
                .value_name("Diagnostic Log File")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("country_code")
                .long("country_code")
                .value_name("country_code")
                .help("country_code.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log_level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics_flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("top_cust_count")
                .long("top_cust_count")
                .value_name("Top Cust Count")
                .help("Top Cust Count Value")
                .required(true)
        )
        .get_matches()
}
