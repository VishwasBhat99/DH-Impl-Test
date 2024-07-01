use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use serde_json::StreamDeserializer;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file: String,
    master1_file: String,
    master2_file: String,
    output_file: String,
    country: String,
    as_on_date: NaiveDate,
    delimeter: String,
    master1_sheet: String,
    master2_sheet: String,
    input_sheet: String,
    default_gl_code: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "master1_file: {}", self.master1_file());
        info!(logger, "master2_file: {}", self.master2_file());
        info!(
            logger,
            "output_file: {}",
            self.output_file()
        );
        info!(logger, "country: {}", self.country());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "delimeter: {}", self.delimeter());
        info!(logger, "master1_sheet: {}", self.master1_sheet());
        info!(logger, "master2_sheet: {}", self.master2_sheet());
        info!(logger, "input_sheet: {}", self.master2_sheet());
        info!(logger, "default_gl_code: {}", self.default_gl_code());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file`.")
            .to_string();
        let master1_file = matches
            .value_of("master1_file")
            .expect("Error getting `master1_file`.")
            .to_string();
        let master2_file = matches
            .value_of("master2_file")
            .expect("Error getting `master2_file`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let country = matches
            .value_of("country")
            .expect("Error getting `country`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let delimeter = matches
            .value_of("delimeter")
            .expect("Error getting `delimeter`.")
            .to_string();
        let master1_sheet = matches
            .value_of("master1_sheet")
            .expect("Error getting `master1_sheet`.")
            .to_string();
        let master2_sheet = matches
            .value_of("master2_sheet")
            .expect("Error getting `master2_sheet`.")
            .to_string();
        let input_sheet = matches
            .value_of("input_sheet")
            .expect("Error getting `input_sheet`.")
            .to_string();
        let default_gl_code = matches
            .value_of("default_gl_code")
            .expect("Error getting `default_gl_code`.")
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
            input_file,
            master1_file,
            master2_file,
            output_file,
            country,
            as_on_date,
            delimeter,
            master1_sheet,
            master2_sheet,
            input_sheet,
            default_gl_code,
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
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn master1_file(&self) -> &str {
        &self.master1_file
    }
    pub fn master2_file(&self) -> &str {
        &self.master2_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn delimeter(&self) -> &str {
        &self.delimeter
    }
    pub fn master1_sheet(&self) -> &str {
        &self.master1_sheet
    }
    pub fn master2_sheet(&self) -> &str {
        &self.master2_sheet
    }
    pub fn input_sheet(&self) -> &str {
        &self.input_sheet
    }
    pub fn default_gl_code(&self) -> &str {
        &self.default_gl_code
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
        .about("LPUC MOC Program!!")
        .version("1.0.3267")
        .author("harsh8501 <harsh.sk@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Input File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("master1_file")
                .long("master1-file")
                .value_name("Master1 File")
                .help("Master1 File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("master2_file")
                .long("master2-file")
                .value_name("Master2 File")
                .help("Master2 File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to Output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("country")
                .help("Country.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("delimeter")
                .long("delimeter")
                .value_name("Delimeter")
                .help("The delimeter for which the program has to run.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::with_name("master1_sheet")
                .long("master1-sheet")
                .value_name("master1_sheet")
                .help("Master1 Sheet.")
                .required(true)
        )
        .arg(
            Arg::with_name("master2_sheet")
                .long("master2-sheet")
                .value_name("master2-sheet")
                .help("Master2 Sheet.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sheet")
                .long("input-sheet")
                .value_name("input-sheet")
                .help("Input Sheet.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_gl_code")
                .long("default-gl-code")
                .value_name("default-gl-code")
                .help("Default GL Code.")
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
