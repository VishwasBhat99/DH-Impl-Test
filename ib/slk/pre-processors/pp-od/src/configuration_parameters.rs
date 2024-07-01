use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub input_file_path: String,
    pub npa_input_file_path: String,
    pub output_file_path: String,
    pub master_file_path: String,
    pub date_fields: String,
    pub header_rows: String,
    pub master_sheet_name: String,
    pub npa_sheet_name: String,
    pub repricing_file_path: String,
    pub repricing_sheet_name: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "npa_input_file_path: {}",
            self.npa_input_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "date_fields: {}", self.date_fields());
        info!(
            logger,
            "repricing_file_path: {}",
            self.repricing_file_path()
        );
        info!(
            logger,
            "repricing_sheet_name: {}",
            self.repricing_sheet_name()
        );
        info!(logger, "header_rows: {}", self.header_rows());
        info!(logger, "header_rows: {}", self.header_rows());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
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
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let npa_input_file_path = matches
            .value_of("npa_input_file")
            .expect("Error getting `npa_input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let master_file_path = matches
            .value_of("master_file")
            .expect("Error getting `master_file_path`.")
            .to_string();
        let date_fields = matches
            .value_of("date_fields")
            .expect("Error getting `date_fields`.")
            .to_string();
        let header_rows = matches
            .value_of("header_rows")
            .expect("Error getting `header_rows`.")
            .to_string();
        let master_sheet_name = matches
            .value_of("master_sheet_name")
            .expect("Error getting `master_sheet_name`.")
            .to_string();
        let npa_sheet_name = matches
            .value_of("npa_sheet_name")
            .expect("Error getting `npa_sheet_name`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let repricing_file_path = matches
            .value_of("repricing_file")
            .expect("Error getting `repricing_file_path`.")
            .to_string();
        let repricing_sheet_name = matches
            .value_of("repricing_sheet_name")
            .expect("Error getting `reprising_sheet_name`.")
            .to_string();
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            npa_input_file_path,
            output_file_path,
            master_file_path,
            date_fields,
            header_rows,
            master_sheet_name,
            npa_sheet_name,
            repricing_file_path,
            repricing_sheet_name,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn npa_input_file_path(&self) -> &str {
        &self.npa_input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn date_fields(&self) -> &str {
        &self.date_fields
    }
    pub fn header_rows(&self) -> &str {
        &self.header_rows
    }
    pub fn master_sheet_name(&self) -> &str {
        &self.master_sheet_name
    }
    pub fn npa_sheet_name(&self) -> &str {
        &self.npa_sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn repricing_file_path(&self) -> &str {
        &self.repricing_file_path
    }
    pub fn repricing_sheet_name(&self) -> &str {
        &self.repricing_sheet_name
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("Updated pp_od for IB/SLK")
        .author("Sachin-M <sachin.m@surya-soft.com>")
        .version("1.0.4382")
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
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to read Input.")
                .required(true)
        )
        .arg(
            Arg::new("npa_input_file")
                .long("npa-input-file")
                .value_name("NPA Input File Path")
                .help("Path to read NPA Input.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to write Output.")
                .required(true)
        )
        .arg(
            Arg::new("master_file")
                .long("master-file")
                .value_name("Master File Path")
                .help("Path to write Master.")
                .required(true)
        )
        .arg(
            Arg::new("date_fields")
                .long("date-fields")
                .value_name("Date fields")
                .help("Date Fields")
                .required(true)
        )
        .arg(
            Arg::new("header_rows")
                .long("header-rows")
                .value_name("Header rows")
                .help("Header rows")
                .required(true)
        )
        .arg(
            Arg::new("master_sheet_name")
                .long("master-sheet-name")
                .value_name(" Master Sheet Name")
                .help("Master Sheet Name")
                .required(true)
        )
        .arg(
            Arg::new("npa_sheet_name")
                .long("npa-sheet-name")
                .value_name("NPA Sheet Name")
                .help("NPA Sheet Name")
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
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::new("repricing_file")
                .long("repricing-file")
                .value_name("repricing_file_path")
                .help("Path to recricing file")
                .required(true)
        )
        .arg(
            Arg::new("repricing_sheet_name")
                .long("repricing-sheet-name")
                .value_name("repricing_sheet_name")
                .help("Sheet Name of Repricing File")
                .default_value("Sheet1")
                .required(false)
        )
        .get_matches()
}
