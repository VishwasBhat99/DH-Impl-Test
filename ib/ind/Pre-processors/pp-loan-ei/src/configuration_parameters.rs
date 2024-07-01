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
    pub master_file_path: String,
    pub npa_master_file_path: String,
    pub output_file_path: String,
    pub sheet_name: String,
    pub npa_sheet_name: String,
    pub date_fields: String,
    pub header_rows: String,
    pub next_rep_file: String,
    pub next_rep_sheet_name: String,
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
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(
            logger,
            "npa_master_file_path: {}",
            self.npa_master_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "sheet_name: {}", self.sheet_name());
        info!(logger, "npa_sheet_name: {}", self.npa_sheet_name());
        info!(logger, "date_fields: {}", self.date_fields());
        info!(logger, "header_rows: {}", self.header_rows());
        info!(logger, "next_rep_file: {}", self.next_rep_file());
        info!(
            logger,
            "next_rep_sheet_name: {}",
            self.next_rep_sheet_name()
        );
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
        let master_file_path = matches
            .value_of("master_file")
            .expect("Error getting `master_file_path`.")
            .to_string();
        let npa_master_file_path = matches
            .value_of("npa_master_file")
            .expect("Error getting `npa_master_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name`.")
            .to_string();
        let npa_sheet_name = matches
            .value_of("npa_sheet_name")
            .expect("Error getting `npa_sheet_name`.")
            .to_string();
        let date_fields = matches
            .value_of("date_fields")
            .expect("Error getting `date_fields`.")
            .to_string();
        let header_rows = matches
            .value_of("header_rows")
            .expect("Error getting `header_rows`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let next_rep_file = matches
            .value_of("next_rep_file")
            .expect("Error getting `next_rep_file`.")
            .to_string();
        let next_rep_sheet_name = matches
            .value_of("next_rep_sheet_name")
            .expect("Error getting `next_rep_sheet_name`.")
            .to_string();
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            master_file_path,
            npa_master_file_path,
            output_file_path,
            sheet_name,
            npa_sheet_name,
            date_fields,
            header_rows,
            log_level,
            is_perf_diagnostics_enabled,
            next_rep_file,
            next_rep_sheet_name,
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
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn npa_master_file_path(&self) -> &str {
        &self.npa_master_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn npa_sheet_name(&self) -> &str {
        &self.npa_sheet_name
    }
    pub fn date_fields(&self) -> &str {
        &self.date_fields
    }
    pub fn header_rows(&self) -> &str {
        &self.header_rows
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
    pub fn next_rep_file(&self) -> &str {
        &self.next_rep_file
    }
    pub fn next_rep_sheet_name(&self) -> &str {
        &self.next_rep_sheet_name
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("Pre Processor for Loans EMI")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .version("1.0.5223")
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
            Arg::new("master_file")
                .long("master-file")
                .value_name("master File Path")
                .help("Path to read Master.")
                .required(true)
        )
        .arg(
            Arg::new("npa_master_file")
                .long("npa-master-file")
                .value_name("npa master File Path")
                .help("Path to read Npa Master.")
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
            Arg::new("sheet_name")
                .long("sheet-name")
                .value_name("sheet-name")
                .help("sheet-name of master file")
                .default_value("sheet1")
                .required(true)
        )
        .arg(
            Arg::new("npa_sheet_name")
                .long("npa-sheet-name")
                .value_name("npa-sheet-name")
                .help("sheet-name of npa master file")
                .default_value("sheet1")
                .required(false)
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
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
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
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::new("next_rep_file")
                .long("next-rep-file")
                .value_name("Next Rep Date File Path")
                .help("Path to read Next Repricing Date Excel File.")
                .required(true)
        )
        .arg(
            Arg::new("next_rep_sheet_name")
                .long("next-rep-sheet-name")
                .value_name("next_rep_sheet_name")
                .help("Sheet Name of Next Rep Date Master file")
                .default_value("Sheet1")
                .required(false)
        )
        .get_matches()
}
