use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    sfr_file_path: String,
    sfr_sheet_name: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    base_currency: String,
    denomination_type: String,
    amt_col: usize,
    country: String,
    log_file_path: String,
    default_llg_code: i32,
    diagnostics_file_path: String,
    log_level: String,
    is_negative: bool,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "sfr_file: {}", self.sfr_file_path());
        info!(logger, "sfr_sheet_name: {}", self.sfr_sheet_name());
        info!(logger, "amount column: {}", self.amt_col());
        info!(logger, "denomination_type: {}", self.denomination_type());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "country: {}", self.country());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "is_negative: {}", self.is_negative());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let sfr_file_path = matches
            .value_of("sfr_file")
            .expect("Error getting `sfr_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let sfr_sheet_name = matches
            .value_of("sfr_sheet_name")
            .expect("Error getting `sfr_sheet_name`.")
            .to_string();
        let amt_col = matches
            .value_of("amt_col")
            .expect("Error getting `amt_col`.")
            .parse::<usize>()
            .expect("Invalid amount column id.");
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
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

        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let denomination_type = matches
            .value_of("denomination_type")
            .expect("Error getting `denomination_type`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error getting `default_llg_code`.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_llg_code` as i64.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let is_negative = matches
            .value_of("is_negative")
            .expect("Error getting `is_negative`.")
            .parse::<bool>()
            .expect("Cannot parse `is_negative` as bool.");

        ConfigurationParameters {
            sfr_file_path,
            sfr_sheet_name,
            output_file_path,
            as_on_date,
            base_currency,
            denomination_type,
            amt_col,
            country,
            log_file_path,
            default_llg_code,
            diagnostics_file_path,
            log_level,
            is_negative,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn sfr_file_path(&self) -> &str {
        &self.sfr_file_path
    }
    pub fn sfr_sheet_name(&self) -> &str {
        &self.sfr_sheet_name
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn amt_col(&self) -> usize {
        self.amt_col
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn denomination_type(&self) -> &str {
        &self.denomination_type
    }
    pub fn country(&self) -> &str {
        &self.country
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
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("NSFR CRR")
        .arg(
            Arg::with_name("sfr_file")
                .long("sfr-file")
                .value_name("SFR File")
                .help("Path to the sfr file.")
                .required(true)
        )
        .arg(
            Arg::with_name("sfr_sheet_name")
                .long("sfr-sheet-name")
                .value_name("SFR Sheet Name")
                .help("SFR file sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("amt_col")
                .long("amt-col")
                .value_name("Amount Column")
                .help("Amount column of sfr sheet.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("Country")
                .help("Country instance name.")
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
        .arg(
            Arg::with_name("is_negative")
                .long("is-negative")
                .value_name("IS NEGATIVE FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether final amount has to multiplied with -1 or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("BASE CURRENCY")
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("denomination_type")
                .long("denomination-type")
                .value_name("Denominatio Type")
                .possible_values(&["CR", "L", "U"])
                .help("The Denomination type.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .get_matches()
}
