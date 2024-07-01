use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    metadata_file_path: String,
    req_field_file: String,
    rules_file_path: String,
    output_file_path: String,
    recon_file_path: String,
    master_file_path: String,
    write_master: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "recon_file_path: {}", self.recon_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "write_master: {}", self.write_master());
        info!(logger, "req_field_file: {}", self.req_field_file());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let rules_file_path = matches
            .value_of("rules_file_path")
            .expect("Error getting `rules_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let recon_file_path = matches
            .value_of("recon_file_path")
            .expect("Error getting `recon_file_path`.")
            .to_string();
        let master_file_path = matches
            .value_of("master_file_path")
            .expect("Error getting `master_file_path`.")
            .to_string();
        let write_master = matches
            .value_of("write_master")
            .expect("Error getting `write_master`.")
            .parse::<bool>()
            .expect("Cannot parse `write_master` as bool.");
        let req_field_file = matches
            .value_of("req_field_file")
            .expect("Error getting `req_field_file`.")
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

        ConfigurationParameters {
            input_file_path,
            metadata_file_path,
            req_field_file,
            rules_file_path,
            output_file_path,
            recon_file_path,
            master_file_path,
            write_master,
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
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn recon_file_path(&self) -> &str {
        &self.recon_file_path
    }
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn write_master(&self) -> bool {
        self.write_master
    }
    pub fn req_field_file(&self) -> &String {
        &self.req_field_file
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
        .about("This program will Split .cf file based on rules!!")
        .version("1.0.4369")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to Input CF File.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file_path")
                .long("metadata-file-path")
                .value_name("Metadata File Path")
                .help("Path to metadata file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .long("rules-file-path")
                .value_name("Rules File Path")
                .help("Path to rules file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to Output CF File.")
                .required(true)
        )
        .arg(
            Arg::with_name("recon_file_path")
                .long("recon-file-path")
                .value_name("Recon File Path")
                .help("Path to Recon CF File.")
                .required(true)
        )
        .arg(
            Arg::with_name("master_file_path")
                .long("master-file-path")
                .value_name("master File Path")
                .help("Path to master File.")
                .required(false)
        )
        .arg(
            Arg::with_name("write_master")
                .long("write-master")
                .value_name("Master writer flag")
                .help("This flag will decide to master file has to be written or not.")
                .possible_values(&["true","false"])
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("req_field_file")
                .long("req_field_file")
                .value_name("Required field file")
                .help("Required Field File")
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
