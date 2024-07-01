use clap;
use clap::{Arg, Command};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file: String,
    output_sheet_name: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    date_formats: Vec<String>,
    date_fields: Vec<String>,
    skip_header: bool,
    field_separator: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "output_sheet_name: {}", self.output_sheet_name());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "skip_header: {}", self.skip_header());
        info!(logger, "date_formats: {:?}", self.date_formats());
        info!(logger, "date_fields: {:?}", self.date_fields());
        info!(logger, "field_separator: {}", self.field_separator());
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
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let date_formats: Vec<String> = matches
            .value_of("date_formats")
            .expect("Error getting `date_formats`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let date_fields: Vec<String> = matches
            .value_of("date_fields")
            .expect("Error getting `date_fields`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let field_separator = matches
            .value_of("field_separator")
            .expect("Error getting `field_separator`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let skip_header = matches
            .value_of("skip_header")
            .expect("Error getting `skip_header`.")
            .parse::<bool>()
            .expect("Cannot parse `skip_header` as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let output_sheet_name = matches
            .value_of("output_sheet_name")
            .expect("Error getting `output_sheet_name`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file,
            output_file_path,
            field_separator,
            as_on_date,
            skip_header,
            date_formats,
            date_fields,
            output_sheet_name,
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
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn output_sheet_name(&self) -> &str {
        &self.output_sheet_name
    }
    pub fn skip_header(&self) -> bool {
        self.skip_header
    }
    pub fn field_separator(&self) -> &str {
        &self.field_separator
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn date_formats(&self) -> &Vec<String> {
        &self.date_formats
    }
    pub fn date_fields(&self) -> &Vec<String> {
        &self.date_fields
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
    Command::new(app_name)
        .about("Program for converting txt/csv files to excel files with some options!!")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .version("1.0.0")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Input file path.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("output File Path")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::new("skip_header")
                .long("skip-header")
                .value_name("Skip Header")
                .possible_values(&["true", "false"])
                .help("Flag which tells whether to Skip a header in input/output.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::new("date_formats")
                .long("date-formats")
                .value_name("Formats of dates")
                .help("The date formats in which input should be read and output should be written.")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::new("date_fields")
                .long("date-fields")
                .value_name("Date Fields")
                .help("The field columns in which date of format dd/mm/yyyy found in input.")
                .default_value("%d-%m-%Y,%d-%m-%Y")
                .required(false)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("output_sheet_name")
                .long("output-sheet-name")
                .value_name("Output File Sheet Name.")
                .help("Output File Sheet Name.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("field_separator")
                .long("field-separator")
                .value_name("Input file field separator")
                .help("Separator between the fields in the input file.")
                .default_value("|")
                .required(false)
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
        .get_matches()
}
