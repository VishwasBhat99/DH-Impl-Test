use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    config_file_path: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    src_local_ccy: String,
    display_local_ccy: String,
    consol_ccy: String,
    currency_conversion_file_path: String,
    default_llg_code: i32,
    default_overdue_llg_code: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "consolidation_currency: {}",
            self.display_local_ccy()
        );
        info!(logger, "base_currency: {}", self.src_local_ccy());
        info!(
            logger,
            "local_consolidation_currency: {}",
            self.consol_ccy()
        );
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(
            logger,
            "default_overdue_llg_code: {}",
            self.default_overdue_llg_code()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `Output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `General log file path`.")
            .to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error getting `config_file_path`.")
            .to_string();
        let src_local_ccy = matches
            .value_of("src_local_ccy")
            .expect("Error getting `src local currency`.")
            .to_string();
        let display_local_ccy = matches
            .value_of("display_local_ccy")
            .expect("Error getting `display local currency`.")
            .to_string();
        let consol_ccy = matches
            .value_of("consol_ccy")
            .expect("Error getting `Local consolidation currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `File level exchange rate file path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `Diagnostics log file path`.")
            .to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error while getting `default llg code`.")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default llg code` as integer.");
        let default_overdue_llg_code = matches
            .value_of("default_overdue_llg_code")
            .unwrap_or("0")
            .to_string()
            .parse::<i32>()
            .expect("Error while parsing `default overdue llg code` as integer.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting ``.")
            .parse::<bool>()
            .expect("Error while parsing `is perf diagnostics enabled` as bool.");

        ConfigurationParameters {
            config_file_path,
            output_file_path,
            as_on_date,
            src_local_ccy,
            display_local_ccy,
            consol_ccy,
            currency_conversion_file_path,
            default_llg_code,
            default_overdue_llg_code,
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
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn src_local_ccy(&self) -> &str {
        &self.src_local_ccy
    }
    pub fn display_local_ccy(&self) -> &str {
        &self.display_local_ccy
    }
    pub fn consol_ccy(&self) -> &str {
        &self.consol_ccy
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
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
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn default_overdue_llg_code(&self) -> i32 {
        self.default_overdue_llg_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("BALM Aggregator.")
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file")
                .value_name("Config File")
                .help("Path to the config file.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
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
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("src_local_ccy")
                .long("src-local-ccy")
                .value_name("SOURCE CURRENCY")
                .help("The source currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("display_local_ccy")
                .long("display-local-ccy")
                .value_name("DISPLAY LOCAL CURRENCY")
                .help("The local display currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("consol_ccy")
                .long("consol-ccy")
                .value_name("CONSOLIDATION CURRENCY")
                .help("If the currency is XYZ we use this as consolidation currency")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_overdue_llg_code")
                .long("default-overdue-llg-code")
                .value_name("DEFAULT OVERDUE LLG CODE")
                .help("This is the default overdue llg code.")
                .required(false)
        )
        .get_matches()
}
