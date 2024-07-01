use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    vayana_loans_file_path: String,
    gam_file_path: String,
    npa_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "vayana_loans_file_path: {}",
            self.vayana_loans_file_path()
        );
        info!(logger, "gam_file_path: {}", self.gam_file_path());
        info!(logger, "npa_file_path: {}", self.npa_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let vayana_loans_file_path = matches
            .value_of("vayana_loans_file_path")
            .expect("Error getting `vayana_loans_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let gam_file_path = matches
            .value_of("gam_file_path")
            .expect("Error getting `gam_file_path`.")
            .to_string();
        let npa_file_path = matches
            .value_of("npa_file_path")
            .expect("Error getting `npa_file_path`.")
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
            vayana_loans_file_path,
            gam_file_path,
            npa_file_path,
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
    pub fn vayana_loans_file_path(&self) -> &str {
        &self.vayana_loans_file_path
    }
    pub fn gam_file_path(&self) -> &str {
        &self.gam_file_path
    }
    pub fn npa_file_path(&self) -> &str {
        &self.npa_file_path
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program to preprocess Vayana loans.")
        .version("1.0.4548")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("vayana_loans_file_path")
                .long("vayana-loans-file-path")
                .value_name("Input file for Vayana Loans")
                .help("Path to Vayana Loans File.")
                .required(true)
        )
        .arg(
            Arg::with_name("gam_file_path")
                .long("gam-file-path")
                .value_name("GAM File")
                .help("Path to GAM data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_file_path")
                .long("npa-file-path")
                .value_name("NPA File")
                .help("Path to NPA data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("output file path")
                .help("Path to output file.")
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
