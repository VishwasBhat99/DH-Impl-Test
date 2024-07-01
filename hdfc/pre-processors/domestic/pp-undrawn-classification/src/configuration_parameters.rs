use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub lcr_master_file_path: String,
    pub line_template_undrawn_path: String,
    pub lcr_master_basel_path: String,
    pub odfd_path: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub as_on_date: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub delimiter: String,
    pub config_file_path: String,
    pub derived_flag: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(
            logger,
            "lcr_master_file_path: {}",
            self.lcr_master_file_path()
        );
        info!(
            logger,
            "line_template_undrawn_path: {}",
            self.line_template_undrawn_path()
        );
        info!(
            logger,
            "lcr_master_basel_path: {}",
            self.lcr_master_basel_path()
        );
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "odfd: {}", self.odfd_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "delimiter: {}", self.delimiter());
        info!(logger, "derieved_flag: {}", self.derived_flag());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let lcr_master_file_path = matches
            .value_of("lcr_master_file_path")
            .expect("Error getting `lcr_master_file_path` value.")
            .to_string();
        let line_template_undrawn_path = matches
            .value_of("line_template_undrawn_path")
            .expect("Error getting `line_template_undrawn_path` value.")
            .to_string();
        let odfd_path = matches
            .value_of("odfd_path")
            .expect("Error getting `odfd_path` value.")
            .to_string();
        let lcr_master_basel_path = matches
            .value_of("lcr_master_basel_path")
            .expect("Error getting `lcr_master_basel_path` value.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path` value.")
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
        let as_on_date = matches
            .value_of("as_on_date")
            .expect("Error getting `as_on_date` value.")
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
        let delimiter = matches
            .value_of("delimiter")
            .expect("Error getting `delimiter` value.")
            .to_string();
        let derived_flag = matches
            .value_of("derived_flag")
            .expect("Error getting `derived_flag` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            lcr_master_file_path,
            line_template_undrawn_path,
            odfd_path,
            lcr_master_basel_path,
            config_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            as_on_date,
            log_level,
            is_perf_diagnostics_enabled,
            delimiter,
            derived_flag,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn lcr_master_file_path(&self) -> &str {
        &self.lcr_master_file_path
    }
    pub fn line_template_undrawn_path(&self) -> &str {
        &self.line_template_undrawn_path
    }
    pub fn odfd_path(&self) -> &str {
        &self.odfd_path
    }
    pub fn lcr_master_basel_path(&self) -> &str {
        &self.lcr_master_basel_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
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
    pub fn as_on_date(&self) -> &str {
        &self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }
    pub fn derived_flag(&self) -> &str {
        &self.derived_flag
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.0.4460")
        .about("This app modifies data to conform with the input requirements of Undrawn Classification CFGen!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcr_master_file_path")
                .long("lcr-master-file")
                .value_name("lcr_master_file_path")
                .help("Path to the reference files: LCR Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("line_template_undrawn_path")
                .long("line-temp-undrawn")
                .value_name("line_template_undrawn_path")
                .help("Path to the reference files: Line template undrawn path.")
                .required(true)
        )
        .arg(
            Arg::with_name("odfd_path")
                .long("odfd-path")
                .value_name("ODFD_PATH")
                .help("Path to the reference files: ODFD Master.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcr_master_basel_path")
                .long("lcr-master-basel-path")
                .value_name("lcr_master_basel_path")
                .help("Path to the reference files: LCR Category Master.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("delimiter")
                .long("delimiter")
                .value_name("DELIMITER")
                .help("Specifies the input file field separator.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::with_name("derived_flag")
                .long("derived-flag")
                .value_name("DERIVED FLAG")
                .help("Specifies the derivation flag either UBS or LNM")
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
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file")
                .value_name("CONFIG FILE")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .get_matches()
}
