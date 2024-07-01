use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    as_on_date: NaiveDate,
    db_config_file_path: String,
    rules_output_path: String,
    input_file_path: String,
    output_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "db_config_file: {}", self.db_config_file_path());
        info!(logger, "rules_output_path: {}", self.rules_output_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let db_config_file_path = matches
            .value_of("db_config_file")
            .expect("Error getting `db_config_file_path`.")
            .to_string();
        let rules_output_path = matches
            .value_of("rules_output_path")
            .expect("Error getting `rules_output_path`.")
            .to_string();
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `Input file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `Output file path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `Account metadata file path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error while getting rules file path.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error while getting `default llg code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default llg code` as integer.");
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
            as_on_date,
            db_config_file_path,
            rules_output_path,
            input_file_path,
            output_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn db_config_file_path(&self) -> &str {
        &self.db_config_file_path
    }
    pub fn rules_output_path(&self) -> &str {
        &self.rules_output_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
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
        .about("LRST Multiple Scenario HC Check Program!!")
        .version("1.0.0")
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("db_config_file")
                .long("dbconfig-file")
                .value_name("dB Config File Path")
                .help("Path to get Database config parameters")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_output_path")
                .long("rules-output-path")
                .value_name("Rules Output Path")
                .help("Path to store extracted files")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
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
