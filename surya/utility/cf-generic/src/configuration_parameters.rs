use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub input_metadata_file: String,
    pub as_on_date: rbdate::NaiveDate,
    pub output_file_path: String,
    pub output_metadata_file: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub cf_fields_col: Vec<String>,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub skip_header_count: usize,
    pub skip_footer_count: usize,
    pub input_date_format: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "input_metadata_file: {}",
            self.input_metadata_file()
        );
        info!(
            logger,
            "output_metadata_file: {}",
            self.output_metadata_file()
        );
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "cf_fields_col: {:?}", self.cf_fields_col());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "skip_header_count: {}", self.skip_header_count());
        info!(logger, "skip_footer_count: {}", self.skip_footer_count());
        info!(logger, "input_date_format: {}", self.input_date_format());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
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
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();
        let input_metadata_file = matches
            .value_of("input_metadata_file")
            .expect("Error getting `input_metadata_file` value.")
            .to_string();
        let output_metadata_file = matches
            .value_of("output_metadata_file")
            .expect("Error getting `output_metadata_file` value.")
            .to_string();
        let skip_header_count = matches
            .value_of("skip_header_count")
            .expect("Error getting `skip_header_count` value.")
            .to_string()
            .parse::<usize>()
            .expect("Error getting `skip_header_count` value.");
        let skip_footer_count = matches
            .value_of("skip_footer_count")
            .expect("Error getting `skip_footer_count` value.")
            .to_string()
            .parse::<usize>()
            .expect("Error getting `skip_footer_count` value.");
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
        let cf_fields_col: Vec<String> = matches
            .value_of("cf_fields_col")
            .expect("Error getting `cf_fields_col`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        ConfigurationParameters {
            input_file_path,
            input_metadata_file,
            output_metadata_file,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            cf_fields_col,
            is_perf_diagnostics_enabled,
            skip_header_count,
            skip_footer_count,
            input_date_format,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn input_metadata_file(&self) -> &str {
        &self.input_metadata_file
    }
    pub fn output_metadata_file(&self) -> &str {
        &self.output_metadata_file
    }
    pub fn as_on_date(&self) -> &rbdate::NaiveDate {
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
    pub fn cf_fields_col(&self) -> &Vec<String> {
        &self.cf_fields_col
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn skip_header_count(&self) -> &usize {
        &self.skip_header_count
    }
    pub fn skip_footer_count(&self) -> &usize {
        &self.skip_footer_count
    }
    pub fn input_date_format(&self) -> &str {
        &self.input_date_format
    }
}

fn get_eligible_arguments_for_command(command_name: &str) -> clap::ArgMatches {
    Command::new(command_name)
        .about("Maturity Generic CF Program!")
        .version("1.0.1")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("input_file_path")
                .long("input-file")
                .value_name("input_file")
                .help("Path to Master Input File.")
                .required(true)
        )
        .arg(
            Arg::new("input_metadata_file")
                .long("input-metadata-file")
                .value_name("input_metadata_file")
                .help("Path to Input Metadata File.")
                .required(true)
        )
        .arg(
            Arg::new("output_metadata_file")
                .long("output-metadata-file")
                .value_name("output_metadata_file")
                .help("Path to Output Metadata File.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to the Output File.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
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
            Arg::new("skip_header_count")
                .long("skip-header-count")
                .value_name("skip_header_count")
                .help("Number of Headers in Input File.")
                .default_value("0")
                .required(false)
        )
        .arg(
            Arg::new("skip_footer_count")
                .long("skip-footer-count")
                .value_name("skip_footer_count")
                .help("Number of Footers in Input File.")
                .default_value("0")
                .required(false)
        )
        .arg(
            Arg::new("input_date_format")
                .long("input-date-format")
                .value_name("Date Format")
                .help("Expected Date Format from Input File for Date Fields.")
                .possible_values(["ddmmyyyy","dd-mm-yyyy","dd-mmm-yyyy","yyyymmdd","yyyy-mm-dd","yyyy-mmm-dd","dd-mmm-yy","dd-mm-yy"])
                .default_value("dd-mm-yyyy")
                .required(false)
        )
        .arg(
            Arg::new("cf_fields_col")
                .long("cf-fields-col")
                .value_name("cashflow fields column")
                .help("Column to pick from input for cashflow fields.")
                .default_value("0,0,0")
                .required(false)
        )
        .get_matches()
}
