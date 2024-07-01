use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub ex_rt_file_path: String,
    pub config_file_path: String,
    pub ccy: String,
    pub lcy: String,
    pub fcy: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "exchange_rate_file: {}", self.ex_rt_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "consolidated currency: {}", self.ccy());
        info!(logger, "local consolidated currency: {}", self.lcy());
        info!(logger, "foreign currency: {}", self.fcy());
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
        let config_file_path = matches
            .value_of("config_file")
            .expect("Error getting `config_file` value.")
            .to_string();
        let ex_rt_file_path = matches
            .value_of("ex_rt_file")
            .expect("Error getting `exchange_rate_file` value.")
            .to_string();
        let ccy = matches
            .value_of("ccy")
            .expect("Error getting `ccy` value.")
            .to_string();
        let lcy = matches
            .value_of("lcy")
            .expect("Error getting `lcy` value.")
            .to_string();
        let fcy = matches
            .value_of("fcy")
            .expect("Error getting `fcy` value.")
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
            as_on_date,
            output_file_path,
            ex_rt_file_path,
            config_file_path,
            ccy,
            lcy,
            fcy,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn ex_rt_file_path(&self) -> &str {
        &self.ex_rt_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn ccy(&self) -> &str {
        &self.ccy
    }
    pub fn lcy(&self) -> &str {
        &self.lcy
    }
    pub fn fcy(&self) -> &str {
        &self.fcy
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
        .about("Pre-processor for Exchange Rate.")
        .version("1.0.5132")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
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
            Arg::with_name("ex_rt_file")
                .long("ex-rt-file")
                .value_name("Exchange Rate File")
                .help("Path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .value_name("Config File")
                .help("Path to the config file.")
                .required(true)
        )
        .arg(
            Arg::with_name("fcy")
                .long("fcy")
                .value_name("fcy")
                .help("Foreign Currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("ccy")
                .long("ccy")
                .value_name("ccy")
                .help("Consolidated Currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcy")
                .long("lcy")
                .value_name("Local Consolidated Currency")
                .help("Local Consolidated Currency.")
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
                .value_name("Diagnostic Log File")
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
        .get_matches()
}
