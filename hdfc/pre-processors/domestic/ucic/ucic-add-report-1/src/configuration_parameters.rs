use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub input_file_path: String,
    pub ucic_biu_file: String,
    pub customer_bal_file: String,
    pub edw_master_file: String,
    pub customer_bal_file_delimiter: String,
    pub ucic_file_delimiter: String,
    pub input_file_delimiter: String,
    pub edw_file_delimiter: String,
    pub log_level: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "ucic_biu_file: {}", self.ucic_biu_file());
        info!(logger, "customer_bal_file: {}", self.customer_bal_file());
        info!(logger, "edw_master_file: {}", self.edw_master_file());
        info!(
            logger,
            "input_file_delimiter: {}",
            self.input_file_delimiter()
        );
        info!(logger, "edw_file_delimiter: {}", self.edw_file_delimiter());
        info!(
            logger,
            "customer_bal_file_delimiter: {}",
            self.customer_bal_file_delimiter()
        );
        info!(
            logger,
            "ucic_file_delimiter: {}",
            self.ucic_file_delimiter()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
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
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let customer_bal_file = matches
            .value_of("customer_bal_file")
            .expect("Error getting `customer_bal_file`.")
            .to_string();
        let ucic_biu_file = matches
            .value_of("ucic_biu_file")
            .expect("Error getting `ucic_biu_file`.")
            .to_string();
        let edw_master_file = matches
            .value_of("edw_master_file")
            .expect("Error getting `edw_master_file`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let ucic_file_delimiter = matches
            .value_of("ucic_file_delimiter")
            .expect("Error getting ucic_file_delimiter")
            .to_string();
        let input_file_delimiter = matches
            .value_of("input_file_delimiter")
            .expect("Error getting input_file_delimiter")
            .to_string();
        let customer_bal_file_delimiter = matches
            .value_of("customer_bal_file_delimiter")
            .expect("Error getting customer_bal_file_delimiter")
            .to_string();
        let edw_file_delimiter = matches
            .value_of("edw_file_delimiter")
            .expect("Error getting edw_file_delimiter")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            input_file_path,
            ucic_biu_file,
            edw_master_file,
            customer_bal_file,
            customer_bal_file_delimiter,
            edw_file_delimiter,
            ucic_file_delimiter,
            log_level,
            is_perf_diagnostics_enabled,
            input_file_delimiter,
            output_file_path,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn ucic_biu_file(&self) -> &str {
        &self.ucic_biu_file
    }
    pub fn customer_bal_file(&self) -> &str {
        &self.customer_bal_file
    }
    pub fn edw_master_file(&self) -> &str {
        &self.edw_master_file
    }
    pub fn ucic_file_delimiter(&self) -> &str {
        &self.ucic_file_delimiter
    }
    pub fn input_file_delimiter(&self) -> &str {
        &self.input_file_delimiter
    }
    pub fn customer_bal_file_delimiter(&self) -> &str {
        &self.customer_bal_file_delimiter
    }
    pub fn edw_file_delimiter(&self) -> &str {
        &self.edw_file_delimiter
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
        .about("Program for UCIC additional report 1 generation")
        .author("Sachin Mulgir <sachin.m@surya-soft.com>")
        .version("1.2.4638")
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
            Arg::new("input_file")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to read Input File.")
                .required(true)
        )
        .arg(
            Arg::new("input_file_delimiter")
                .long("input-file-delimiter")
                .value_name("input_file_delimiter")
                .help("Delimiter used in Input File")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to read Output File.")
                .required(true)
        )
        .arg(
            Arg::new("ucic_biu_file")
                .long("ucic-biu-file")
                .value_name("ucic_biu_file")
                .help("Path to read Ucic Biu File.")
                .required(true)
        )
        .arg(
            Arg::new("customer_bal_file")
                .long("customer-bal-file")
                .value_name("customer_bal_file")
                .help("Path to read Customer Balance File.")
                .required(true)
        )
        .arg(
            Arg::new("edw_master_file")
                .long("edw-master-file")
                .value_name("edw_master_file")
                .help("Path to read EDW Master` File.")
                .required(true)
        )
        .arg(
            Arg::new("ucic_file_delimiter")
                .long("ucic-file-delimiter")
                .value_name("ucic_file_delimiter")
                .help("Delimiter used in UCIC Master File")
                .required(true)
        )
        .arg(
            Arg::new("customer_bal_file_delimiter")
                .long("customer-bal-file-delimiter")
                .value_name("customer_bal_file_delimiter")
                .help("Delimiter used in Customer Balance File")
                .required(true)
        )
        .arg(
            Arg::new("edw_file_delimiter")
                .long("edw-file-delimiter")
                .value_name("edw_file_delimiter")
                .help("Delimiter used in EDW Master File")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
