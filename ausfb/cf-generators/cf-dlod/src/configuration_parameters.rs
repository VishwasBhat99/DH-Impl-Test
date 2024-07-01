use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    dlod_file_1_path: String,
    dlod_file_2_path: String,
    as_on_date: NaiveDate,
    dlod_separator_1: String,
    dlod_date_format_1: String,
    dlod_separator_2: String,
    dlod_date_format_2: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "dlod_cf_file_1: {}", self.dlod_file_1_path());
        info!(logger, "dlod_cf_file_2: {}", self.dlod_file_2_path());
        info!(logger, "dlod_separator_1: {}", self.dlod_separator_1());
        info!(logger, "dlod_date_format_1: {}", self.dlod_date_format_1());
        info!(logger, "dlod_separator_2: {}", self.dlod_separator_2());
        info!(logger, "dlod_date_format_2: {}", self.dlod_date_format_2());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let dlod_file_1_path = matches
            .value_of("dlod_file_1")
            .expect("Error getting `dlod_file_1_path`.")
            .to_string();
        let dlod_file_2_path = matches
            .value_of("dlod_file_2")
            .expect("Error getting `dlod_file_2_path`.")
            .to_string();
        let dlod_separator_1 = matches
            .value_of("dlod_separator_1")
            .expect("Error getting `dlod_separator_1`.")
            .to_string();
        let dlod_date_format_1 = matches
            .value_of("dlod_date_format_1")
            .expect("Error getting `dlod_date_format_1`.")
            .to_string();
        let dlod_separator_2 = matches
            .value_of("dlod_separator_2")
            .expect("Error getting `dlod_separator_2`.")
            .to_string();
        let dlod_date_format_2 = matches
            .value_of("dlod_date_format_2")
            .expect("Error getting `dlod_date_format_2`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );

        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            dlod_file_1_path,
            dlod_file_2_path,
            dlod_separator_1,
            dlod_separator_2,
            dlod_date_format_1,
            dlod_date_format_2,
            as_on_date,
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
    pub fn dlod_file_1_path(&self) -> &str {
        &self.dlod_file_1_path
    }
    pub fn dlod_file_2_path(&self) -> &str {
        &self.dlod_file_2_path
    }
    pub fn dlod_separator_1(&self) -> &str {
        &self.dlod_separator_1
    }
    pub fn dlod_separator_2(&self) -> &str {
        &self.dlod_separator_2
    }
    pub fn dlod_date_format_1(&self) -> &str {
        &self.dlod_date_format_1
    }
    pub fn dlod_date_format_2(&self) -> &str {
        &self.dlod_date_format_2
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
        .about("Cashflow derivation for DropLine OverDraft.")
        .author("Bhargavi052 <bhargavi.n@surya-soft.com>")
        .version("1.0.4577")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_file_1")
                .long("dlod-file-1")
                .value_name("DLOD CF File 1")
                .help("Path to the input DLOD CF file 1.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_file_2")
                .long("dlod-file-2")
                .value_name("DLOD CF File 2")
                .help("Path to the input DLOD CF file 2.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_separator_1")
                .long("dlod-1-separator")
                .value_name("DLOD File 1 Separator")
                .help("Path to the input DLOD file 1 separator.")
                .default_value(",")
                .required(false)
        )
        .arg(
            Arg::with_name("dlod_separator_2")
                .long("dlod-2-separator")
                .value_name("DLOD File 2 Separator")
                .help("Path to the input DLOD file 2 separator.")
                .default_value(",")
                .required(false)
        )
        .arg(
            Arg::with_name("dlod_date_format_1")
                .long("dlod-date-format-1")
                .value_name("DLOD File 1 Date format")
                .help("Path to the input DLOD file 1 Date Format.")
                .default_value("%d-%m-%y")
                .required(false)
        )
        .arg(
            Arg::with_name("dlod_date_format_2")
                .long("dlod-date-format-2")
                .value_name("DLOD File 2 Date format")
                .help("Path to the input DLOD file 2 Date Format.")
                .default_value("%d-%m-%y")
                .required(false)
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
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
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
