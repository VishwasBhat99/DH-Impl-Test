use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub config_file: String,
    pub config_sheet_name: String,
    pub as_on_date: NaiveDate,
    pub output_file: String,
    pub concat_fields: Vec<usize>,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_exclude_overdue_interest_cashflow: bool,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "config_file: {}", self.config_file());
        info!(logger, "config_sheet_name: {}", self.config_sheet_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "concat_fields: {:?}", self.concat_fields());
        info!(
            logger,
            "is_exclude_overdue_interest_cashflow: {:?}",
            self.is_exclude_overdue_interest_cashflow()
        );
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();
        let config_file = matches
            .value_of("config_file")
            .expect("Error getting `config_file` value.")
            .to_string();
        let config_sheet_name = matches
            .value_of("config_sheet_name")
            .expect("Error getting `config_sheet_name` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let concat_fields: Vec<usize> = matches
            .value_of("concat_fields")
            .expect("Error getting `concat_fields`.")
            .to_string()
            .split(',')
            .map(|s| {
                s.parse()
                    .expect("Could not read the concat fields as numbers.")
            })
            .collect();
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
        let is_exclude_overdue_interest_cashflow = matches
            .value_of("is_exclude_overdue_interest_cashflow")
            .expect("Error getting `is_exclude_overdue_interest_cashflow` value.")
            .parse::<bool>()
            .expect("Cannot parse `is_exclude_overdue_interest_cashflow` value as bool.");
        ConfigurationParameters {
            input_file_path,
            config_file,
            config_sheet_name,
            as_on_date,
            output_file,
            concat_fields,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_exclude_overdue_interest_cashflow,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn config_file(&self) -> &str {
        &self.config_file
    }
    pub fn config_sheet_name(&self) -> &str {
        &self.config_sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn concat_fields(&self) -> &Vec<usize> {
        &self.concat_fields
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
    pub fn is_exclude_overdue_interest_cashflow(&self) -> bool {
        self.is_exclude_overdue_interest_cashflow
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Merger program for LST files.")
        .version("1.0.4149")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("INPUT FILE PATH")
                .help("Path to all input files.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .value_name("CONFIG FILE")
                .help("Path to the config file.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_sheet_name")
                .long("config-sheet-name")
                .value_name("CONFIG SHEET NAME")
                .help("Config File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("OUTPUT FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("concat_fields")
                .long("concat-fields")
                .value_name("CONCAT FIELDS")
                .help("Column numbers of the fields to be concatenated.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("LOG FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAG LOG FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
            Arg::with_name("is_exclude_overdue_interest_cashflow")
                .long("is-exclude-overdue-int-cf")
                .value_name("IS EXCLUDE OVERDUE INTEREST FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to exclude cashflows with cf date less than or equal to asondate.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name(" AS ON DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
