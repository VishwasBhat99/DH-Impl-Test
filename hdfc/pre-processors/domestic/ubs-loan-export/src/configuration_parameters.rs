use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    npa_file_path: String,
    product_code_skip: String,
    cashflow_component_skip: String,
    ubs_loans_master_file_path: String,
    ubs_loans_cashflow_file_path: String,
    output_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "npa_file_path: {}", self.npa_file_path());
        info!(
            logger,
            "ubs_loans_master_file_path: {}",
            self.ubs_loans_master_file_path()
        );
        info!(
            logger,
            "ubs_loans_cashflow_file_path: {}",
            self.ubs_loans_cashflow_file_path()
        );
        info!(logger, "product_code_skip: {}", self.product_code_skip());
        info!(
            logger,
            "cashflow_component_skip: {}",
            self.cashflow_component_skip()
        );
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let npa_file_path = matches
            .value_of("npa_file_path")
            .expect("Error getting `npa_file_path`.")
            .to_string();
        let ubs_loans_master_file_path = matches
            .value_of("ubs_loans_master_file_path")
            .expect("Error getting `ubs_loans_master_file_path`.")
            .to_string();
        let ubs_loans_cashflow_file_path = matches
            .value_of("ubs_loans_cashflow_file_path")
            .expect("Error getting `ubs_loans_cashflow_file_path`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let product_code_skip = matches
            .value_of("product_code_skip")
            .expect("Error getting product code to skip.")
            .to_string();
        let cashflow_component_skip = matches
            .value_of("cashflow_component_skip")
            .expect("Error getting Cashflow component to skip.")
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
            npa_file_path,
            ubs_loans_cashflow_file_path,
            ubs_loans_master_file_path,
            cashflow_component_skip,
            product_code_skip,
            output_file,
            as_on_date,
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
    pub fn npa_file_path(&self) -> &str {
        &self.npa_file_path
    }
    pub fn ubs_loans_cashflow_file_path(&self) -> &str {
        &self.ubs_loans_cashflow_file_path
    }
    pub fn ubs_loans_master_file_path(&self) -> &str {
        &self.ubs_loans_master_file_path
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn product_code_skip(&self) -> &str {
        &self.product_code_skip
    }
    pub fn cashflow_component_skip(&self) -> &str {
        &self.cashflow_component_skip
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("UBS Loans Loader Program")
        .version("1.4.4734")
        .author("Sonali<sonali.s@surya-soft.com>")
        .arg(
            Arg::with_name("npa_file_path")
                .long("npa-file-path")
                .value_name("NPA FILE")
                .help("Path to get NPA file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ubs_loans_master_file_path")
                .long("ubsloans-master-file")
                .value_name("UBS LOANS MASTER FILE")
                .help("Path to get UBS Loans Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ubs_loans_cashflow_file_path")
                .long("ubsloans-cashflow-file")
                .value_name("UBS LOANS CASHFLOW FILE")
                .help("Path to get UBS Loan Cashflow File.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("OUTPUT")
                .help("Path to Output File.")
                .required(true)
        )
        .arg(
            Arg::with_name("product_code_skip")
                .long("product-code-skip")
                .value_name("PRODUCT CODE")
                .help("Path to get the value of product code that has to be skiped.")
                .required(true)
        )
        .arg(
            Arg::with_name("cashflow_component_skip")
                .long("cashflow-comp-skip")
                .value_name("CASHFLOW COMP")
                .help("Path to get the value of cashflow coomponent that has to be skiped.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("LOG_FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAGLOG_FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG_LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS_FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
