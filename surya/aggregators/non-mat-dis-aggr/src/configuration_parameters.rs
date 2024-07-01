use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub seasonal_dis_rules_path: String,
    pub distribution_rules_path: String,
    pub input_file_path: String,
    pub master_file_path: String,
    pub is_seasonal: bool,
    pub from_bucket: String,
    pub to_bucket: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.seasonal_dis_rules_path());
        info!(
            logger,
            "distribution_rules_path: {}",
            self.distribution_rules_path()
        );
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "is_seasonal: {}", self.is_seasonal());
        info!(logger, "from_bucket: {}", self.from_bucket());
        info!(logger, "to_bucket: {}", self.to_bucket());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let seasonal_dis_rules_path = matches
            .value_of("seasonal_dis_rules_path")
            .expect("Error getting `input_file` value.")
            .to_string();
        let distribution_rules_path: String = matches
            .value_of("distribution_rules_path")
            .expect("Error getting `distribution_rules_path` value.")
            .to_string();
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();
        let master_file_path = matches
            .value_of("master_file_path")
            .expect("Error getting `master_file_path` value.")
            .to_string();
        let is_seasonal = matches
            .value_of("is_seasonal")
            .expect("Error getting `is_seasonal` value.")
            .parse::<bool>()
            .expect("Cannot parse `adj_cf_flag` as bool.");
        let from_bucket = matches
            .value_of("from_bucket")
            .expect("Error getting `from_bucket` value.")
            .to_string();
        let to_bucket = matches
            .value_of("to_bucket")
            .expect("Error getting `to_bucket` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");

        ConfigurationParameters {
            seasonal_dis_rules_path,
            distribution_rules_path,
            input_file_path,
            master_file_path,
            is_seasonal,
            from_bucket,
            to_bucket,
            as_on_date,
            output_file_path,
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
    pub fn seasonal_dis_rules_path(&self) -> &str {
        &self.seasonal_dis_rules_path
    }
    pub fn distribution_rules_path(&self) -> &str {
        &self.distribution_rules_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn is_seasonal(&self) -> &bool {
        &self.is_seasonal
    }
    pub fn from_bucket(&self) -> &str {
        &self.from_bucket
    }
    pub fn to_bucket(&self) -> &str {
        &self.to_bucket
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
        .about("This app aggregates seasonal and non seasonal products using distribution aggregation rules.")
        .version("1.0.3765")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("seasonal_dis_rules_path")
                .long("seasonal-dis-rules-path")
                .value_name("seasonal_dis_rules_path")
                .help("Path to Rules file 1.")
                .required(true)
        )
        .arg(
            Arg::with_name("distribution_rules_path")
                .long("distribution-rules-path")
                .value_name("distribution_rules_path")
                .help("Path to Rules file 2.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("input_file_path")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("master_file_path")
                .long("master-file-path")
                .value_name("master_file_path")
                .help("Path to master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("is_seasonal")
                .long("is-seasonal")
                .value_name("is_seasonal")
                .help("Flag to determine seasonal or non-seasonal product.")
                .required(true)
        )
        .arg(
            Arg::with_name("from_bucket")
                .long("from-bucket")
                .value_name("from_bucket")
                .help("First Bucket ID for which percentage is to be considered.")
                .required(true)
        )
        .arg(
            Arg::with_name("to_bucket")
                .long("to-bucket")
                .value_name("to_bucket")
                .help("Last Bucket ID for which percentage is to be considered.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output_file")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("log_file")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("diagnostics_file")
                .help("Path to write diagnostics log.")
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
                .value_name("as_on_date")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
