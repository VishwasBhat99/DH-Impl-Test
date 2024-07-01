use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub core_input_file: String,
    pub core_add_input_file: String,
    pub non_core_input_file: String,
    pub non_core_add_input_file: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "core_input_file: {}", self.core_input_file());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(
            logger,
            "non_core_input_file: {}",
            self.non_core_input_file()
        );
        info!(
            logger,
            "core_add_input_file: {}",
            self.core_add_input_file()
        );
        info!(
            logger,
            "non_core_add_input_file: {}",
            self.non_core_add_input_file()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let core_input_file = matches
            .value_of("core_input_file")
            .expect("Error getting `core_input_file` value.")
            .to_string();
        let core_add_input_file = matches
            .value_of("core_add_input_file")
            .expect("Error getting `core_add_input_file` value.")
            .to_string();
        let non_core_add_input_file = matches
            .value_of("non_core_add_input_file")
            .expect("Error getting `non_core_add_input_file` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let non_core_input_file = matches
            .value_of("non_core_input_file")
            .expect("Error getting `non_core_input_file` value.")
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

        ConfigurationParameters {
            core_input_file,
            non_core_input_file,
            core_add_input_file,
            non_core_add_input_file,
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
    pub fn core_input_file(&self) -> &str {
        &self.core_input_file
    }
    pub fn core_add_input_file(&self) -> &str {
        &self.core_add_input_file
    }
    pub fn non_core_add_input_file(&self) -> &str {
        &self.non_core_add_input_file
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn non_core_input_file(&self) -> &str {
        &self.non_core_input_file
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
        .version("1.3.4961")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app modifies data to conform with the input requirements of OD CFGen!")
        .arg(
            Arg::with_name("core_input_file")
                .long("core-file")
                .value_name("Core Input File")
                .help("Path to core input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("core_add_input_file")
                .long("core-add-file")
                .value_name("Core Additional Input File")
                .help("Path to core Additional input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("non_core_add_input_file")
                .long("non-core-add-file")
                .value_name("Non Core Additional Input File")
                .help("Path to non core additional input file that needs to be processed.")
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
            Arg::with_name("non_core_input_file")
                .long("non-core-file")
                .value_name("Non Core File Path")
                .help("Path to non-core input file that needs to be processed.")
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
        .get_matches()
}
