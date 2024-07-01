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
    pub holiday_file_path: String,
    pub input_file_delimiter: String,
    pub holiday_file_delimiter: String,
    pub input_date_format: String,
    pub as_on_date: NaiveDate,
    pub currency: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "holiday_file: {}", self.holiday_file_path());
        info!(logger, "input_field_separator: {}", self.input_delimiter());
        info!(
            logger,
            "Holiday_field_separator: {}",
            self.holiday_delimiter()
        );
        info!(logger, "input_date_format: {}", self.date_format());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "currency: {:?}", self.currency());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();
        let input_file_delimiter = matches
            .value_of("input_file_delimiter")
            .expect("Error getting `input_file_delimiter` value.")
            .to_string();
        let holiday_file_delimiter = matches
            .value_of("holiday_file_delimiter")
            .expect("Error getting `holiday_file_delimiter` value.")
            .to_string();
        let input_date_format = matches
            .value_of("input_date_format")
            .expect("Error getting `input_date_format` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );

        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
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
        let holiday_file_path = matches
            .value_of("holiday_file_path")
            .expect("Error getting `holiday_file_path` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            holiday_file_path,
            input_file_delimiter,
            holiday_file_delimiter,
            input_date_format,
            as_on_date,
            currency,
            output_file_path,
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
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn holiday_file_path(&self) -> &str {
        &self.holiday_file_path
    }
    pub fn holiday_delimiter(&self) -> &str {
        &self.holiday_file_delimiter
    }
    pub fn input_delimiter(&self) -> &str {
        &self.input_file_delimiter
    }
    pub fn date_format(&self) -> &str {
        &self.input_date_format
    }
    pub fn as_on_date(&self) -> &NaiveDate {
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
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Pre-processor for Accrued Reservables.")
        .version("1.1.4114")
        .author("Bhargavi052 <bhargavi.n@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("holiday_file_path")
                .long("holiday-file")
                .value_name("holiday_file_path")
                .help("Path to the holiday file.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_delimiter")
                .long("input-file-delimiter")
                .value_name("Input File Delimiter")
                .help("Input file field seperator.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::with_name("currency")
                .long("ccy")
                .value_name("Currency")
                .help("Currency to lookup in Holiday Input file.")
                .default_value("KWD")
                .required(false)
        )
        .arg(
            Arg::with_name("holiday_file_delimiter")
                .long("holiday-file-delimiter")
                .value_name("holiday_file_delimiter")
                .help("Holiday file field seperator.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::with_name("input_date_format")
                .long("input-date-format")
                .value_name("Input Date Format")
                .help("Input Date Format.")
                .possible_values(&["dd-mm-yyyy","dd.mm.yyyy","dd/mm/yyyy","yyyy-mm-dd","yyyy.mm.dd","yyyy/mm/dd","dd-mmm-yyyy","dd/mmm/yyyy","dd.mmm.yyyy","yyyy-mmm-dd","yyyy/mmm/dd","yyyy.mmm.dd","dd.mm.yy","dd/mm/yy","dd-mm-yy","yy-mm-dd","yy/mm/dd","yy.mm.dd"])
                .default_value("dd-mm-yyyy")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
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
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
