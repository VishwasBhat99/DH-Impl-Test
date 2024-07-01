use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    acc_class_map_file_path: String,
    req_file_path: String,
    metadata_file_path: String,
    rules_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    rev_col_haircut_flag: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "acc_class_map_file_path: {}",
            self.acc_class_map_file_path()
        );
        info!(logger, "req_file_path: {}", self.req_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "rev_col_haircut_flag: {}",
            self.rev_col_haircut_flag()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let req_file_path = matches
            .value_of("req_file_path")
            .expect("Error getting `req_file_path`.")
            .to_string();
        let acc_class_map_file_path = matches
            .value_of("acc_class_map_file_path")
            .expect("Error getting `acc_class_map_file_path`.")
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
        let rev_col_haircut_flag = matches
            .value_of("rev_col_haircut_flag")
            .expect("Error getting `rev_col_haircut_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `rev_col_haircut_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            acc_class_map_file_path,
            req_file_path,
            metadata_file_path,
            rules_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            rev_col_haircut_flag,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn req_file_path(&self) -> &str {
        &self.req_file_path
    }
    pub fn acc_class_map_file_path(&self) -> &str {
        &self.acc_class_map_file_path
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
    pub fn rev_col_haircut_flag(&self) -> bool {
        self.rev_col_haircut_flag
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("CRM P1 Program!!")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .version("1.0.5164")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to Input CF File.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_file_path")
                .long("req-file-path")
                .value_name("Req Fiedls File Path")
                .help("Path to req fields file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("acc_class_map_file_path")
                .long("acc-class-map-file-path")
                .value_name("Account Classification Mapping")
                .help("Path to Account Classification Mapping file path.")
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
            Arg::with_name("rev_col_haircut_flag")
                .long("rev-col-hair-cut-flag")
                .value_name("Collateral Hair Cut Flag")
                .help("To calculate revised collateral after hair cut.")
                .default_value("false")
                .required(false)
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
