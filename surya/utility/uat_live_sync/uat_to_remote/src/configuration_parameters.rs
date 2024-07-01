use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    remote_folder_path: String,
    remote_account_config_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    all_streams_path: String,
    batch_info_file: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "remote_folder_path: {}", self.remote_folder_path());
        info!(logger, "all_streams_path: {}", self.all_streams_path());
        info!(logger, "batch_info_file: {}", self.batch_info_file());
        info!(
            logger,
            "remote_account_config_file: {}",
            self.remote_account_config_file()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let remote_folder_path = matches
            .value_of("remote_folder_path")
            .expect("Error getting `remote_folder_path` value.")
            .to_string();
        let remote_account_config_file = matches
            .value_of("account_config_file")
            .expect("Error getting `remote_account_config_file` value.")
            .to_string();
        let all_streams_path = matches
            .value_of("all_streams")
            .expect("Error getting `all_streams` value.")
            .to_string();
        let batch_info_file = matches
            .value_of("batch_info_file")
            .expect("Error getting `batch_info_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path` value.")
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

        ConfigurationParameters {
            remote_folder_path,
            remote_account_config_file,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            all_streams_path,
            batch_info_file,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn remote_folder_path(&self) -> &str {
        &self.remote_folder_path
    }
    pub fn all_streams_path(&self) -> &str {
        &self.all_streams_path
    }
    pub fn batch_info_file(&self) -> &str {
        &self.batch_info_file
    }
    pub fn remote_account_config_file(&self) -> &str {
        &self.remote_account_config_file
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
        .about("Program to Sync UAT to Remote!!")
        .arg(
            Arg::with_name("remote_folder_path")
                .long("remote-folder-path")
                .value_name("Remote Folder Path")
                .help("Path to Remote Folder")
                .required(true)
        )
        .arg(
            Arg::with_name("all_streams")
                .long("all-streams")
                .value_name("All Streams Folder")
                .help("Path to All Streams Folder")
                .required(true)
        )
        .arg(
            Arg::with_name("batch_info_file")
                .long("batch-info-file")
                .value_name("Batch Info File")
                .help("Path to Batch Info File")
                .required(true)
        )
        .arg(
            Arg::with_name("account_config_file")
                .long("account-config-file")
                .value_name("Account Config File")
                .help("Json file having login details")
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
        .get_matches()
}
