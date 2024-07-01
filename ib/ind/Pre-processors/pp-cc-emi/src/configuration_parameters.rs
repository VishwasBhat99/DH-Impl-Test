use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub input_file_path: String,
    pub output_file_path: String,
    pub master_file_path: String,
    pub bgl_cgl_file_path: String,
    pub sheet_name: String,
    pub currency: String,
    pub branch_code: String,
    pub int_rate: f64,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "bgl_cgl_file_path: {}", self.bgl_cgl_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "currency: {}", self.currency());
        info!(logger, "branch_code: {}", self.branch_code());
        info!(logger, "int_rate: {}", self.int_rate());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
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
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let master_file_path = matches
            .value_of("master_file")
            .expect("Error getting `master_file_path`.")
            .to_string();
        let bgl_cgl_file_path = matches
            .value_of("bgl_cgl_file")
            .expect("Error getting `bgl_cgl_file_path`.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let branch_code = matches
            .value_of("branch_code")
            .expect("Error getting `branch_code`.")
            .to_string();
        let int_rate = matches
            .value_of("int_rate")
            .expect("Error getting `int_rate`.")
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            output_file_path,
            master_file_path,
            bgl_cgl_file_path,
            sheet_name,
            currency,
            branch_code,
            int_rate,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn bgl_cgl_file_path(&self) -> &str {
        &self.bgl_cgl_file_path
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn branch_code(&self) -> &str {
        &self.branch_code
    }
    pub fn int_rate(&self) -> &f64 {
        &self.int_rate
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("Pre Processor EMI Credit Card for IB")
        .author("Ankur Gangwar<ankur.g@surya-soft.com>")
        .version("1.0.3313")
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to read Input.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to write Output.")
                .required(true)
        )
        .arg(
            Arg::new("master_file")
                .long("master-file")
                .value_name("Master File Path")
                .help("Path to write Master.")
                .required(true)
        )
        .arg(
            Arg::new("bgl_cgl_file")
                .long("bgl-cgl-file")
                .value_name("bgl-cgl File Path")
                .help("Path to write bgl_cgl file.")
                .required(true)
        )
        .arg(
            Arg::new("sheet_name")
                .long("sheet-name")
                .value_name("Sheet Name")
                .help("Sheet Name")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::new("currency")
                .long("currency")
                .value_name("currency")
                .help("The currency the program assumes.")
                .required(true)
        )
        .arg(
            Arg::new("int_rate")
                .long("int-rate")
                .value_name("Interest Rate")
                .help("The interest rate the program assumes.")
                .default_value("0.0")
                .required(false)
        )
        .arg(
            Arg::new("branch_code")
                .long("branch-code")
                .value_name("Branch Code")
                .help("The branch code the program assumes.")
                .default_value("0")
                .required(false)
        )
        .get_matches()
}
