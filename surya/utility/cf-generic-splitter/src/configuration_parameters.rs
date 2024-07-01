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
    source_map_file_path: String,
    rule_file_path: String,
    metadata_file_path: String,
    dates_pos: String,
    default_llg_code: i32,
    default_file_name: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(
            logger,
            "source_map_file_path: {}",
            self.source_map_file_path()
        );
        info!(logger, "rule_file_path: {}", self.rule_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "default_file_name: {}", self.default_file_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "dates_pos: {}", self.dates_pos());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let source_map_file_path = matches
            .value_of("source_map_file_path")
            .expect("Error getting `source_map_file_path`.")
            .to_string();
        let rule_file_path = matches
            .value_of("rule_file_path")
            .expect("Error getting `rule_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let dates_pos = matches
            .value_of("dates_pos")
            .expect("Error getting `dates_pos`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error getting `default_llg_code`.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_llg_code` as i32.");
        let default_file_name = matches
            .value_of("default_file_name")
            .expect("Error getting `default_file_name`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting as on date as DD-MM-YYYY."),
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
            input_file_path,
            output_file_path,
            source_map_file_path,
            rule_file_path,
            metadata_file_path,
            default_llg_code,
            default_file_name,
            dates_pos,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn source_map_file_path(&self) -> &str {
        &self.source_map_file_path
    }
    pub fn rule_file_path(&self) -> &str {
        &self.rule_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn dates_pos(&self) -> &str {
        &self.dates_pos
    }
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn default_file_name(&self) -> &str {
        &self.default_file_name
    }
    pub fn as_on_date(&self) -> String {
        self.as_on_date.format("%Y%m%d").to_string()
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
        .about("A generic program to split input cf file accounts based on source code from rule file.")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .value_name("input file path")
                .help("Path to input file")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("output file path")
                .help("Path to output file")
                .required(true)
        )
        .arg(
            Arg::with_name("source_map_file_path")
                .long("source-map-file-path")
                .value_name("source map file path")
                .help("Path to source map file")
                .required(true)
        )
        .arg(
            Arg::with_name("rule_file_path")
                .long("rule-file-path")
                .value_name("rule file path")
                .help("Path to rule file")
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
            Arg::with_name("dates_pos")
                .long("dates-pos")
                .value_name("date field positions")
                .help("position of date fields in input.")
                .required(true)
        )
        .arg(
            Arg::new("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .arg(
            Arg::new("default_file_name")
                .long("default-file-name")
                .value_name("Default file name")
                .help("Path to default file name.")
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
