use std::borrow::BorrowMut;

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
    input_file_path: String,
    input_sheet_name: String,
    output_file_path: String,
    excel_config_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    subsidiary_id: String,
    denomination: String,
    currency: String,
    as_on_date: NaiveDate,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_consolidated_flag: Option<bool>,
    exchange_rate_file_path: Option<String>,
    base_ccy: Option<String>,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "input_sheet_name: {}", self.input_sheet_name());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "excel_config_file: {}", self.excel_config_file());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "subsidiary_id: {}", self.subsidiary_id());
        info!(logger, "Denomination: {}", self.denomination());
        info!(logger, "currency: {}", self.currency());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "is_consolidated_flag: {:#?}",
            self.is_consolidated_flag()
        );
        info!(
            logger,
            "exchange_rate_file_path: {:#?}",
            self.exchange_rate_file_path()
        );
        info!(logger, "base_ccy: {:#?}", self.base_ccy());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let input_sheet_name = matches
            .value_of("input_sheet_name")
            .expect("Error getting `input_sheet_name` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let excel_config_file = matches
            .value_of("excel_config_file")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let subsidiary_id = matches
            .value_of("subsidiary_id")
            .expect("Error getting `subsidiary_id`.")
            .to_string();
        let denomination = matches
            .value_of("denomination")
            .expect("Error getting `denomination`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let is_consolidated_flag = matches
            .value_of("is_consolidated_flag")
            .map(|s| s.parse().ok())
            .flatten();

        let exchange_rate_file_path = matches
            .value_of("exchange_rate_file_path")
            .map(|s| s.to_string());

        let base_ccy = matches.value_of("base_ccy").map(|s| s.to_string());

        ConfigurationParameters {
            input_file_path,
            input_sheet_name,
            output_file_path,
            excel_config_file,
            log_file_path,
            diagnostics_file_path,
            subsidiary_id,
            currency,
            as_on_date,
            log_level,
            is_perf_diagnostics_enabled,
            exchange_rate_file_path,
            is_consolidated_flag,
            base_ccy,
            denomination,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn input_sheet_name(&self) -> &str {
        &self.input_sheet_name
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn excel_config_file(&self) -> &str {
        &self.excel_config_file
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn subsidiary_id(&self) -> &str {
        &self.subsidiary_id
    }
    pub fn denomination(&self) -> &str {
        &self.denomination
    }
    pub fn currency(&self) -> &str {
        &self.currency
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
    pub fn is_consolidated_flag(&self) -> Option<bool> {
        self.is_consolidated_flag
    }
    pub fn exchange_rate_file_path(&self) -> &Option<String> {
        &self.exchange_rate_file_path
    }
    pub fn base_ccy(&self) -> &Option<String> {
        &self.base_ccy
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.2.5206")
        .about("Subsidiary LCR upload program.")
        .version("1.0.5270")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to Input File.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sheet_name")
                .long("input-sheet-name")
                .value_name("SHEET NAME")
                .help("Sheet name of input file.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to Output File.")
                .required(true)
        )
        .arg(
            Arg::with_name("excel_config_file")
                .long("excel-config-file")
                .value_name("Excel Config File Path")
                .help("Path to Excel Config File.")
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
            Arg::with_name("subsidiary_id")
                .long("subsidiary-id")
                .value_name("Subsidiary ID")
                .help("Specifies the customer ID.")
                .required(true)
        )
        .arg(
            Arg::with_name("denomination")
                .long("denomination")
                .value_name("Denomination")
                .help("Specifies the Denomination.")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("Currency")
                .help("Specifies the Currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("Date")
                .help("The date for which program has to run.")
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
            Arg::with_name("base_ccy")
                .long("base-ccy")
                .value_name("BASE CCY")
                .help("This will help to get base currency.")
                .default_value("INR")
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
            Arg::with_name("exchange_rate_file_path")
                .long("exchange-rate-file-path")
                .value_name("Exchange rate file path")
                .help("Path to exchange rate.")
                .required(false)
        )
        .arg(
            Arg::with_name("is_consolidated_flag")
                .long("is-consolidated-flag")
                .value_name("IS CONSOLIDATED FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides weather exchange rate logic is applied or not.")
                .default_value("true")
                .required(false)
        )
        .get_matches()
}
