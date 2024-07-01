use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_bloom_file_path: String,
    pub exchange_file_path: String,
    pub face_value: f64,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub order_number: i64,
    pub transaction_type: String,
    pub defalut_ccy: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(
            logger,
            "input_bloom_file_path: {}",
            self.input_bloom_file_path()
        );
        info!(logger, "exchange_file_path: {}", self.exchange_file_path());
        info!(logger, "face_value: {}", self.face_value());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "order_number: {}", self.order_number());
        info!(logger, "transaction_type: {}", self.transaction_type());
        info!(logger, "defalut_ccy: {}", self.defalut_ccy());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_bloom_file_path = matches
            .value_of("input_bloom_file_path")
            .expect("Error getting `input_bloom_file_path` value.")
            .to_string();
        let exchange_file_path = matches
            .value_of("exchange_file_path")
            .expect("Error getting `exchange_file_path` value.")
            .to_string();
        let face_value = matches
            .value_of("face_value")
            .expect("Error getting `face_value` value.")
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        let order_number = matches
            .value_of("order_number")
            .expect("Error getting `order_number` value.")
            .to_string()
            .parse::<i64>()
            .unwrap_or(1001);
        let transaction_type = matches
            .value_of("transaction_type")
            .expect("Error getting `transaction_type` value.")
            .to_string();
        let defalut_ccy = matches
            .value_of("defalut_ccy")
            .expect("Error getting `defalut_ccy` value.")
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
            .value_of("log_file_path")
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
            input_bloom_file_path,
            exchange_file_path,
            face_value,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            order_number,
            transaction_type,
            defalut_ccy,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_bloom_file_path(&self) -> &str {
        &self.input_bloom_file_path
    }
    pub fn exchange_file_path(&self) -> &str {
        &self.exchange_file_path
    }
    pub fn face_value(&self) -> &f64 {
        &self.face_value
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn order_number(&self) -> &i64 {
        &self.order_number
    }
    pub fn transaction_type(&self) -> &str {
        &self.transaction_type
    }
    pub fn defalut_ccy(&self) -> &str {
        &self.defalut_ccy
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates basel blr05 equity bond output.")
        .version("1.0.4244")
        .author("Tanuj <tanuj.s@surya-soft.com>")
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("Log File")
                .help("Path to write logs.")
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
            Arg::with_name("input_bloom_file_path")
                .long("input-bloom-file-path")
                .value_name("Input bloom File Path")
                .help("Path to the Input bloom file.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_file_path")
                .long("exchange-file-path")
                .value_name("exchange_file_path")
                .help("Path to the exchange file path file.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("face_value")
                .long("face-value")
                .value_name("Face Value")
                .help("Face Amount.")
                .required(true)
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
            Arg::with_name("order_number")
                .long("order-number")
                .value_name("Order Number")
                .help("Order Number.")
                .required(true)
        )
        .arg(
            Arg::with_name("transaction_type")
                .long("transaction-type")
                .value_name("Order Number")
                .help("Order Number.")
                .required(true)
        )
        .arg(
            Arg::with_name("defalut_ccy")
                .long("default-ccy")
                .value_name("Defalut Ccy")
                .help("Defalut Ccy.")
                .required(true)
        )
        .get_matches()
}
