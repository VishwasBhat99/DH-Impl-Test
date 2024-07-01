use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub input_date_format: String,
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub input_file_path: String,
    pub sc_dump_file_path: String,
    pub tcfsl_file_path: String,
    pub brnet_file_path: String,
    pub writeoff_merged_file_path: String,
    pub output_file_path: String,
    pub tcfsl_sheet_name: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "input_date_format: {:?}", self.input_date_format());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "sc_dump_file_path: {}", self.sc_dump_file_path());
        info!(logger, "tcfsl_file_path: {}", self.tcfsl_file_path());
        info!(logger, "brnet_file_path: {}", self.brnet_file_path());
        info!(
            logger,
            "writeoff_merged_file_path: {}",
            self.writeoff_merged_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "tcfsl_sheet_name: {}", self.tcfsl_sheet_name());
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
        let input_date_format = match matches
            .value_of("input_date_format")
            .expect("Error getting `input_date_format` value.")
        {
            "ddmmyyyy" => "%d%m%Y",
            "dd-mm-yyyy" => "%d-%m-%Y",
            "dd-mmm-yyyy" => "%d-%b-%Y",
            "yyyymmdd" => "%Y%m%d",
            "yyyy-mm-dd" => "%Y-%m-%d",
            "yyyy-mmm-dd" => "%Y-%b-%d",
            "dd-mmm-yy" => "%d-%b-%y",
            "dd-mm-yy" => "%d-%m-%y",
            _ => panic!("Invalid Date Format!"),
        }
        .to_string();
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
        let sc_dump_file_path = matches
            .value_of("sc_dump_file")
            .expect("Error getting `sc_dump_file_path`.")
            .to_string();
        let tcfsl_file_path = matches
            .value_of("tcfsl_file")
            .expect("Error getting `tcfsl_file_path`.")
            .to_string();
        let brnet_file_path: String = matches
            .value_of("brnet_file")
            .expect("Error getting `brnet_file_Path`.")
            .to_string();
        let writeoff_merged_file_path: String = matches
            .value_of("writeoff_merged_file")
            .expect("Error getting `writeoff_merged_Path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let tcfsl_sheet_name = matches
            .value_of("tcfsl_sheet_name")
            .expect("Error getting `tcfsl_sheet_name`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        ConfigurationParameters {
            input_date_format,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            sc_dump_file_path,
            tcfsl_file_path,
            brnet_file_path,
            writeoff_merged_file_path,
            output_file_path,
            tcfsl_sheet_name,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_date_format(&self) -> &str {
        &self.input_date_format
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn sc_dump_file_path(&self) -> &str {
        &self.sc_dump_file_path
    }
    pub fn tcfsl_file_path(&self) -> &str {
        &self.tcfsl_file_path
    }
    pub fn brnet_file_path(&self) -> &str {
        &self.brnet_file_path
    }
    pub fn writeoff_merged_file_path(&self) -> &str {
        &self.writeoff_merged_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn tcfsl_sheet_name(&self) -> &str {
        &self.tcfsl_sheet_name
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
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("pp_brnet")
        .author("Ravindar-01 <ravinar.sr@surya-soft.com>")
        .version("1.0.5230")
        .arg(
            Arg::with_name("input_date_format")
                .long("input-date-format")
                .value_name("Input Date Format")
                .help("Path to write logs.")
                .possible_values(&["ddmmyyyy","dd-mm-yyyy","dd-mmm-yyyy","yyyymmdd","yyyy-mm-dd","yyyy-mmm-dd","dd-mmm-yy","dd-mm-yy"])
                .default_value("dd-mm-yyyy")
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
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("Base Input File Path")
                .help("Path to read Base Input.")
                .required(true)
        )
        .arg(
            Arg::new("sc_dump_file")
                .long("sc-dump-file")
                .value_name("Schedule Dump File Path")
                .help("Path to read Schedule Dump.")
                .required(true)
        )
        .arg(
            Arg::new("tcfsl_file")
                .long("tcfsl-file")
                .value_name("TCFSL File Path")
                .help("Path to read TCFSL.")
                .required(true)
        )
        .arg(
            Arg::new("brnet_file")
                .long("brnet-file")
                .value_name("Brnet File Path")
                .help("Path to read Brnet.")
                .required(true)
        )
        .arg(
            Arg::new("writeoff_merged_file")
                .long("writeoff-merged-file")
                .value_name("writeoff merged File Path")
                .help("Path to read writeoff merged.")
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
            Arg::new("tcfsl_sheet_name")
                .long("tcfsl-sheet-name")
                .value_name("tcfsl Sheet Name")
                .help("Path to write tcfsl Sheet Name.")
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
        .get_matches()
}
