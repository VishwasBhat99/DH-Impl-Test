use chrono::NaiveDate;
use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    output_file_path: String,
    sheet_name: String,
    currency_conversion_file_path: String,
    base_currency: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());

        info!(logger, "Sheet Name: {}", self.sheet_name());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error while getting `input file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error while getting `output file path`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error while getting `log file path.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error while getting `sheet-name`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error while getting Exchange Rate file path.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error while getting base currency.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error while getting `diagnostics file path`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting `is perfect diagnostics`.")
            .parse::<bool>()
            .expect("Error while parsing `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            as_on_date,
            currency_conversion_file_path,
            base_currency,
            sheet_name,
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
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
        .about("Care Cr Off bal Moc")
        .version("1.0.4034")
        .author("Pawan Jenu <pawan.j@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("AS-ON-DATE")
                .help("The date the for which the program ran.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
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
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("Base Currency")
                .help("The path to the base currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("sheet_name")
                .long("sheet-name")
                .value_name("Sheet Name")
                .help("Sheet Name")
                .required(true)
        )
        .get_matches()
}
