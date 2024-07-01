use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub int_rate_file_path: String,
    pub ratecode_file_path: String,
    pub tbl_code_file_path: String,
    pub npa_file_path: String,
    pub config_file_path: String,
    pub additional_loan_file: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "int_rate_file: {}", self.int_rate_file_path());
        info!(logger, "npa_file_path: {}", self.npa_file_path());
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "ratecode_file_path: {}", self.ratecode_file_path());
        info!(logger, "tbl_code_file: {}", self.tbl_code_file_path());
        info!(logger, "additional_loan_file: {}", self.additional_loan_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let int_rate_file_path = matches
            .value_of("int_rate_file")
            .expect("Error getting `int_rate_file` value.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path` value.")
            .to_string();
        let npa_file_path = matches
            .value_of("npa_file_path")
            .expect("Error getting `npa_file_path` value.")
            .to_string();
        let additional_loan_file = matches
            .value_of("additional_loan_file")
            .expect("Error getting `additional_loan_file` value.")
            .to_string();
        let ratecode_file_path = matches
            .value_of("ratecode_file_path")
            .expect("Error getting `ratecode_file_path` value.")
            .to_string();
        let tbl_code_file_path = matches
            .value_of("tbl_code_file")
            .expect("Error getting `tbl_code_file` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
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
            input_file_path,
            int_rate_file_path,
            npa_file_path,
            config_file_path,
            as_on_date,
            ratecode_file_path,
            tbl_code_file_path,
            additional_loan_file,
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
        &self.input_file_path
    }
    pub fn int_rate_file_path(&self) -> &str {
        &self.int_rate_file_path
    }
    pub fn npa_file_path(&self) -> &str {
        &self.npa_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn ratecode_file_path(&self) -> &str {
        &self.ratecode_file_path
    }
    pub fn tbl_code_file_path(&self) -> &str {
        &self.tbl_code_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
    pub fn additional_loan_file(&self) -> &str {
        &self.additional_loan_file
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates preprocessor output for Finacle-OD.")
        .version("1.1.5319")
        .author("Saurabh Singh <saurabh.s@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("int_rate_file")
                .short("ir")
                .long("int-rate-file")
                .value_name("INT_RATE_FILE")
                .help("Path to int rate file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .short("cf")
                .long("config-file-path")
                .value_name("CONFIG_FILE_PATH")
                .help("Path to int config file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_file_path")
                .short("npa")
                .long("npa-file-path")
                .value_name("NPA_FILE_PATH")
                .help("Path to npa file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ratecode_file_path")
                .short("rc")
                .long("ratecode-file-path")
                .value_name("ratecode_file_path")
                .help("Path to rate code file.")
                .required(true)
        )
        .arg(
            Arg::with_name("tbl_code_file")
                .short("tb")
                .long("tbl-code-file")
                .value_name("TBL_CODE_FILE")
                .help("Path to tbl code file file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("additional_loan_file")
                .short("a")
                .long("additional-loan-file")
                .value_name("additional_loan_file")
                .help("Path to additional loan file file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics log.")
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
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
