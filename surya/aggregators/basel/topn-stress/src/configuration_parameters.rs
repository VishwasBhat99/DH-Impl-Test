use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    retail_input_file_path: String,
    non_retail_input_file_path: String,
    topn_dep_file_path: String,
    class_llg_mapper_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    exrt_rate_file_path: String,
    ccy_id: String,
    default_llg: String,
    country_id: String,
    stress_app_type: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "retail_input_file_path: {}",
            self.retail_input_file_path()
        );
        info!(
            logger,
            "non_retail_input_file_path: {}",
            self.non_retail_input_file_path()
        );
        info!(logger, "topn_dep_file_path: {}", self.topn_dep_file_path());
        info!(
            logger,
            "class_llg_mapper_file_path: {}",
            self.class_llg_mapper_file_path()
        );
        info!(logger, "ccy_id: {}", self.ccy_id());
        info!(logger, "default_llg: {}", self.default_llg());
        info!(logger, "country_id: {}", self.country_id());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "stress_app_type: {}", self.stress_app_type());
        info!(
            logger,
            "exrt_rate_file_path: {}",
            self.exrt_rate_file_path()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
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
        let retail_input_file_path = matches
            .value_of("retail_input_file_path")
            .expect("Error getting `retail_input_file_path`.")
            .to_string();
        let non_retail_input_file_path = matches
            .value_of("non_retail_input_file_path")
            .expect("Error getting `non_retail_input_file_path`.")
            .to_string();
        let topn_dep_file_path = matches
            .value_of("topn_dep_file_path")
            .expect("Error getting `topn_dep_file_path`.")
            .to_string();
        let class_llg_mapper_file_path = matches
            .value_of("class_llg_mapper_file_path")
            .expect("Error getting `class_llg_mapper_file_path`.")
            .to_string();
        let ccy_id = matches
            .value_of("ccy_id")
            .expect("Error getting `ccy_id`.")
            .to_string();
        let country_id = matches
            .value_of("country_id")
            .expect("Error getting `country_id`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let exrt_rate_file_path = matches
            .value_of("exrt_rate_file_path")
            .expect("Error getting `exrt_rate_file_path`.")
            .to_string();
        let stress_app_type = matches
            .value_of("stress_app_type")
            .expect("Error getting `stress_app_type`.")
            .to_string();
        let default_llg = matches
            .value_of("default_llg")
            .expect("Error getting `default_llg`.")
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
            retail_input_file_path,
            non_retail_input_file_path,
            default_llg,
            topn_dep_file_path,
            class_llg_mapper_file_path,
            stress_app_type,
            ccy_id,
            country_id,
            output_file_path,
            exrt_rate_file_path,
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
    pub fn retail_input_file_path(&self) -> &str {
        &self.retail_input_file_path
    }
    pub fn non_retail_input_file_path(&self) -> &str {
        &self.non_retail_input_file_path
    }
    pub fn topn_dep_file_path(&self) -> &str {
        &self.topn_dep_file_path
    }
    pub fn class_llg_mapper_file_path(&self) -> &str {
        &self.class_llg_mapper_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn exrt_rate_file_path(&self) -> &str {
        &self.exrt_rate_file_path
    }
    pub fn default_llg(&self) -> &str {
        &self.default_llg
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn stress_app_type(&self) -> &str {
        &self.stress_app_type
    }
    pub fn ccy_id(&self) -> &str {
        &self.ccy_id
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
    pub fn country_id(&self) -> &str {
        &self.country_id
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Basel TopN Stress Testing Program")
        .version("1.0.4036")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("retail_input_file_path")
                .long("retail-input-file")
                .value_name("Retail Input File Path")
                .help("Path to the retail input file.")
                .required(true)
        )
        .arg(
            Arg::new("non_retail_input_file_path")
                .long("non-retail-input-file")
                .value_name("Non Retail Input File Path")
                .help("Path to the non retail input file.")
                .required(true)
        )
        .arg(
            Arg::new("class_llg_mapper_file_path")
                .long("class-llg-mapper-file")
                .value_name("Class LLG Mapper File")
                .help("Path to the Class LLG mapper file.")
                .required(true)
        )
        .arg(
            Arg::new("topn_dep_file_path")
                .long("topn-dep-file")
                .value_name("TopN Dep File")
                .help("Path to the topn deposits file.")
                .required(false)
        )
        .arg(
            Arg::new("output_file_path")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("exrt_rate_file_path")
                .long("exrt-rate-file")
                .value_name("Exchange Rate File")
                .help("Path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::new("default_llg")
                .long("default-llg")
                .value_name("Default LLG")
                .help("LLGID to be stamped for default cases.")
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
            Arg::new("ccy_id")
                .long("ccy-id")
                .value_name("Currency ID")
                .help("Currency ID to be stamped.")
                .required(true)
        )
        .arg(
            Arg::new("country_id")
                .long("country-id")
                .value_name("Country ID")
                .help("Country Code to be stamped.")
                .required(true)
        )
        .arg(
            Arg::new("stress_app_type")
                .long("stress-app-type")
                .value_name("Stress App Type")
                .help("Stress App Type to be stamped.")
                .possible_values(["BANK","CLASS"])
                .default_value("BANK")
                .required(false)
        )
        .get_matches()
}
