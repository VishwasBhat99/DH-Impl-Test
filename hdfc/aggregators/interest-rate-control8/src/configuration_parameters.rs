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
    cur_mth_src_file: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    source_system: String,
    summary_file_path: String,
    drilldown_file_path: String,
    master_file: String,
    sheet_name: String,
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
        info!(logger, "cur_mth_src_file: {}", self.cur_mth_src_file());
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
        info!(logger, "source_system: {}", self.source_system());
        info!(logger, "summary_file: {}", self.summary_file_path());
        info!(logger, "drilldown_file: {}", self.drilldown_file_path());
        info!(logger, "master_file: {}", self.master_file());
        info!(logger, "sheet_name: {}", self.sheet_name());
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
        let cur_mth_src_file = matches
            .value_of("cur_mth_src_file")
            .expect("Error getting `cur_mth_src_file`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `account_metadata_file_path`.")
            .to_string();
        let source_system = matches
            .value_of("source_system")
            .expect("Error getting `source_system`.")
            .to_string();
        let summary_file_path = matches
            .value_of("summary_file")
            .expect("Error while getting `summary report file path`.")
            .to_string();
        let drilldown_file_path = matches
            .value_of("drilldown_file")
            .expect("Error while getting `drilldown report file path`.")
            .to_string();
        let master_file = matches
            .value_of("master_file")
            .expect("Error while getting `master_file path`.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error while getting `sheet_name path`.")
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
            cur_mth_src_file,
            req_fields_file_path,
            account_metadata_file_path,
            source_system,
            summary_file_path,
            drilldown_file_path,
            master_file,
            sheet_name,
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

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn cur_mth_src_file(&self) -> &str {
        &self.cur_mth_src_file
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
    }
    pub fn source_system(&self) -> &str {
        &self.source_system
    }
    pub fn summary_file_path(&self) -> &str {
        &self.summary_file_path
    }
    pub fn drilldown_file_path(&self) -> &str {
        &self.drilldown_file_path
    }
    pub fn master_file(&self) -> &str {
        &self.master_file
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
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
        .about("IR-Control Report for accounts with similar TD tenors.")
        .arg(
            Arg::with_name("cur_mth_src_file")
                .long("cur-mth-src-file")
                .value_name("current month source file path")
                .help("Path to current month source file")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file-path")
                .value_name("Required Fields")
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
            Arg::with_name("source_system")
                .long("source-system")
                .value_name("Source System")
                .help("The type of advance.")
                .required(true)
        )
        .arg(
            Arg::with_name("summary_file")
                .long("summary-file")
                .value_name("Summary File")
                .help("Path to the summary report file")
                .required(true)
        )
        .arg(
            Arg::with_name("drilldown_file")
                .long("drilldown-file")
                .value_name("drilldown File")
                .help("Path to the drilldown report file")
                .required(true)
        )
        .arg(
            Arg::with_name("master_file")
                .long("master-file")
                .value_name("master File")
                .help("Path to master file")
                .required(true)
        )
        .arg(
            Arg::with_name("sheet_name")
                .long("sheet-name")
                .value_name("sheet name")
                .help("Path to sheet name")
                .required(true)
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
