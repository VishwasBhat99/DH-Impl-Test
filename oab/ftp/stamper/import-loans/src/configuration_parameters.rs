use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_args_for_app(app_name);

    let parameters = ConfigurationParameters::new_from_matches(matches);
    return parameters;
}

pub struct ConfigurationParameters {
    input_file_path: String,
    rule_file_path: String,
    stamp_field: String,
    default_stamp_code: i32,
    metadata_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "rule_file_path: {}", self.rule_file_path());
        info!(logger, "stamp_field: {}", self.stamp_field());
        info!(logger, "default_stamp_code: {}", self.default_stamp_code());
        info!(logger, "meta_data_file: {}", self.metadata_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches.value_of("input_file").unwrap().to_string();
        let rule_file_path = matches.value_of("rule_file_path").unwrap().to_string();
        let stamp_field = matches.value_of("stamp_field").unwrap().to_string();
        let default_stamp_code = matches
            .value_of("default_stamp_code")
            .expect("Error while getting `default llg code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default stamp_code code` as i32.");
        let metadata_file_path = matches.value_of("metadata_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .unwrap()
            .to_string();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        ConfigurationParameters {
            input_file_path,
            rule_file_path,
            stamp_field,
            default_stamp_code,
            metadata_file_path,
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
    pub fn input_file_path(&self) -> &str {
        return &self.input_file_path;
    }
    pub fn rule_file_path(&self) -> &str {
        return &self.rule_file_path;
    }
    pub fn stamp_field(&self) -> &str {
        return &self.stamp_field;
    }
    pub fn default_stamp_code(&self) -> i32 {
        return self.default_stamp_code;
    }
    pub fn metadata_file_path(&self) -> &str {
        return &self.metadata_file_path;
    }
    pub fn output_file_path(&self) -> &str {
        return &self.output_file_path;
    }
    pub fn log_file_path(&self) -> &str {
        return &self.log_file_path;
    }
    pub fn diagnostics_file_path(&self) -> &str {
        return &self.diagnostics_file_path;
    }
    pub fn log_level(&self) -> &str {
        return &self.log_level;
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        return self.is_perf_diagnostics_enabled;
    }
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.1.1")
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("rule_file_path")
                .short("r")
                .long("rule-file")
                .value_name("FILE")
                .help("Path to the rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("stamp_field")
                .short("f")
                .long("stamp-field")
                .value_name("FIELD")
                .help("Provides the field name to which rule is stampped")
                .required(true)
        )
        .arg(
            Arg::with_name("default_stamp_code")
                .short("d")
                .long("default-stamp-code")
                .value_name("STAMP CODE")
                .help("Provides the default stamp code")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file")
                .short("m")
                .long("metadata-file")
                .value_name("FILE")
                .help("Path to metadata file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .short("e")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
