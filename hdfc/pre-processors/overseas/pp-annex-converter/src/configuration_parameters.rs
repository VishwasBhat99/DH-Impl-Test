use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    input_file_path: String,
    output_file_path: String,
    input_sheet_name: String,
    from_ccy: String,
    to_ccy: String,
    exrt_file_path: String,
    output_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "to_ccy: {}", self.to_ccy());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "from_ccy: {}", self.from_ccy());
        info!(logger, "input_sheet_name: {}", self.input_sheet_name());
        info!(logger, "exrt_file_patg: {}", self.exrt_file_path());
        info!(logger, "output_sheet_name: {}", self.output_sheet_name());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let from_ccy = matches
            .value_of("from_ccy")
            .expect("Error getting `from_ccy`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();

        let input_sheet_name = matches
            .value_of("input_sheet_name")
            .expect("Error getting `input_sheet_name`.")
            .to_string();
        let output_sheet_name = matches
            .value_of("output_sheet_name")
            .expect("Error getting `output_sheet_name`.")
            .to_string();
        let to_ccy = matches
            .value_of("to_ccy")
            .expect("Error getting `to_ccy`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let exrt_file_path = matches
            .value_of("exrt_file_path")
            .expect("Error getting `exrt_file_path`.")
            .to_string();

        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            input_file_path,
            output_file_path,
            input_sheet_name,
            from_ccy,
            to_ccy,
            exrt_file_path,
            output_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn exrt_file_path(&self) -> &str {
        &self.exrt_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
    pub fn input_sheet_name(&self) -> &str {
        &self.input_sheet_name
    }
    pub fn output_sheet_name(&self) -> &str {
        &self.output_sheet_name
    }
    pub fn to_ccy(&self) -> &str {
        &self.to_ccy
    }
    pub fn from_ccy(&self) -> &str {
        &self.from_ccy
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Annex File Converter.")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("INPUT FILE File")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("exrt_file_path")
                .long("exrt-file-path")
                .value_name("EXCHANGE RATE FILE")
                .help("Path to exrt file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("OUTPUT FILE File")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file-path")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file-path")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sheet_name")
                .long("input-sheet-name")
                .value_name("Input SHeet Name")
                .help("Value of input file sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_sheet_name")
                .long("output-sheet-name")
                .value_name("Output SHeet Name")
                .help("Value of output file sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("to_ccy")
                .long("to-ccy")
                .value_name("Currency to which amount should be converted")
                .help("Value of currency to which amount should be converted.")
                .required(true)
        )
        .arg(
            Arg::with_name("from_ccy")
                .long("from-ccy")
                .value_name("Currency from which amount should be converted")
                .help("Value of currency from which amount should be converted.")
                .required(true)
        )
        .get_matches()
}
