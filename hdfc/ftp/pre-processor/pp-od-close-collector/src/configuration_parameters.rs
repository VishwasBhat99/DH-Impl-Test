use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub stamper_file_path: String,
    pub amb_file_path: String,
    pub alm_file_delimeter: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "stamper_file_path: {}", self.stamper_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "amb_file_path: {}", self.amb_file_path());
        info!(logger, "amb_file_delimeter: {}", self.amb_file_delimeter());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
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
        let amb_file_path = matches
            .value_of("amb_file_path")
            .expect("Error getting `amb_file_path` value.")
            .to_string();
        let alm_file_delimeter: String = matches
            .value_of("amb_file_delimeter")
            .expect("Error getting `amb_file_delimeter` value.")
            .to_string();
        let stamper_file_path = matches
            .value_of("stamper_file_path")
            .expect("Error getting `stamper_file_path` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            stamper_file_path,
            amb_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            alm_file_delimeter,
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
    pub fn amb_file_path(&self) -> &str {
        &self.amb_file_path
    }
    pub fn amb_file_delimeter(&self) -> &str {
        &self.alm_file_delimeter
    }
    pub fn stamper_file_path(&self) -> &str {
        &self.stamper_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of OD Stamper!")
        .version("1.0.4961")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
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
            Arg::with_name("amb_file_path")
                .long("amb-file")
                .value_name("AMB File Path")
                .help("AMB File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("amb_file_delimeter")
                .long("amb-file-delimeter")
                .value_name("AMB File Delimeter")
                .help("AMB File Path.")
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
            Arg::with_name("stamper_file_path")
                .long("stamper-file")
                .value_name("stamper_file_path")
                .help("Stamper File Path.")
                .required(true)
        )
        .get_matches()
}
