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
    pub fei_file_path: String,
    pub tfat_file_path: String,
    pub fbh_file_path: String,
    pub idt_file_path: String,
    pub fae_file_path: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "fei_file_path: {}", self.fei_file_path());
        info!(logger, "tfat_file_path: {}", self.tfat_file_path());
        info!(logger, "fbh_file_path: {}", self.fbh_file_path());
        info!(logger, "idt_file_path: {}", self.idt_file_path());
        info!(logger, "fae_file_path: {}", self.fae_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
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
        let fei_file_path = matches
            .value_of("fei_file_path")
            .expect("Error getting `fei_file_path` value.")
            .to_string();
        let tfat_file_path = matches
            .value_of("tfat_file_path")
            .expect("Error getting `tfat_file_path` value.")
            .to_string();
        let fbh_file_path = matches
            .value_of("fbh_file_path")
            .expect("Error getting `fbh_file_path` value.")
            .to_string();
        let idt_file_path = matches
            .value_of("idt_file_path")
            .expect("Error getting `idt_file_path` value.")
            .to_string();
        let fae_file_path = matches
            .value_of("fae_file_path")
            .expect("Error getting `fae_file_path` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            fei_file_path,
            tfat_file_path,
            fbh_file_path,
            idt_file_path,
            fae_file_path,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn fei_file_path(&self) -> &str {
        &self.fei_file_path
    }
    pub fn tfat_file_path(&self) -> &str {
        &self.tfat_file_path
    }
    pub fn fbh_file_path(&self) -> &str {
        &self.fbh_file_path
    }
    pub fn idt_file_path(&self) -> &str {
        &self.idt_file_path
    }
    pub fn fae_file_path(&self) -> &str {
        &self.fae_file_path
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
    App::new(app_name)
        .about("Program to fetch interest rates for Bills accounts.")
        .version("1.0.4951")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to the Input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("fei_file_path")
                .long("fei-file-path")
                .value_name("FEI File Path")
                .help("Path to the FEI File Path")
                .required(true)
        )
        .arg(
            Arg::with_name("tfat_file_path")
                .long("tfat-file-path")
                .value_name("TFAT File Path")
                .help("Path to the TFAT File.")
                .required(true)
        )
        .arg(
            Arg::with_name("fbh_file_path")
                .long("fbh-file-path")
                .value_name("FBH File Path")
                .help("Path to the FBH File.")
                .required(true)
        )
        .arg(
            Arg::with_name("idt_file_path")
                .long("idt-file-path")
                .value_name("idt File Path")
                .help("Path to the idt File.")
                .required(true)
        )
        .arg(
            Arg::with_name("fae_file_path")
                .long("fae-file-path")
                .value_name("FAE File Path")
                .help("Path to the FAE File.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output file path.")
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
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
