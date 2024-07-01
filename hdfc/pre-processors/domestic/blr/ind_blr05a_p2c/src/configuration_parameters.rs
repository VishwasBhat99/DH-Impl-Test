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
    pub as_on_date: NaiveDate,
    pub input_file: String,
    pub output_file: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "diagnostics_file_path: {}",
            self.diagnostics_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file_path")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_file_path")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        ConfigurationParameters {
            as_on_date,
            input_file,
            output_file,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
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
        .about("Breach/Penalty in respect of Domestic Regulatory Liquidity Requirements (CRR and SLR)!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("input_file")
                .help("Input File")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("Log File Path")
                .help("Path to the Log file.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_file_path")
                .long("diagnostics-file-path")
                .value_name("DiagLog File")
                .help("Path to the Diagnostic File.")
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
        .get_matches()
}
