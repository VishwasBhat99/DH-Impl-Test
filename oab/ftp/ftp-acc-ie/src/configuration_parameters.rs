use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub ftp_accie_file_path: String,
    pub as_on_date: rbdate::NaiveDate,
    pub exrt_file_path: String,
    pub output_file_path: String,
    pub interest_income: Vec<String>,
    pub interest_expense: Vec<String>,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub base_ccy: String,
    pub req_fields_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(
            logger,
            "ftp_accie_file_path: {}",
            self.ftp_accie_file_path()
        );
        info!(logger, "interest_income: {:?}", self.interest_income());
        info!(logger, "interest_expense: {:?}", self.interest_expense());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "exrt_file_path: {}", self.exrt_file_path());
        info!(logger, "base_ccy: {}", self.base_ccy());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path` value.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `Req fields file path`.")
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
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();
        let ftp_accie_file_path = matches
            .value_of("ftp_accie_file_path")
            .expect("Error getting `ftp_accie_file_path` value.")
            .to_string();
        let interest_income: Vec<String> = matches
            .value_of("interest_income")
            .expect("Error getting `interest_income`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let interest_expense: Vec<String> = matches
            .value_of("interest_expense")
            .expect("Error getting `interest_expense`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let exrt_file_path = matches
            .value_of("exrt_file_path")
            .expect("Error getting `exrt_file_path` value.")
            .to_string();
        let base_ccy = matches
            .value_of("base_ccy")
            .expect("Error getting `base_ccy` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            metadata_file_path,
            ftp_accie_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            exrt_file_path,
            is_perf_diagnostics_enabled,
            interest_expense,
            interest_income,
            base_ccy,
            req_fields_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn ftp_accie_file_path(&self) -> &str {
        &self.ftp_accie_file_path
    }
    pub fn as_on_date(&self) -> &rbdate::NaiveDate {
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
    pub fn interest_income(&self) -> &Vec<String> {
        &self.interest_income
    }
    pub fn interest_expense(&self) -> &Vec<String> {
        &self.interest_expense
    }
    pub fn exrt_file_path(&self) -> &str {
        &self.exrt_file_path
    }
    pub fn base_ccy(&self) -> &str {
        &self.base_ccy
    }
}

fn get_eligible_arguments_for_command(command_name: &str) -> clap::ArgMatches {
    Command::new(command_name)
        .about("Program to get stamper output with Interest-{Income, Expense}")
        .version("1.0.3304")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("input_file_path")
                .long("input-file")
                .value_name("input_file")
                .help("Path to Master Input File.")
                .required(true)
        )
        .arg(
            Arg::new("metadata_file_path")
                .long("metadata-file")
                .value_name("metadata_file_path")
                .help("Path to Metadata File.")
                .required(true)
        )
        .arg(
            Arg::new("ftp_accie_file_path")
                .long("ftp-accie-file")
                .value_name("ftp_accie_file_path")
                .help("Path to Input FTP Account Interest Income-Expense File.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("REQ_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write log to")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("output_file")
                .help("Path to Output File.")
                .required(true)
        )
        .arg(
            Arg::new("base_ccy")
                .long("base-ccy")
                .value_name("Currency")
                .help("Home Currency/ Base Currency")
                .default_value("OMR")
                .required(false)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::new("interest_income")
                .long("interest-income")
                .value_name("cashflow fields column")
                .help("Sources whose interest amount to be stamped as interest-income")
                .required(true)
        )
        .arg(
            Arg::new("interest_expense")
                .long("interest-expense")
                .value_name("cashflow fields column")
                .help("Sources whose interest amount to be stamped as interest-expense.")
                .required(true)
        )
        .arg(
            Arg::new("exrt_file_path")
                .long("exrt-file")
                .value_name("exrt_file_path")
                .help("Path to Exchange Rate File.")
                .required(true)
        )
        .get_matches()
}
