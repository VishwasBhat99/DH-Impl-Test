use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub re_struct_file_path: String,
    pub resid_mort_file_path: String,
    pub rw_file_path: String,
    pub inp_del: String,
    pub re_struct_field_pos: usize,
    pub resid_field_pos: usize,
    pub rw_field_pos: usize,
    pub acc_no_pos: usize,
    pub src_codes: Vec<String>,
    pub re_struct_desc: String,
    pub resid_desc: String,
    pub rw_desc: f64,
    pub comparator: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub check_and_write_for_all_cases: bool,
    pub remove_last_char: usize,
    pub exp_def_flag_pos: usize,
    pub exp_def_flag_desc: String,
    pub cap_mkt_exp_pos: usize,
    pub cap_mkt_exp_desc: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(
            logger,
            "re-structured_file_path: {}",
            self.re_struct_file_path()
        );
        info!(
            logger,
            "residual_mortgage_file_path: {}",
            self.resid_mort_file_path()
        );
        info!(logger, "rw_file_path: {}", self.rw_file_path());
        info!(logger, "input_delimeter: {}", self.inp_del());
        info!(
            logger,
            "re-structured_field_pos: {}",
            self.re_struct_field_pos()
        );
        info!(
            logger,
            "residual_mortage_field_pos: {}",
            self.resid_field_pos()
        );
        info!(logger, "account_number_pos: {}", self.acc_no_pos());
        info!(logger, "src_codes: {:?}", self.src_codes());
        info!(
            logger,
            "re-structured_description: {}",
            self.re_struct_desc()
        );
        info!(
            logger,
            "residual_mortgage_description: {}",
            self.resid_desc()
        );
        info!(logger, "rw_description: {}", self.rw_desc());
        info!(logger, "comparator: {}", self.comparator());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "check_and_write_for_all_cases: {}",
            self.check_and_write_for_all_cases()
        );
        info!(logger, "remove_last_character: {}", self.remove_last_char());
        info!(logger, "exp_def_flag_pos: {}", self.exp_def_flag_pos());
        info!(logger, "exp_def_flag_desc: {}", self.exp_def_flag_desc());
        info!(logger, "cap_mkt_exp_pos: {}", self.cap_mkt_exp_pos());
        info!(logger, "cap_mkt_exp_desc: {}", self.cap_mkt_exp_desc());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let rw_field_pos = matches
            .value_of("rw_field_pos")
            .expect("Error getting `rw_field_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `rw_field_pos` as usize.");
        let acc_no_pos = matches
            .value_of("acc_no_pos")
            .expect("Error getting `acc_no_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `acc_no_pos` as usize.");
        let src_codes = matches
            .value_of("src_codes")
            .expect("Error getting `src_codes` value.")
            .to_string()
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let re_struct_desc = matches
            .value_of("re_struct_desc")
            .expect("Error getting `re_struct_desc` value.")
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
        let re_struct_file_path = matches
            .value_of("re_struct_file_path")
            .expect("Error getting `re_struct_file_path` value.")
            .to_string();
        let resid_mort_file_path = matches
            .value_of("resid_mort_file_path")
            .expect("Error getting `resid_mort_file_path` value.")
            .to_string();
        let rw_file_path = matches
            .value_of("rw_file_path")
            .expect("Error getting `rw_file_path` value.")
            .to_string();
        let inp_del = matches
            .value_of("inp_del")
            .expect("Error getting `inp_del` value.")
            .to_string();
        let re_struct_field_pos = matches
            .value_of("re_struct_field_pos")
            .expect("Error getting `re_struct_field_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `re_struct_field_pos` as usize.");
        let resid_field_pos = matches
            .value_of("resid_field_pos")
            .expect("Error getting `resid_field_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `resid_field_pos` as usize.");
        let rw_desc = matches
            .value_of("rw_desc")
            .expect("Error getting `rw_desc` value.")
            .parse::<f64>()
            .expect("Cannot parse `rw_desc` value as float.");
        let comparator = matches
            .value_of("comparator")
            .expect("Error getting `comparator` value.")
            .to_string();
        let resid_desc = matches
            .value_of("resid_desc")
            .expect("Error getting `resid_desc` value.")
            .to_string();
        let check_and_write_for_all_cases = matches
            .value_of("check_and_write_for_all_cases")
            .expect("Error getting `check_and_write_for_all_cases` value.")
            .parse::<bool>()
            .expect("Cannot parse `check_and_write_for_all_cases` value as bool.");
        let remove_last_char = matches
            .value_of("remove_last_char")
            .expect("Error getting `remove_last_char` value")
            .parse::<usize>()
            .expect("Cannot parse `remove_last_char` as usize");
        let exp_def_flag_pos = matches
            .value_of("exp_def_flag_pos")
            .expect("Error getting `exp_def_flag_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `exp_def_flag_pos` as usize.");
        let exp_def_flag_desc = matches
            .value_of("exp_def_flag_desc")
            .expect("Error getting `exp_def_flag_desc` value.")
            .to_string();
        let cap_mkt_exp_pos = matches
            .value_of("cap_mkt_exp_pos")
            .expect("Error getting `cap_mkt_exp_pos` value.")
            .parse::<usize>()
            .expect("Cannot parse `cap_mkt_exp_pos` as usize.");
        let cap_mkt_exp_desc = matches
            .value_of("cap_mkt_exp_desc")
            .expect("Error getting `cap_mkt_exp_desc` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            re_struct_file_path,
            resid_mort_file_path,
            rw_file_path,
            inp_del,
            re_struct_field_pos,
            resid_field_pos,
            rw_field_pos,
            acc_no_pos,
            src_codes,
            re_struct_desc,
            resid_desc,
            rw_desc,
            comparator,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            check_and_write_for_all_cases,
            remove_last_char,
            exp_def_flag_pos,
            exp_def_flag_desc,
            cap_mkt_exp_pos,
            cap_mkt_exp_desc,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn re_struct_file_path(&self) -> &str {
        &self.re_struct_file_path
    }
    pub fn resid_mort_file_path(&self) -> &str {
        &self.resid_mort_file_path
    }
    pub fn rw_file_path(&self) -> &str {
        &self.rw_file_path
    }
    pub fn inp_del(&self) -> &str {
        &self.inp_del
    }
    pub fn re_struct_field_pos(&self) -> usize {
        self.re_struct_field_pos
    }
    pub fn resid_field_pos(&self) -> usize {
        self.resid_field_pos
    }
    pub fn rw_field_pos(&self) -> usize {
        self.rw_field_pos
    }
    pub fn acc_no_pos(&self) -> usize {
        self.acc_no_pos
    }
    pub fn src_codes(&self) -> &Vec<String> {
        &self.src_codes
    }
    pub fn re_struct_desc(&self) -> &str {
        &self.re_struct_desc
    }
    pub fn resid_desc(&self) -> &str {
        &self.resid_desc
    }
    pub fn rw_desc(&self) -> f64 {
        self.rw_desc
    }
    pub fn comparator(&self) -> &str {
        &self.comparator
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
    pub fn check_and_write_for_all_cases(&self) -> bool {
        self.check_and_write_for_all_cases
    }
    pub fn remove_last_char(&self) -> usize {
        self.remove_last_char
    }
    pub fn exp_def_flag_pos(&self) -> usize {
        self.exp_def_flag_pos
    }
    pub fn exp_def_flag_desc(&self) -> &str {
        &self.exp_def_flag_desc
    }
    pub fn cap_mkt_exp_pos(&self) -> usize {
        self.cap_mkt_exp_pos
    }
    pub fn cap_mkt_exp_desc(&self) -> &str {
        &self.cap_mkt_exp_desc
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Pre-processor for Basel File Splitter.")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .version("1.0.4617")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("re_struct_file_path")
                .long("re-struct-file")
                .value_name("re_struct_file_path")
                .help("Path to the re-structured file.")
                .required(true)
        )
        .arg(
            Arg::with_name("resid_mort_file_path")
                .long("resid-mort-file")
                .value_name("resid_mort_file_path")
                .help("Path to the residential mortgage file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rw_file_path")
                .long("rw-file")
                .value_name("rw_file_path")
                .help("Path to the risk weight file.")
                .required(true)
        )
        .arg(
            Arg::with_name("inp_del")
                .long("inp-del")
                .value_name("inp_del")
                .help("Input Delimeter.")
                .required(true)
        )
        .arg(
            Arg::with_name("re_struct_field_pos")
                .long("re-struct-field-pos")
                .value_name("re_struct_field_pos")
                .help("Re-structured Field Position.")
                .required(true)
        )
        .arg(
            Arg::with_name("resid_field_pos")
                .long("resid-field-pos")
                .value_name("resid_field_pos")
                .help("Residual Mortgage Field Position.")
                .required(true)
        )
        .arg(
            Arg::with_name("rw_field_pos")
                .long("rw-field-pos")
                .value_name("rw_field_pos")
                .help("Risk Weight Field Position..")
                .required(true)
        )
        .arg(
            Arg::with_name("acc_no_pos")
                .long("acc-no-pos")
                .value_name("acc_no_pos")
                .help("Account Number Position.")
                .required(true)
        )
        .arg(
            Arg::with_name("exp_def_flag_pos")
                .long("exp-def-flag-pos")
                .value_name("exp_def_flag_pos")
                .help("EXP flag position.")
                .required(true)
        )
        .arg(
            Arg::with_name("cap_mkt_exp_pos")
                .long("cap-mkt-exp-pos")
                .value_name("cap_mkt_exp_pos")
                .help("CAP MKT position.")
                .required(true)
        )
        .arg(
            Arg::with_name("exp_def_flag_desc")
                .long("exp-def-flag-desc")
                .value_name("exp_def_flag_desc")
                .help("EXP flag Description.")
                .required(true)
        )
        .arg(
            Arg::with_name("cap_mkt_exp_desc")
                .long("cap-mkt-exp-desc")
                .value_name("cap_mkt_exp_desc")
                .help("CAP MKT description.")
                .required(true)
        )
        .arg(
            Arg::with_name("src_codes")
                .long("src-codes")
                .value_name("src_codes")
                .help("Source File Codes.")
                .required(true)
        )
        .arg(
            Arg::with_name("re_struct_desc")
                .long("re-struct-desc")
                .value_name("Re-structured Field Description")
                .help("Re-structured Field Description.")
                .required(true)
        )
        .arg(
            Arg::with_name("resid_desc")
                .long("resid-desc")
                .value_name("Residual Mortgage Field Description")
                .help("Residual Mortgage Field Description.")
                .required(true)
        )
        .arg(
            Arg::with_name("rw_desc")
                .long("rw-desc")
                .value_name("Risk Weight Field Description")
                .help("Re-structured Field Description.")
                .required(true)
        )
        .arg(
            Arg::with_name("comparator")
                .long("comparator")
                .value_name("Comparator")
                .possible_values(&["==", ">", "<", "<=", ">=", "!="])
                .help("Comparator.")
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
                .value_name("Diagnostic Log File")
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
            Arg::with_name("check_and_write_for_all_cases")
                .long("check-and-write-for-all-cases")
                .value_name("WRITE TO ALL FILES FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether to check all the cases and write to all the outputs accordingly.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("remove_last_char")
                .long("remove-last-char")
                .value_name("remove_last_char")
                .help("Remove last characters from Acc_number")
                .required(false).default_value("0")
        )
        .get_matches()
}
