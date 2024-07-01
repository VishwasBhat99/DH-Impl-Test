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
    skip_rows: Vec<String>,
    sheet_name: String,
    output_file_path: String,
    field_delimeter: String,
    append_required: bool,
    as_on_date: NaiveDate,
    fields_with_date: Vec<String>,
    log_file_path: String,
    diagnostics_file_path: String,
    default_date: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "skip_rows: {:?}", self.skip_rows());
        info!(logger, "sheet_name: {}", self.sheet_name());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "fields_with_date: {:?}", self.fields_with_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "field_delimeter: {}", self.field_delimeter());
        info!(logger, "append_required: {}", self.append_required());
        info!(logger, "default_date: {}", self.default_date());
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
        let fields_with_date: Vec<String> = matches
            .value_of("fields_with_date")
            .expect("Error getting `fields_with_date`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let default_date = matches
            .value_of("default_date")
            .expect("Error getting `default_date`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let field_delimeter = matches
            .value_of("field_delimeter")
            .expect("Error getting `field_delimeter`.")
            .to_string();
        let append_required = matches
            .value_of("append_required")
            .expect("Error getting `append_required`.")
            .parse::<bool>().expect("please give the correct value for append required possible values are true or false");
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name`.")
            .to_string();
        let skip_rows: Vec<String> = matches
            .value_of("skip_rows")
            .expect("Error getting `skip_rows`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file,
            skip_rows,
            output_file_path,
            default_date,
            as_on_date,
            fields_with_date,
            sheet_name,
            log_file_path,
            field_delimeter,
            append_required,
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
    pub fn skip_rows(&self) -> &Vec<String> {
        &self.skip_rows
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn field_delimeter(&self) -> &str {
        &self.field_delimeter
    }
    pub fn append_required(&self) -> &bool {
        &self.append_required
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn default_date(&self) -> &str {
        &self.default_date
    }
    pub fn fields_with_date(&self) -> &Vec<String> {
        &self.fields_with_date
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
        .about("Program for converting excel files to text files!!")
        .author("Ravindar Singh<ravindar.sr@surya-soft.com>")
        .version("1.0.4652")
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
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::new("default_date")
                .long("default-date")
                .value_name("DEF DATE")
                .help("The default date the program has to stamp in case of invalid values.")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::new("fields_with_date")
                .long("fields-with-date")
                .value_name("DATE FIELDS")
                .help("The field columns which have date.")
                .default_value("")
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
            Arg::new("sheet_name")
                .long("sheet-name")
                .value_name("Input File Sheet Name.")
                .help("Input File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("field_delimeter")
                .long("field-delimeter")
                .value_name("Field Delimeter")
                .help("Field Delimeter.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::new("append_required")
                .long("append-required")
                .value_name("append required")
                .help("Append Required.")
                .default_value("false")
                .possible_values(&["true", "false"])
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
            Arg::new("skip_rows")
                .long("skip-rows")
                .value_name("Skip Rows")
                .help("This value tells about the rows to be skipped from processing")
                .default_value("")
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
