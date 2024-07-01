use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    as_on_date: NaiveDate,
    existing_business_value: f64,
    prj_business_value: f64,
    currency: String,
    disbursement_by_day_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "existing_business_value: {}",
            self.existing_business_value()
        );
        info!(logger, "prj_business_value: {}", self.prj_business_value());
        info!(logger, "currency: {}", self.currency());
        info!(
            logger,
            "disbursement_by_day_file_path: {}",
            self.disbursement_by_day_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let existing_business_value = matches
            .value_of("existing_business_value")
            .expect("Error getting `existing_business_value`.")
            .parse::<f64>()
            .expect("Cannot parse `existing_business_value` as f64.");
        let prj_business_value = matches
            .value_of("prj_business_value")
            .expect("Error getting `prj_business_value`.")
            .parse::<f64>()
            .expect("Cannot parse `prj_business_value` as f64.");
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let disbursement_by_day_file_path = matches
            .value_of("disbursement_by_day_file_path")
            .expect("Error getting `disbursement_by_day_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
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
            as_on_date,
            existing_business_value,
            prj_business_value,
            currency,
            disbursement_by_day_file_path,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn existing_business_value(&self) -> f64 {
        self.existing_business_value
    }
    pub fn prj_business_value(&self) -> f64 {
        self.prj_business_value
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn disbursement_by_day_file_path(&self) -> &str {
        &self.disbursement_by_day_file_path
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
        .about("Generate Hypothetical Non Maturity Accounts!!")
        .version("1.0.0")
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("existing_business_value")
                .long("existing-business-value")
                .value_name("Existing Business Value")
                .help("Value of existing business.")
                .required(true)
        )
        .arg(
            Arg::with_name("prj_business_value")
                .long("prj-business-value")
                .value_name("Projected Business Value")
                .help("Projected business value.")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("Currency")
                .help("Currency of New Business.")
                .required(true)
        )
        .arg(
            Arg::with_name("disbursement_by_day_file_path")
                .long("disbursement-by-day-file-path")
                .value_name("Disbursement by Day File Path")
                .help("Path to Disbursement by Day File.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to Output File.")
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
