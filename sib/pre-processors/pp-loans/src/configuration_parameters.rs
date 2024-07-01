use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    gam_input_file: String,
    eit_input_file: String,
    ach_input_file: String,
    overdue_input_file: String,
    npa_input_file: String,
    intrate_input_file: String,
    lam_input_file: String,
    itc_input_file: String,
    rate_code_mapping_master: String,
    loan_add_file: String,
    output_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "gam_input_file: {}", self.gam_input_file());
        info!(logger, "eit_input_file: {}", self.eit_input_file());
        info!(logger, "ach_input_file: {}", self.ach_input_file());
        info!(logger, "lam_input_file: {}", self.lam_input_file());
        info!(logger, "itc_input_file: {}", self.itc_input_file());
        info!(
            logger,
            "rate_code_mapping_master: {}",
            self.rate_code_mapping_master()
        );
        info!(logger, "loan_add_file: {}", self.loan_add_file());
        info!(logger, "overdue_input_file: {}", self.overdue_input_file());
        info!(logger, "npa_input_file: {}", self.npa_input_file());
        info!(logger, "intrate_input_file: {}", self.intrate_input_file());
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let gam_input_file = matches
            .value_of("gam_input_file")
            .expect("Error getting `gam_input_file`.")
            .to_string();
        let eit_input_file = matches
            .value_of("eit_input_file")
            .expect("Error getting `eit_input_file`.")
            .to_string();
        let ach_input_file = matches
            .value_of("ach_input_file")
            .expect("Error getting `ach_input_file`.")
            .to_string();
        let lam_input_file = matches
            .value_of("lam_input_file")
            .expect("Error getting `lam_input_file`.")
            .to_string();
        let itc_input_file = matches
            .value_of("itc_input_file")
            .expect("Error getting `itc_input_file`.")
            .to_string();
        let rate_code_mapping_master = matches
            .value_of("rate_code_mapping_master")
            .expect("Error getting `rate_code_mapping_master`.")
            .to_string();
        let overdue_input_file = matches
            .value_of("overdue_input_file")
            .expect("Error getting `overdue_input_file`.")
            .to_string();
        let npa_input_file = matches
            .value_of("npa_input_file")
            .expect("Error getting `npa_input_file`.")
            .to_string();
        let intrate_input_file = matches
            .value_of("intrate_input_file")
            .expect("Error getting `intrate_input_file`.")
            .to_string();
        let loan_add_file = matches
            .value_of("loan_add_file")
            .expect("Error getting `loan_add_file`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
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
            gam_input_file,
            ach_input_file,
            eit_input_file,
            lam_input_file,
            itc_input_file,
            rate_code_mapping_master,
            overdue_input_file,
            npa_input_file,
            intrate_input_file,
            loan_add_file,
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
    pub fn gam_input_file(&self) -> &str {
        &self.gam_input_file
    }
    pub fn ach_input_file(&self) -> &str {
        &self.ach_input_file
    }
    pub fn eit_input_file(&self) -> &str {
        &self.eit_input_file
    }
    pub fn lam_input_file(&self) -> &str {
        &self.lam_input_file
    }
    pub fn itc_input_file(&self) -> &str {
        &self.itc_input_file
    }
    pub fn rate_code_mapping_master(&self) -> &str {
        &self.rate_code_mapping_master
    }
    pub fn overdue_input_file(&self) -> &str {
        &self.overdue_input_file
    }
    pub fn npa_input_file(&self) -> &str {
        &self.npa_input_file
    }
    pub fn intrate_input_file(&self) -> &str {
        &self.intrate_input_file
    }
    pub fn loan_add_file(&self) -> &str {
        &self.loan_add_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
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

fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("PP Loans Program for SIB!!")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("gam_input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Loans Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("eit_input_file")
                .long("eit-input-file")
                .value_name("EIT_FILE")
                .help("EIT Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("lam_input_file")
                .long("lam-input-file")
                .value_name("LAM_FILE")
                .help("LAM Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("itc_input_file")
                .long("itc-input-file")
                .value_name("ITC_FILE")
                .help("ITC Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("rate_code_mapping_master")
                .long("rate-code-mapping-master")
                .value_name("RATECODE_MASTER")
                .help("Rate Code Mapping Master Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("ach_input_file")
                .long("ach-input-file")
                .value_name("ACH_FILE")
                .help("ACH Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("overdue_input_file")
                .long("overdue-input-file")
                .value_name("OVERDUE_FILE")
                .help("Loans Overdue Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_input_file")
                .long("npa-input-file")
                .value_name("NPA_FILE")
                .help("NPA Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("intrate_input_file")
                .long("intrate-input-file")
                .value_name("INTRATE_FILE")
                .help("Loans Interest Rate Input File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("loan_add_file")
                .long("loan-add-file")
                .value_name("LOAN_ADD_FILE")
                .help("Loans Add File Input File Path.")
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
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS_FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
