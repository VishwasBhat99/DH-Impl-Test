extern crate clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub output_file_path: String,
    pub is_perf_diagnostics_enabled: bool,
    pub diagnostics_file_path: String,
    pub log_file_path: String,
    pub log_level: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();

        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();

        let diagnostics_file_path = matches
            .value_of("diagnostics_file_path")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");
        let log_file_path = matches
            .value_of("log_file_path")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            output_file_path,
            is_perf_diagnostics_enabled,
            diagnostics_file_path,
            log_file_path,
            log_level,
        }
    }
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            " is_perf_diagnostics_enabled: {}",
            self.is_perf_diagnostics_enabled()
        );
        info!(
            logger,
            "diagnostics_file_path: {}",
            self.diagnostics_file_path()
        );
    }
}

impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Rate card extractor program for IRRBB!!!")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .short("i")
                .value_name("INPUT FILE")
                .help("Path to input file that needs to be processed.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .short("o")
                .value_name("OUTPUT FILE")
                .help("Path to output file that needs to be written.")
                .required(true),
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("LOG FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_file_path")
                .long("diagnostics-file-path")
                .value_name("DIAGNOSTIC FILE")
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
        .get_matches()
}
