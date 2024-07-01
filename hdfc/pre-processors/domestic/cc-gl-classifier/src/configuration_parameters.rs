use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub input_file: String,
    pub master_file: String,
    pub master_file_sheet_name: String,
    pub country_name: String,
    pub ccy: String,
    pub source_names: Vec<String>,
    pub rf_llg: String,
    pub b1_llg: String,
    pub b2_llg: String,
    pub b3_llg: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "master_file: {}", self.master_file());
        info!(
            logger,
            "master_file_sheet_name: {}",
            self.master_file_sheet_name()
        );
        info!(logger, "country_name: {}", self.country_name);
        info!(logger, "ccy: {}", self.ccy);
        info!(logger, "source_names: {:?}", self.source_names);
        info!(logger, "rf_llg: {}", self.rf_llg);
        info!(logger, "b1_llg: {}", self.b1_llg);
        info!(logger, "b2_llg: {}", self.b2_llg);
        info!(logger, "b3_llg: {}", self.b3_llg);
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "final_output_file: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file`.")
            .to_string();
        let master_file = matches
            .value_of("master_file")
            .expect("Error getting `master_file`.")
            .to_string();
        let master_file_sheet_name = matches
            .value_of("master_file_sheet_name")
            .expect("Error getting `master_file_sheet_name`.")
            .to_string();
        let country_name = matches
            .value_of("country_name")
            .expect("Error getting `Country Name`.")
            .to_string();
        let ccy = matches
            .value_of("ccy")
            .expect("Error getting `CCY`.")
            .to_string();
        let source_names = matches
            .value_of("source_names")
            .expect("Error getting `Source Names`.")
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let rf_llg = matches
            .value_of("rf_llg")
            .expect("Error getting `RF LLG`.")
            .to_string();
        let b1_llg = matches
            .value_of("b1_llg")
            .expect("Error getting `b1 LLG`.")
            .to_string();
        let b2_llg = matches
            .value_of("b2_llg")
            .expect("Error getting `b2 LLG`.")
            .to_string();
        let b3_llg = matches
            .value_of("b3_llg")
            .expect("Error getting `b3 LLG`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
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
            input_file,
            master_file,
            master_file_sheet_name,
            country_name,
            ccy,
            source_names,
            rf_llg,
            b1_llg,
            b2_llg,
            b3_llg,
            as_on_date,
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
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn master_file(&self) -> &str {
        &self.master_file
    }
    pub fn master_file_sheet_name(&self) -> &str {
        &self.master_file_sheet_name
    }
    pub fn country_name(&self) -> &str {
        &self.country_name
    }
    pub fn ccy(&self) -> &str {
        &self.ccy
    }
    pub fn source_names(&self) -> &Vec<String> {
        &self.source_names
    }
    pub fn rf_llg(&self) -> &str {
        &self.rf_llg
    }
    pub fn b1_llg(&self) -> &str {
        &self.b1_llg
    }
    pub fn b2_llg(&self) -> &str {
        &self.b2_llg
    }
    pub fn b3_llg(&self) -> &str {
        &self.b3_llg
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("Program to Find amt classification of credit card gl Customer")
        .version("1.0.3704")
        .author("Ravindar Singh<ravindar.sr@surya-soft.com>")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to input file path.")
                .required(true)
        )
        .arg(
            Arg::new("master_file")
                .long("master-file")
                .value_name("master File Path")
                .help("Path to master file.")
                .required(true)
        )
        .arg(
            Arg::new("master_file_sheet_name")
                .long("master-file-sheet-name")
                .value_name("master File Sheet NamePath")
                .help("Path to master file sheet name.")
                .required(true)
        )
        .arg(
            Arg::new("country_name")
                .long("country-name")
                .value_name("Country Name")
                .help("country name")
                .required(true)
        )
        .arg(
            Arg::new("ccy")
                .long("ccy")
                .value_name("Currency")
                .help("currency")
                .required(true),
        )
        .arg(
            Arg::new("source_names")
                .long("source-names")
                .value_name("Source Names")
                .help("source names")
                .required(true),
        )
        .arg(
            Arg::new("rf_llg")
                .long("rf-llg")
                .value_name("RF LLG")
                .help("rf llg")
                .required(true),
        )
        .arg(
            Arg::new("b1_llg")
                .long("b1-llg")
                .value_name("B1 LLG")
                .help("b1 llg")
                .required(true),
        )
        .arg(
            Arg::new("b2_llg")
                .long("b2-llg")
                .value_name("B2 LLG")
                .help("b2 llg")
                .required(true),
        )
        .arg(
            Arg::new("b3_llg")
                .long("b3-llg")
                .value_name("B3 LLG")
                .help("b3 llg")
                .required(true),
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Final Output File Path")
                .help("Path to final output file.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
