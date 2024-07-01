use chrono::{Local, NaiveDate};
use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use sdb_day_convention::conventions::Conventions;
use slog::Logger;
use std;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    num_threads: u8,
    batch_size: u32,
    output_file_path: String,
    as_on_date: NaiveDate,
    day_convention: Conventions,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_contractual: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "is_contractual: {}", self.is_contractual());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "num_threads: {}", self.num_threads());
        info!(logger, "batch_size: {}", self.batch_size());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "day_convention: {:?}", self.day_convention());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "as_on_date: {}", self.as_on_date());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` parameter value.")
            .to_string();
        let num_threads = matches
            .value_of("num_threads")
            .expect("Error getting `num_threads` parameter value.")
            .parse::<u8>()
            .expect("Error parsing `num_threads` value as u8.");
        let batch_size = matches
            .value_of("batch_size")
            .expect("Error getting `batch_size` parameter value.")
            .parse::<u32>()
            .expect("Error parsing `batch_size` value as u32.");
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` parameter value.")
            .to_string();
        let day_convention = {
            let conv = matches
                .value_of("day_convention")
                .expect("Error getting `day_convention` value.");
            match conv {
                "ACT/ACT" => {
                    Conventions::ACTbyACT
                }
                "ACT/365" => {
                    Conventions::ACTby365
                }
                "ACT/360" => {
                    Conventions::ACTby360
                }
                "30/360" => {
                    Conventions::Thirtyby360
                }
                _ => {
                    panic!("Incorrect day convention parameter passed. Must be one of { ACT/ACT, ACT/365, ACT/360, 30/360 }")
                }
            }
        };
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Cannot parse `as_on_date` parameter value."),
        );

        let timestamp = Local::now()
            .naive_local()
            .format("%d%m%Y_%H%M%S")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        log_file_path = log_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let mut diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        diagnostics_file_path = diagnostics_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` parameter value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` parameter value.")
            .parse::<bool>()
            .expect("Error parsing `perf_diag_flag` value as bool.");

        let is_contractual = matches
            .value_of("is_contractual")
            .expect("Error getting `is_contractual` parameter value.")
            .parse::<bool>()
            .expect("Error parsing `is_contractual` value as bool.");

        ConfigurationParameters {
            input_file_path,
            num_threads,
            batch_size,
            output_file_path,
            as_on_date,
            day_convention,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_contractual,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn num_threads(&self) -> u8 {
        self.num_threads
    }
    pub fn batch_size(&self) -> u32 {
        self.batch_size
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn day_convention(&self) -> &Conventions {
        &self.day_convention
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn as_on_date(&self) -> NaiveDate {
        self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn is_contractual(&self) -> bool {
        self.is_contractual
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates Cashflows for Finnone Loans!")
        .version("1.0.4933")
        .author("shashank.676 <shashank.p@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("num_threads")
                .short("t")
                .long("num-threads")
                .value_name("NUM THREADS")
                .help("Number of threads to create to process input file.")
                .validator(small_positive_int_validator)
                .required(true)
        )
        .arg(
            Arg::with_name("batch_size")
                .short("b")
                .long("batch-size")
                .value_name("BATCH SIZE")
                .help("Batch size to process input file.")
                .validator(large_positive_int_validator)
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general log file.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .short("e")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("is_contractual")
                .long("is-contractual")
                .value_name("IS CONTRACTUAL")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to write contractual cfs or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("day_convention")
                .short("C")
                .long("day-convention")
                .value_name("CONVENTION")
                .help("The convention to be used for interest calculation.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}

fn small_positive_int_validator(arg: String) -> Result<(), String> {
    let int_arg = arg.parse::<u8>();
    match int_arg {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "'{}' is not an integer value between {} and {}.",
            arg,
            std::u8::MIN,
            std::u8::MAX
        )),
    }
}

fn large_positive_int_validator(arg: String) -> Result<(), String> {
    let int_arg = arg.parse::<u32>();
    match int_arg {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "'{}' is not an integer value between {} and {}.",
            arg,
            std::u32::MIN,
            std::u32::MAX
        )),
    }
}
