use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);

    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    skip_header: bool,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    source_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "skip_header: {}", self.skip_header());
        info!(logger, "source_name: {}", self.source_name());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input-file-path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser
            .parse_opt(
                matches
                    .value_of("as_on_date")
                    .expect("Error getting `as_on_date` value."),
            )
            .expect("Cannot parse `as_on_date` value as `DD-MM-YYYY` format");

        let output_file_path = matches
            .value_of("output_file_path")
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
            .expect("Cannot parse `perf_diag_flag` value as bool type");
        let skip_header = matches
            .value_of("skip_header")
            .expect("Error getting `skip_header` value.")
            .parse::<bool>()
            .expect("Cannot parse `skip_header` value as bool type");
        let source_name = matches
            .value_of("source_name")
            .expect("Error getting `source_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            skip_header,
            log_level,
            is_perf_diagnostics_enabled,
            source_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
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
    pub fn skip_header(&self) -> bool {
        self.skip_header
    }
    pub fn source_name(&self) -> &str {
        &self.source_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates Cashflows with TD Penalty Waiver Feature!")
        .author("Tanuj Singh Rathore <tanuuj.s@surya-soft.com>")
        .version("1.0.4995")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("OUTPUT FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("source_name")
                .long("source-name")
                .value_name("Source Name")
                .help("Source Name.")
                .required(true)
        )
         .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general logs.")
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
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("skip_header")
                .long("skip-header")
                .value_name("SKIP HEADER FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to skip the header from input file.")
                .default_value("true")
                .required(false)
        )
        .get_matches()
}
