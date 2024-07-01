use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    loan_acc_detail_home: String,
    loan_acc_detail_auto: String,
    loan_acc_detail_personal_f1: String,
    loan_acc_detail_personal_f2: String,
    loan_repay_structure: String,
    mclr_data_file: String,
    npa_data_file: String,
    plr_loan_acc_file: String,
    quarter_end_dates_file: String,
    output_file_path: String,
    all_acc_required: bool,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "loan_acc_detail_home: {}",
            self.loan_acc_detail_home()
        );
        info!(
            logger,
            "loan_acc_detail_auto: {}",
            self.loan_acc_detail_auto()
        );
        info!(
            logger,
            "loan_acc_detail_personal_f1: {}",
            self.loan_acc_detail_personal_f1()
        );
        info!(logger, "mclr_data_file: {}", self.mclr_data_file());
        info!(logger, "npa_data_file: {}", self.npa_data_file());
        info!(logger, "plr_loan_acc_file: {}", self.plr_loan_acc_file());
        info!(
            logger,
            "loan_acc_detail_personal_f2: {}",
            self.loan_acc_detail_personal_f2()
        );
        info!(
            logger,
            "loan_repay_structure: {}",
            self.loan_repay_structure()
        );
        info!(
            logger,
            "quarter_end_dates_file: {}",
            self.quarter_end_dates_file()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "all_acc_required: {}", self.all_acc_required());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let loan_acc_detail_home = matches
            .value_of("loan_acc_detail_home")
            .expect("Error getting `loan_acc_detail_home`.")
            .to_string();
        let loan_acc_detail_auto = matches
            .value_of("loan_acc_detail_auto")
            .expect("Error getting `loan_acc_detail_auto`.")
            .to_string();
        let loan_acc_detail_personal_f1 = matches
            .value_of("loan_acc_detail_personal_f1")
            .expect("Error getting `loan_acc_detail_personal_f1`.")
            .to_string();
        let mclr_data_file = matches
            .value_of("mclr_data_file")
            .expect("Error getting `mclr_data_file`.")
            .to_string();
        let npa_data_file = matches
            .value_of("npa_data_file")
            .expect("Error getting `npa_data_file`.")
            .to_string();
        let plr_loan_acc_file = matches
            .value_of("plr_loan_acc_file")
            .expect("Error getting `plr_loan_acc_file`.")
            .to_string();
        let loan_acc_detail_personal_f2 = matches
            .value_of("loan_acc_detail_personal_f2")
            .expect("Error getting `loan_acc_detail_personal_f2`.")
            .to_string();
        let loan_repay_structure = matches
            .value_of("loan_repay_structure")
            .expect("Error getting `loan_repay_structure`.")
            .to_string();
        let quarter_end_dates_file = matches
            .value_of("quarter_end_dates_file")
            .expect("Error getting `quarter_end_dates_file`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let all_acc_required = matches
            .value_of("all_acc_required")
            .expect("Error getting `all_acc_required`.")
            .parse::<bool>()
            .expect("Cannot parse `all_acc_required` as bool.");
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
            loan_acc_detail_home,
            loan_acc_detail_auto,
            loan_acc_detail_personal_f1,
            mclr_data_file,
            npa_data_file,
            plr_loan_acc_file,
            loan_acc_detail_personal_f2,
            loan_repay_structure,
            output_file_path,
            all_acc_required,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            quarter_end_dates_file,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn loan_acc_detail_home(&self) -> &str {
        &self.loan_acc_detail_home
    }
    pub fn loan_acc_detail_auto(&self) -> &str {
        &self.loan_acc_detail_auto
    }
    pub fn loan_acc_detail_personal_f1(&self) -> &str {
        &self.loan_acc_detail_personal_f1
    }
    pub fn mclr_data_file(&self) -> &str {
        &self.mclr_data_file
    }
    pub fn npa_data_file(&self) -> &str {
        &self.npa_data_file
    }
    pub fn plr_loan_acc_file(&self) -> &str {
        &self.plr_loan_acc_file
    }
    pub fn loan_acc_detail_personal_f2(&self) -> &str {
        &self.loan_acc_detail_personal_f2
    }
    pub fn loan_repay_structure(&self) -> &str {
        &self.loan_repay_structure
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn all_acc_required(&self) -> bool {
        self.all_acc_required
    }
    pub fn quarter_end_dates_file(&self) -> &str {
        &self.quarter_end_dates_file
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program For Pre Processing Finnone Loan Data.")
        .version("1.0.4862")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("loan_acc_detail_home")
                .long("loan-acc-detail-home")
                .value_name("Loan_acc_detail_home")
                .help("loan_acc_detail_home input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("loan_acc_detail_auto")
                .long("loan-acc-detail-auto")
                .value_name("Loan_acc_detail_auto")
                .help("loan_acc_detail_auto input file")
                .required(true)
        )
        .arg(
            Arg::with_name("loan_acc_detail_personal_f1")
                .long("loan-acc-detail-personal-f1")
                .value_name("Loan_acc_detail_personal_f1")
                .help("loan_acc_detail_personal_f1 input file")
                .required(true)
        )
        .arg(
            Arg::with_name("mclr_data_file")
                .long("mclr-data-file")
                .value_name("mclr_data_file")
                .help("MCLR data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_data_file")
                .long("npa-data-file")
                .value_name("npa_data_file")
                .help("NPA data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("plr_loan_acc_file")
                .long("plr-loan-acc-file")
                .value_name("plr_loan_acc_file")
                .help("PLR Loan  acc file.")
                .required(true)
        )
        .arg(
            Arg::with_name("loan_acc_detail_personal_f2")
                .long("loan-acc-detail-personal-f2")
                .value_name("Loan_acc_detail_personal_f2")
                .help("loan_acc_detail_personal_f2 input file")
                .required(true)
        )
        .arg(
            Arg::with_name("loan_repay_structure")
                .long("loan-repay-structure")
                .value_name("loan_repay_structure")
                .help("loan_repay_structure.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("output File Path")
                .help("Path to output file.")
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
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
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
            Arg::with_name("all_acc_required")
                .long("all-acc-required")
                .value_name("ALL ACCOUNTS REQUIRED")
                .possible_values(&["true", "false"])
                .help("This flag helps to decide whether accounts with 'NA' CIF NO should be printed or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("quarter_end_dates_file")
                .long("quarter-end-dates-file")
                .value_name("quarter_end_dates_file")
                .help(" Path to Quarter End Dates File.")
                .required(true)
        )
        .get_matches()
}
