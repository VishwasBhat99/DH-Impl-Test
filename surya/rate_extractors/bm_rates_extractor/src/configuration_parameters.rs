use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    skip_bmid_vec: Vec<String>,
    skip_date_vec: Vec<String>,
    log_file_path: String,
    diagnostics_file_path: String,
    no_avg_days: usize,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "skip_date_bmid: {:?}", self.skip_bmid_vec());
        info!(logger, "skip_date_vec: {:?}", self.skip_date_vec());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "no_avg_days: {}", self.no_avg_days());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_value`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error while getting `as_on_date`."),
        );
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let no_avg_days = matches
            .value_of("no_avg_days")
            .expect("Error getting `no_avg_days  as usize`.")
            .parse::<usize>()
            .expect("Cannot parse `no_avg_days` as usize.");
        let skip_date_vec: Vec<String> = matches
            .value_of("skip_date_vec")
            .expect("Error getting `skip_date_vec`.")
            .to_string()
            .split(',')
            .map(|date| date.to_string())
            .collect();
        let skip_bmid_vec: Vec<String> = matches
            .value_of("skip_bmid_vec")
            .expect("Error getting `skip_bmid_vec`.")
            .to_string()
            .split(',')
            .map(|bmid| bmid.to_string())
            .collect();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            as_on_date,
            skip_bmid_vec,
            skip_date_vec,
            no_avg_days,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
    pub fn no_avg_days(&self) -> &usize {
        &self.no_avg_days
    }
    pub fn skip_bmid_vec(&self) -> &Vec<String> {
        &self.skip_bmid_vec
    }
    pub fn skip_date_vec(&self) -> &Vec<String> {
        &self.skip_date_vec
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("2.0.2613")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>, VishwasBhat99 <vishwas.b@surya-soft.com>")
        .about("Benchmark Rates Filler.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("no_avg_days")
                .long("no-avg-days")
                .value_name("NO AVG DAYS")
                .default_value("1")
                .help("The number of days for which the program has to calculate average of rates.")
                .required(false)
        )
        .arg(
            Arg::with_name("skip_bmid_vec")
                .long("skip-bmid-vec")
                .value_name("Skip BMID's")
                .help("This value tells about the BMID's to be skipped from processing for skipped dates")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::with_name("skip_date_vec")
                .long("skip-date-vec")
                .value_name("Skip Dates")
                .help("This value tells about the Dates to be skipped from processing")
                .default_value("")
                .required(false)
        )
        .get_matches()
}
