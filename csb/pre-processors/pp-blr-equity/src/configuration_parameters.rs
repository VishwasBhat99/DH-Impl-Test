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
    input_file_path: String,
    output_file_path: String,
    fields_file_path: String,
    symbol: String,
    face_value: String,
    as_on_date: NaiveDate,
    exchange_rate_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "fields_file: {}", self.fields_file_path());
        info!(logger, "symbol: {}", self.symbol());
        info!(logger, "face value: {}", self.face_value());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(
            logger,
            "exchange_rate_file: {}",
            self.exchange_rate_file_path()
        );
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
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
        let fields_file_path = matches
            .value_of("fields_file")
            .expect("Error getting `fields_file` value.")
            .to_string();
        let symbol = matches
            .value_of("symbol")
            .expect("Error getting `symbol` value.")
            .to_string();
        let face_value = matches
            .value_of("face_value")
            .expect("Error getting `face_value` value.")
            .to_string();
        let exchnage_rate_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file` value.")
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
            output_file_path,
            fields_file_path,
            symbol,
            face_value,
            as_on_date,
            log_file_path,
            exchange_rate_file_path: exchnage_rate_file_path,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }

    pub fn fields_file_path(&self) -> &str {
        &self.fields_file_path
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    pub fn face_value(&self) -> &str {
        &self.face_value
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn exchange_rate_file_path(&self) -> &str {
        &self.exchange_rate_file_path
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
        .about("This app generates basel blr05 equity output")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
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
            Arg::with_name("fields_file")
                .long("fields-file")
                .value_name("Fields File")
                .help("Path to the config fields file.")
                .required(true)
        )
        .arg(
            Arg::with_name("symbol")
                .long("symbol")
                .value_name("Symbol value")
                .help("value for Symbol")
                .required(true)
        )
        .arg(
            Arg::with_name("face_value")
                .long("face-value")
                .value_name("Face value")
                .help("value for Face Value")
                .required(true)
        )
        .arg(
        Arg::with_name("exchange_rate_file")
            .long("exchange-rate-file")
            .value_name("EXCHANGE RATE FILE")
            .help("Exchange rate file path")
            .default_value("")
            .required(false)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Diagnostics log file path")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which program has to run.")
                .required(true)
        )

        .get_matches()
}
