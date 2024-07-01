use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_config: String,
    output_path: String,
    exp_base_file: String,
    exp_base_file_sheet_name: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_limit_required: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_config: {}", self.input_config());
        info!(logger, "output_path: {}", self.output_path());
        info!(logger, "exp_base_file: {}", self.exp_base_file());
        info!(
            logger,
            "exp_base_file_sheet_name: {}",
            self.exp_base_file_sheet_name()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "is_limit_required: {}", self.is_limit_required());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_config = matches
            .value_of("input_config")
            .expect("Error getting `input_config`.")
            .to_string();
        let output_path = matches
            .value_of("output_path")
            .expect("Error getting `output_path`.")
            .to_string();
        let exp_base_file = matches
            .value_of("exp_base_file")
            .expect("Error getting `exp_base_file`.")
            .to_string();
        let exp_base_file_sheet_name = matches
            .value_of("exp_base_file_sheet_name")
            .expect("Error getting `exp_base_file_sheet_name`.")
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
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let is_limit_required = matches
            .value_of("is_limit_required")
            .expect("Error getting `is_limit_required`.")
            .parse::<bool>()
            .expect("Cannot parse `is_limit_required` as bool.");

        ConfigurationParameters {
            input_config,
            output_path,
            exp_base_file,
            exp_base_file_sheet_name,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_limit_required,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_config(&self) -> &str {
        &self.input_config
    }
    pub fn output_path(&self) -> &str {
        &self.output_path
    }
    pub fn exp_base_file(&self) -> &str {
        &self.exp_base_file
    }
    pub fn exp_base_file_sheet_name(&self) -> &str {
        &self.exp_base_file_sheet_name
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
    pub fn is_limit_required(&self) -> bool {
        self.is_limit_required
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program to calculate the total balance at customer level!!")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .version("1.1.4849")
        .arg(
            Arg::with_name("input_config")
                .long("input-config")
                .value_name("Input Config File Path")
                .help("Path to input config file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_path")
                .long("output-path")
                .value_name("Output File Path")
                .help("Path to write output.")
                .required(true)
        )
        .arg(
            Arg::with_name("exp_base_file")
                .long("exp-base-file")
                .value_name("Exposure Base File Path")
                .help("Path to Exposure file.")
                .required(true)
        )
        .arg(
            Arg::with_name("exp_base_file_sheet_name")
                .long("exp-base-file-sheet-name")
                .value_name("Exposure Base File Sheet Name")
                .help("Exposure Base File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
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
            Arg::with_name("is_limit_required")
                .long("is-limit-required")
                .value_name("LIMIT CALC FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether limit_amt and limit_status will be written to the output files.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
