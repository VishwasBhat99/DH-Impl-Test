use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use sdb_day_convention::Conventions;
use slog::Logger;

pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    currency: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    day_convention: Conventions,
    is_adj_cf_req: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "is_adj_cf_req: {}", self.is_adj_cf_req());
        info!(logger, "day_convention: {:?}", self.day_convention());
        info!(logger, "currency: {}", self.currency());
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
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
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
        let is_adj_cf_req = matches
            .value_of("is_adj_cf_req")
            .expect("Error getting `is_adj_cf_req` as true/false`.")
            .parse::<bool>()
            .expect("Cannot parse `is_adj_cf_req` as bool.");
        let day_convention = {
            let conv = matches
                .value_of("day_convention")
                .expect("Error getting `day_convention` value.")
                .to_string();
            match &conv[..] {
                "ACT/ACT" => Conventions::ACTbyACT,
                "ACT/365" => Conventions::ACTby365,
                "ACT/360" => Conventions::ACTby360,
                "30/360" => Conventions::Thirtyby360,
                _ => {
                    panic!("Incorrect day convention parameter passed:- Must be one of ACT/ACT, ACT/365, ACT/360, 30/360")
                }
            }
        };

        ConfigurationParameters {
            input_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            day_convention,
            is_adj_cf_req,
            currency,
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
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn day_convention(&self) -> &Conventions {
        &self.day_convention
    }
    pub fn is_adj_cf_req(&self) -> bool {
        self.is_adj_cf_req
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
}

fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("This program will generate the cf file for refinance (SIB)")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("input_file")
                .long("input-file-path")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file-path")
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
            Arg::with_name("is_adj_cf_req")
                .long("is-adj-cf-req")
                .value_name("IS_ADJ_CF_REQ")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to remaining amount to cf-vec on mat-date.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("CURRENCY")
                .help("Currency to be Stamped.")
                .default_value("INR")
                .required(false)
        )
        .arg(
            Arg::with_name("day_convention")
                .long("day-convention")
                .value_name("CONVENTION")
                .help("The convention to be used for interest calculation.")
                .default_value("ACT/ACT")
                .required(false)
        )
        .get_matches()
}
