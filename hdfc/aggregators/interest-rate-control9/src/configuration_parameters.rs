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
    input_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    summary_file_path: String,
    drilldown_file_path: String,
    club_ten_master: String,
    club_ten_sheet: String,
    cust_type_master: String,
    cust_type_staff_sheet: String,
    cust_type_senior_sheet: String,
    club_ten_rate_master: String,
    tenor_rate_staff: String,
    tenor_rate_senior: String,
    tenor_rate_others: String,
    amb_master: String,
    bucket_master: String,
    bucket_sheet: String,
    ret_cust_aggr_lcy_file: String,
    non_ret_cust_aggr_lcy_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(logger, "summary_file: {}", self.summary_file_path());
        info!(logger, "drilldown_file: {}", self.drilldown_file_path());
        info!(logger, "club_ten_master: {}", self.club_ten_master());
        info!(logger, "club_ten_sheet: {}", self.club_ten_sheet());
        info!(logger, "cust_type_master: {}", self.cust_type_master());
        info!(
            logger,
            "cust_type_staff_sheet: {}",
            self.cust_type_staff_sheet()
        );
        info!(
            logger,
            "cust_type_senior_sheet: {}",
            self.cust_type_senior_sheet()
        );
        info!(
            logger,
            "club_ten_rate_master: {}",
            self.club_ten_rate_master()
        );
        info!(logger, "tenor_rate_staff: {}", self.tenor_rate_staff());
        info!(logger, "tenor_rate_senior: {}", self.tenor_rate_senior());
        info!(logger, "tenor_rate_others: {}", self.tenor_rate_others());
        info!(logger, "amb_master: {}", self.amb_master());
        info!(logger, "bucket_master: {}", self.bucket_master());
        info!(logger, "bucket_sheet: {}", self.bucket_sheet());
        info!(
            logger,
            "ret_cust_aggr_lcy_file: {}",
            self.ret_cust_aggr_lcy_file()
        );
        info!(
            logger,
            "non_ret_cust_aggr_lcy_file: {}",
            self.non_ret_cust_aggr_lcy_file()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_path_file`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `account_metadata_file_path`.")
            .to_string();
        let summary_file_path = matches
            .value_of("summary_file")
            .expect("Error while getting `summary report file path`.")
            .to_string();
        let drilldown_file_path = matches
            .value_of("drilldown_file")
            .expect("Error while getting `drilldown report file path`.")
            .to_string();
        let club_ten_master = matches
            .value_of("club_ten_master")
            .expect("Error while getting `club_ten_master` file.")
            .to_string();
        let club_ten_sheet = matches
            .value_of("club_ten_sheet")
            .expect("Error while getting `club_ten_master sheet name`.")
            .to_string();
        let cust_type_master = matches
            .value_of("cust_type_master")
            .expect("Error while getting `cust_type_master` file.")
            .to_string();
        let cust_type_staff_sheet = matches
            .value_of("cust_type_staff_sheet")
            .expect("Error while getting `cust_type_staff_sheet` name.")
            .to_string();
        let cust_type_senior_sheet = matches
            .value_of("cust_type_senior_sheet")
            .expect("Error while getting `cust_type_senior_sheet` name.")
            .to_string();
        let club_ten_rate_master = matches
            .value_of("club_ten_rate_master")
            .expect("Error while getting `clubbed_tenor_rate_master` file.")
            .to_string();
        let tenor_rate_staff = matches
            .value_of("tenor_rate_staff")
            .expect("Error while getting `clubbed_tenor_rate_staff` sheet name.")
            .to_string();
        let tenor_rate_senior = matches
            .value_of("tenor_rate_senior")
            .expect("Error while getting `clubbed_tenor_rate_senior` sheet name.")
            .to_string();
        let tenor_rate_others = matches
            .value_of("tenor_rate_others")
            .expect("Error while getting `clubbed_tenor_rate_others` sheet name.")
            .to_string();
        let amb_master = matches
            .value_of("amb_master")
            .expect("Error while getting `amb_master` file.")
            .to_string();
        let bucket_master = matches
            .value_of("bucket_master")
            .expect("Error getting `bucket_master` file.")
            .to_string();
        let bucket_sheet = matches
            .value_of("bucket_sheet")
            .expect("Error getting `bucket_sheet` name.")
            .to_string();
        let ret_cust_aggr_lcy_file = matches
            .value_of("ret_cust_aggr_lcy_file")
            .expect("Error while getting `ret_cust_aggr_lcy_file` path.")
            .to_string();
        let non_ret_cust_aggr_lcy_file = matches
            .value_of("non_ret_cust_aggr_lcy_file")
            .expect("Error while getting `non_ret_cust_aggr_lcy_file` path.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting as on date as DD-MM-YYYY."),
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
            input_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            summary_file_path,
            drilldown_file_path,
            club_ten_master,
            club_ten_sheet,
            cust_type_master,
            cust_type_staff_sheet,
            cust_type_senior_sheet,
            club_ten_rate_master,
            tenor_rate_staff,
            tenor_rate_senior,
            tenor_rate_others,
            amb_master,
            bucket_master,
            bucket_sheet,
            ret_cust_aggr_lcy_file,
            non_ret_cust_aggr_lcy_file,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properateies (they're private).
// Also, because users of these properateies usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
    }
    pub fn summary_file_path(&self) -> &str {
        &self.summary_file_path
    }
    pub fn drilldown_file_path(&self) -> &str {
        &self.drilldown_file_path
    }
    pub fn club_ten_master(&self) -> &str {
        &self.club_ten_master
    }
    pub fn club_ten_sheet(&self) -> &str {
        &self.club_ten_sheet
    }
    pub fn cust_type_master(&self) -> &str {
        &self.cust_type_master
    }
    pub fn cust_type_staff_sheet(&self) -> &str {
        &self.cust_type_staff_sheet
    }
    pub fn cust_type_senior_sheet(&self) -> &str {
        &self.cust_type_senior_sheet
    }
    pub fn club_ten_rate_master(&self) -> &str {
        &self.club_ten_rate_master
    }
    pub fn tenor_rate_staff(&self) -> &str {
        &self.tenor_rate_staff
    }
    pub fn tenor_rate_senior(&self) -> &str {
        &self.tenor_rate_senior
    }
    pub fn tenor_rate_others(&self) -> &str {
        &self.tenor_rate_others
    }
    pub fn amb_master(&self) -> &str {
        &self.amb_master
    }
    pub fn bucket_master(&self) -> &str {
        &self.bucket_master
    }
    pub fn bucket_sheet(&self) -> &str {
        &self.bucket_sheet
    }
    pub fn ret_cust_aggr_lcy_file(&self) -> &str {
        &self.ret_cust_aggr_lcy_file
    }
    pub fn non_ret_cust_aggr_lcy_file(&self) -> &str {
        &self.non_ret_cust_aggr_lcy_file
    }
    pub fn as_on_date(&self) -> String {
        self.as_on_date.format("%Y%m%d").to_string()
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
        .about("IR-Control Report for accounts with similar TD value month and tenors.")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input file path")
                .help("Path to base input file. ")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file-path")
                .value_name("Required Fields file")
                .help("File that contains required fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file-path")
                .value_name("Account Metadata File")
                .help("Path to json file that has metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("summary_file")
                .long("summary-file")
                .value_name("Summary File")
                .help("Path to the summary report file.")
                .required(true)
        )
        .arg(
            Arg::with_name("drilldown_file")
                .long("drilldown-file")
                .value_name("Drilldown File")
                .help("Path to the drilldown report file.")
                .required(true)
        )
        .arg(
            Arg::with_name("club_ten_master")
                .long("club-ten-master")
                .value_name("clubbed tenor master")
                .help("Path to clubbed tenor master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("club_ten_sheet")
                .long("club-ten-master-sheet")
                .value_name("clubbed tenor master sheet")
                .help("Clubbed tenor master sheet name.")
                .default_value("Sheet1")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_type_master")
                .long("cust-type-master")
                .value_name("customer type master")
                .help("Path to customer type master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_type_senior_sheet")
                .long("cust-type-senior-sheet")
                .value_name("Customer type senior")
                .help("Customer type senior sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_type_staff_sheet")
                .long("cust-type-staff-sheet")
                .value_name("Customer type staff")
                .help("Customer type staff sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("club_ten_rate_master")
                .long("club-ten-rate-master")
                .value_name("clubbed tenor rate master")
                .help("Clubbed tenor rate master file name.")
                .required(true)
        )
        .arg(
            Arg::with_name("tenor_rate_senior")
                .long("tenor-rate-senior")
                .value_name("Tenor rate senior")
                .help("Tenor rate senior sheet name.")
                .required(true),
        )
        .arg(
            Arg::with_name("tenor_rate_staff")
                .long("tenor-rate-staff")
                .value_name("Tenor rate staff")
                .help("Tenor rate staff sheet name.")
                .required(true),
        )
        .arg(
            Arg::with_name("tenor_rate_others")
                .long("tenor-rate-others")
                .value_name("Tenor rate others")
                .help("Tenor rate others sheet name.")
                .required(true),
        )
        .arg(
            Arg::with_name("amb_master")
                .long("amb-master-file-path")
                .value_name("AMB master file path")
                .help("Path to AMB master file path")
                .required(true)
        )
        .arg(
            Arg::with_name("bucket_master")
                .long("bucket-master")
                .value_name("Amount Bucket Master File")
                .help("Path to Amount Bucket Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("bucket_sheet")
                .long("bucket-sheet")
                .value_name("Amount Bucket Master Sheet Name")
                .help("Name of Amount Bucket Master File sheet")
                .default_value("Sheet1")
                .required(true),
        )
        .arg(
            Arg::with_name("ret_cust_aggr_lcy_file")
                .long("ret-cust-aggr-lcy-file")
                .value_name("ret cust aggr lcy file")
                .help("Path to ret-cust-aggr-lcy file")
                .required(true)
        )
        .arg(
            Arg::with_name("non_ret_cust_aggr_lcy_file")
                .long("non-ret-cust-aggr-lcy-file")
                .value_name("non ret cust aggr lcy file")
                .help("Path to non-ret-cust-aggr-lcy file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On Date")
                .help("The date for which the program has to be processed.")
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
        .get_matches()
}
