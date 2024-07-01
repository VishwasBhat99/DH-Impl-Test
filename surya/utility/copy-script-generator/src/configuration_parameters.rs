use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub scenario_file_path: String,
    pub executor_id: String,
    pub command: String,
    pub target_server: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub diagnostics_flag: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "scenario_file_path: {}", self.scenario_file_path());
        info!(logger, "executor_id: {}", self.executor_id());
        info!(logger, "command: {}", self.command());
        info!(logger, "target_server: {}", self.target_server());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "diagnostics_flag: {}", self.diagnostics_flag());
        info!(
            logger,
            "diagnostics_file_path: {}",
            self.diagnostics_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let scenario_file_path = matches
            .value_of("scenario_file_path")
            .expect("Error getting `scenario_file_path` value.")
            .to_string();
        let executor_id = matches
            .value_of("executor_id")
            .expect("Error getting `executor_id` value.")
            .to_string();
        let command = matches
            .value_of("command")
            .expect("Error getting `command` value.")
            .to_string();
        let target_server = matches
            .value_of("target_server")
            .expect("Error getting `target_server` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file_path")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_file_path")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let diagnostics_flag = matches
            .value_of("diagnostics_flag")
            .expect("Error getting `diagnostics_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `diagnostics_flag` value as bool.");

        ConfigurationParameters {
            scenario_file_path,
            executor_id,
            command,
            target_server,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            diagnostics_flag,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn scenario_file_path(&self) -> &str {
        &self.scenario_file_path
    }
    pub fn executor_id(&self) -> &str {
        &self.executor_id
    }
    pub fn command(&self) -> &str {
        &self.command
    }
    pub fn target_server(&self) -> &str {
        &self.target_server
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
    pub fn diagnostics_flag(&self) -> bool {
        self.diagnostics_flag
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Copy Script Generator!")
        .arg(
            Arg::with_name("scenario_file_path")
                .long("scenario-file-path")
                .short("s")
                .value_name("Scenario File Path")
                .help("Path to the scenario files.")
                .required(true),
        )
        .arg(
            Arg::with_name("executor_id")
                .long("executor-id")
                .short("e")
                .value_name("Executor Id")
                .help("Executor Id.")
                .required(true),
        )
        .arg(
            Arg::with_name("command")
                .long("command")
                .short("c")
                .value_name("Command")
                .help("Command.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("target_server")
                .long("target-server")
                .value_name("Target Server")
                .help("Target Server.")
                .required(true),
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_file_path")
                .long("diagnostics-file-path")
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
            Arg::with_name("diagnostics_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
