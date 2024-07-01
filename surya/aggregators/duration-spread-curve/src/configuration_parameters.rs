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
    balm_rating_file: String,
    llg_to_spread_mapper_file: String,
    spread_rate_file: String,
    req_fields_file_path: String,
    output_file_path: String,
    metadata_file_path: String,
    rules_file_path: String,
    as_on_date: NaiveDate,
    default_values_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_interpol_low_ver_req: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "balm_rating_file: {}", self.balm_rating_file());
        info!(
            logger,
            "llg_to_spread_mapper_file: {}",
            self.llg_to_spread_mapper_file()
        );
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );

        info!(logger, "spread_rate_file: {}", self.spread_rate_file());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "default_values_file: {}",
            self.default_values_file()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "is_perf_diagnostics_enabled: {}",
            self.is_perf_diagnostics_enabled()
        );
        info!(
            logger,
            "is_interpol_low_ver_req: {}",
            self.is_interpol_low_ver_req()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let balm_rating_file = matches
            .value_of("balm_rating_file")
            .expect("Error getting `balm_rating_file`.")
            .to_string();
        let llg_to_spread_mapper_file = matches
            .value_of("llg_to_spread_mapper_file")
            .expect("Error getting `llg_to_spread_mapper_file`.")
            .to_string();
        let spread_rate_file = matches
            .value_of("spread_rate_file")
            .expect("Error getting `spread_rate_file`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `req_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `account metadata file path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error getting `rules_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let default_values_file = matches
            .value_of("default_values_file")
            .expect("Error getting `default_values_file`.")
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
        let is_interpol_low_ver_req = matches
            .value_of("is_interpol_low_ver_req")
            .expect("Error getting interpolation flag")
            .parse::<bool>()
            .expect("Cannot parse `is_interpol_low_ver_req` as bool.");

        ConfigurationParameters {
            input_file_path,
            balm_rating_file,
            llg_to_spread_mapper_file,
            spread_rate_file,
            rules_file_path,
            req_fields_file_path,
            output_file_path,
            metadata_file_path,
            as_on_date,
            default_values_file,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_interpol_low_ver_req,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn balm_rating_file(&self) -> &str {
        &self.balm_rating_file
    }
    pub fn llg_to_spread_mapper_file(&self) -> &str {
        &self.llg_to_spread_mapper_file
    }
    pub fn spread_rate_file(&self) -> &str {
        &self.spread_rate_file
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
    pub fn default_values_file(&self) -> &str {
        &self.default_values_file
    }
    pub fn is_interpol_low_ver_req(&self) -> bool {
        self.is_interpol_low_ver_req
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Weighted Spread Rate Calculator/Aggregator")
        .version("1.0.4224")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("input_file_path")
                .long("input-file-path")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::new("balm_rating_file")
                .long("balm-rating-file")
                .value_name("Balm Rating File")
                .help("Path to the balm rating file.")
                .required(true)
        )
        .arg(
            Arg::new("llg_to_spread_mapper_file")
                .long("llg-to-spread-mapper-file")
                .value_name("LLG to Spread Mapper File")
                .help("Path to the LLG to spread mapper file.")
                .required(true)
        )
        .arg(
            Arg::new("spread_rate_file")
                .long("spread-rate-file")
                .value_name("Spread Rate File")
                .help("Path to the spread rate file.")
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
            Arg::new("rules_file_path")
                .long("rules-file-path")
                .value_name("Rules File")
                .help("Path to the rules file.")
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
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("default_values_file")
                .long("default-values-file")
                .value_name("DEFAULT VALUES FILE")
                .help("Path to Default Values File.")
                .required(true)
        )
        .arg(
            Arg::new("req_fields_file")
                .long("req-fields-file")
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
        .arg(
            Arg::new("is_interpol_low_ver_req")
                .long("is-interpol-low-ver-req")
                .possible_values(["true", "false"])
                .help("This flag that decides whether interpolation till lower vertex required or not")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
