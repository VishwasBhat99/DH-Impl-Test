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
    pub input_file_path: String,
    pub ccf_file_path: String,
    pub rw_file_path: String,
    pub exfields_file_path: String,
    pub input_sheet_name: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub metadata_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    fields_with_date: Vec<String>,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "metadata_file: {}", self.metadata_file_path());
        info!(logger, "ccf_file: {}", self.ccf_file_path());
        info!(logger, "rw_file: {}", self.rw_file_path());
        info!(logger, "exfields_file: {}", self.exfields_file_path());
        info!(logger, "input_sheet_name: {}", self.input_sheet_name());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "fields_with_date: {:?}", self.fields_with_date());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let fields_with_date: Vec<String> = matches
            .value_of("fields_with_date")
            .expect("Error getting `fields_with_date`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let metadata_path = matches
            .value_of("metadata_file")
            .expect("Error getting `metadata_file` value.")
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
        let rw_file_path = matches
            .value_of("rw_file")
            .expect("Error getting `rw_file` value.")
            .to_string();
        let ccf_file_path = matches
            .value_of("ccf_file")
            .expect("Error getting `ccf_file` value.")
            .to_string();
        let exfields_file_path = matches
            .value_of("exfields_file")
            .expect("Error getting `exfields_file` value.")
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
        let input_sheet_name = matches
            .value_of("input_sheet_name")
            .expect("Error getting `input_sheet_name` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            ccf_file_path,
            rw_file_path,
            exfields_file_path,
            input_sheet_name,
            as_on_date,
            output_file_path,
            metadata_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            fields_with_date,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_path
    }
    pub fn fields_with_date(&self) -> &Vec<String> {
        &self.fields_with_date
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn ccf_file_path(&self) -> &str {
        &self.ccf_file_path
    }
    pub fn rw_file_path(&self) -> &str {
        &self.rw_file_path
    }
    pub fn exfields_file_path(&self) -> &str {
        &self.exfields_file_path
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
    pub fn input_sheet_name(&self) -> &str {
        &self.input_sheet_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of GL CFGen!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file")
                .long("metadata-file")
                .value_name("Metadata File")
                .help("Path to Metadata file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("fields_with_date")
                .long("fields-with-date")
                .value_name("DATE FIELDS")
                .help("The field columns which have date.")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::with_name("rw_file")
                .long("rw-file")
                .value_name("RW File")
                .help("Path to RW file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ccf_file")
                .long("ccf-file")
                .value_name("CCF File")
                .help("Path to CCF file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("exfields_file")
                .long("exfields-file")
                .value_name("exfields File")
                .help("Path to exfields file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Diagnostics log file path")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sheet_name")
                .long("input-sheet-name")
                .value_name("InputSheet name")
                .help("Path to input Sheet name that needs to be processed.")
                .required(true)
        )
        .get_matches()
}
