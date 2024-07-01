use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);

    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    day_convention: Conventions,
    curr_bal_accs_prefix: Vec<String>,
    int_proj_accs_prefix: Vec<String>,
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
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "day_convention: {:?}", self.day_convention());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "curr_bal_accs_prefix: {:?}",
            self.curr_bal_accs_prefix()
        );
        info!(
            logger,
            "int_proj_accs_prefix: {:?}",
            self.int_proj_accs_prefix()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input-file-path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser
            .parse_opt(
                matches
                    .value_of("as_on_date")
                    .expect("Error getting `as_on_date` value."),
            )
            .expect("Cannot parse `as_on_date` value as `DD-MM-YYYY` format");
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
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
            .expect("Cannot parse `perf_diag_flag` value as bool type");
        let curr_bal_accs_prefix: Vec<String> = matches
            .value_of("curr_bal_accs_prefix")
            .expect("Error getting `curr_bal_accs_prefix`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let int_proj_accs_prefix: Vec<String> = matches
            .value_of("int_proj_accs_prefix")
            .expect("Error getting `int_proj_accs_prefix`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
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
                    panic!( "{}","Incorrect day convention parameter passed:- Must be one of { ACT/ACT, ACT/365, ACT/360, 30/360")
                }
            }
        };

        ConfigurationParameters {
            input_file_path,
            curr_bal_accs_prefix,
            as_on_date,
            output_file_path,
            day_convention,
            log_file_path,
            diagnostics_file_path,
            log_level,
            int_proj_accs_prefix,
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
    pub fn day_convention(&self) -> &Conventions {
        &self.day_convention
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn curr_bal_accs_prefix(&self) -> &Vec<String> {
        &self.curr_bal_accs_prefix
    }
    pub fn int_proj_accs_prefix(&self) -> &Vec<String> {
        &self.int_proj_accs_prefix
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates Cashflows for Term Deposits!")
        .version("1.0.4570")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .short("I")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("O")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("curr_bal_accs_prefix")
                .long("curr-bal-accs-prefix")
                .value_name("CURR BAL ACCS PREFIX")
                .help("Vector of Accs-Prefixes for which (curr_bal = currbal + int_available).")
                .required(true)
        )
        .arg(
            Arg::with_name("int_proj_accs_prefix")
                .long("int-proj-accs-prefix")
                .value_name("INT PROJ ACCS PREFIX")
                .help("Vector of Accs-Prefixes for which (int_proj = currbal + int_available).")
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
                .short("d")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("day_convention")
                .long("day-convention")
                .value_name("CONVENTION")
                .help("The convention to be used for interest calculation.")
                .required(true)
        )
        .get_matches()
}
