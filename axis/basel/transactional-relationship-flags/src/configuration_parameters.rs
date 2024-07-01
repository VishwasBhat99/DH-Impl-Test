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
    cmg_file: String,
    stable_reln_file: String,
    ca_file: String,
    ca_metadata_file: String,
    sa_file: String,
    sa_metadata_file: String,
    td_file: String,
    td_metadata_file: String,
    rd_file: String,
    default_llg_code: i32,
    rd_metadata_file: String,
    tbl_dep_comp_def_file: String,
    req_fields_file_path: String,
    salary_pension_reln_file: String,
    output_file_path: String,
    ca_balm_rule_file_path: String,
    sa_balm_rule_file_path: String,
    td_balm_rule_file_path: String,
    rd_balm_rule_file_path: String,
    is_nwd_code_in_use: String,
    nwd_code_lookup: String,
    nwd_constitution_codes: String,
    nwd_residual_days_limit: i64,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "cmg_file: {}", self.cmg_file());
        info!(logger, "stable_reln_file: {}", self.stable_reln_file());
        info!(logger, "ca_file: {}", self.ca_file());
        info!(logger, "sa_file: {}", self.sa_file());
        info!(logger, "td_file: {}", self.td_file());
        info!(logger, "is_nwd_code_in_use: {}", self.is_nwd_code_in_use());
        info!(logger, "nwd_code_lookup: {}", self.nwd_code_lookup());
        info!(
            logger,
            "nwd_constitution_codes: {}",
            self.nwd_constitution_codes()
        );
        info!(
            logger,
            "nwd_residual_days_limit: {}",
            self.nwd_residual_days_limit()
        );
        info!(
            logger,
            "tbl_dep_comp_def_file: {}",
            self.tbl_dep_comp_def_file()
        );
        info!(logger, "rd_file: {}", self.rd_file());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(logger, "ca_metadata_file: {}", self.ca_metadata_file());
        info!(logger, "sa_metadata_file: {}", self.sa_metadata_file());
        info!(logger, "td_metadata_file: {}", self.td_metadata_file());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "rd_metadata_file: {}", self.rd_metadata_file());
        info!(
            logger,
            "salary_pension_reln_file: {}",
            self.salary_pension_reln_file()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(
            logger,
            "ca_balm_rule_file_path: {}",
            self.ca_balm_rule_file_path()
        );
        info!(
            logger,
            "sa_balm_rule_file_path: {}",
            self.sa_balm_rule_file_path()
        );
        info!(
            logger,
            "td_balm_rule_file_path: {}",
            self.td_balm_rule_file_path()
        );
        info!(
            logger,
            "rd_balm_rule_file_path: {}",
            self.rd_balm_rule_file_path()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let cmg_file = matches
            .value_of("cmg_file")
            .expect("Error getting `cmg_file`.")
            .to_string();
        let is_nwd_code_in_use = matches
            .value_of("is_nwd_code_in_use")
            .expect("Error getting `is_nwd_code_in_use`.")
            .to_string();
        let nwd_code_lookup = matches
            .value_of("nwd_code_lookup")
            .expect("Error getting `nwd_code_lookup`.")
            .to_string();
        let nwd_constitution_codes = matches
            .value_of("nwd_constitution_codes")
            .expect("Error getting `nwd_constitution_codes`.")
            .to_string();
        let nwd_residual_days_limit = matches
            .value_of("nwd_residual_days_limit")
            .expect("Error getting `nwd_residual_days_limit`.")
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);
        let default_llg_code = matches
            .value_of("default_llg_code")
            .expect("Error getting `default_llg_code`.")
            .to_string()
            .parse::<i32>()
            .expect("Cannot parse `default_llg_code` as i32.");
        let stable_reln_file = matches
            .value_of("stable_reln_file")
            .expect("Error getting `stable_reln_file`.")
            .to_string();
        let ca_file = matches
            .value_of("ca_file")
            .expect("Error getting `ca_file`.")
            .to_string();
        let tbl_dep_comp_def_file = matches
            .value_of("tbl_dep_comp_def_file")
            .expect("Error getting `tbl_dep_comp_def_file`.")
            .to_string();
        let sa_file = matches
            .value_of("sa_file")
            .expect("Error getting `sa_file`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let td_file = matches
            .value_of("td_file")
            .expect("Error getting default_llg_code`td_file`.")
            .to_string();
        let rd_file = matches
            .value_of("rd_file")
            .expect("Error getting `rd_file`.")
            .to_string();
        let ca_metadata_file = matches
            .value_of("ca_metadata_file")
            .expect("Error getting `ca_metadata_file`.")
            .to_string();
        let sa_metadata_file = matches
            .value_of("sa_metadata_file")
            .expect("Error getting `sa_metadata_file`.")
            .to_string();
        let td_metadata_file = matches
            .value_of("td_metadata_file")
            .expect("Error getting `td_metadata_file`.")
            .to_string();
        let rd_metadata_file = matches
            .value_of("rd_metadata_file")
            .expect("Error getting `rd_metadata_file`.")
            .to_string();
        let salary_pension_reln_file = matches
            .value_of("salary_pension_reln_file")
            .expect("Error getting `salary_pension_reln_file`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let ca_balm_rule_file_path = matches
            .value_of("ca_balm_rule_file_path")
            .expect("Error getting `ca_balm_rule_file_path`.")
            .to_string();
        let sa_balm_rule_file_path = matches
            .value_of("sa_balm_rule_file_path")
            .expect("Error getting `sa_balm_rule_file_path`.")
            .to_string();
        let td_balm_rule_file_path = matches
            .value_of("td_balm_rule_file_path")
            .expect("Error getting `td_balm_rule_file_path`.")
            .to_string();
        let rd_balm_rule_file_path = matches
            .value_of("rd_balm_rule_file_path")
            .expect("Error getting `rd_balm_rule_file_path`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();

        ConfigurationParameters {
            cmg_file,
            stable_reln_file,
            ca_file,
            sa_file,
            td_file,
            rd_file,
            is_nwd_code_in_use,
            nwd_code_lookup,
            nwd_constitution_codes,
            nwd_residual_days_limit,
            default_llg_code,
            ca_metadata_file,
            sa_metadata_file,
            td_metadata_file,
            req_fields_file_path,
            rd_metadata_file,
            tbl_dep_comp_def_file,
            ca_balm_rule_file_path,
            sa_balm_rule_file_path,
            td_balm_rule_file_path,
            rd_balm_rule_file_path,
            salary_pension_reln_file,
            output_file_path,
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
    pub fn cmg_file(&self) -> &str {
        &self.cmg_file
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn stable_reln_file(&self) -> &str {
        &self.stable_reln_file
    }
    pub fn tbl_dep_comp_def_file(&self) -> &str {
        &self.tbl_dep_comp_def_file
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn is_nwd_code_in_use(&self) -> &str {
        &self.is_nwd_code_in_use
    }
    pub fn nwd_code_lookup(&self) -> &str {
        &self.nwd_code_lookup
    }
    pub fn default_llg_code(&self) -> &i32 {
        &self.default_llg_code
    }
    pub fn nwd_constitution_codes(&self) -> &str {
        &self.nwd_constitution_codes
    }
    pub fn nwd_residual_days_limit(&self) -> &i64 {
        &self.nwd_residual_days_limit
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn ca_balm_rule_file_path(&self) -> &str {
        &self.ca_balm_rule_file_path
    }
    pub fn sa_balm_rule_file_path(&self) -> &str {
        &self.sa_balm_rule_file_path
    }
    pub fn td_balm_rule_file_path(&self) -> &str {
        &self.td_balm_rule_file_path
    }
    pub fn rd_balm_rule_file_path(&self) -> &str {
        &self.rd_balm_rule_file_path
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
    pub fn ca_file(&self) -> &str {
        &self.ca_file
    }
    pub fn sa_file(&self) -> &str {
        &self.sa_file
    }
    pub fn td_file(&self) -> &str {
        &self.td_file
    }
    pub fn rd_file(&self) -> &str {
        &self.rd_file
    }
    pub fn ca_metadata_file(&self) -> &str {
        &self.ca_metadata_file
    }
    pub fn sa_metadata_file(&self) -> &str {
        &self.sa_metadata_file
    }
    pub fn td_metadata_file(&self) -> &str {
        &self.td_metadata_file
    }
    pub fn rd_metadata_file(&self) -> &str {
        &self.rd_metadata_file
    }
    pub fn salary_pension_reln_file(&self) -> &str {
        &self.salary_pension_reln_file
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .version("1.0.4002")
        .about("This program generates Transactional Relationship Flags for Customer IDs")
        .arg(
            Arg::with_name("cmg_file")
                .long("cmg-file")
                .value_name("CMG Input file")
                .help("Path to CMG Input File")
                .required(true),
        )
        .arg(
            Arg::with_name("is_nwd_code_in_use")
                .long("is-nwd-code-in-use")
                .value_name("Is NWD code in use.")
                .help("Is NWD code in use Flag")
                .required(true)
                .possible_values(&["true", "false"]),
        )
        .arg(
            Arg::with_name("nwd_code_lookup")
                .long("nwd-code-lookup")
                .value_name("nwd_code_lookup.")
                .help("NWD Lookup Codes")
                .required(false),
        )
        .arg(
            Arg::with_name("default_llg_code")
                .long("default-llg-code")
                .value_name("default_llg_code.")
                .help("Default-LLg-Code")
                .required(true),
        )
        .arg(
            Arg::with_name("nwd_constitution_codes")
                .long("nwd-constitution-codes")
                .value_name("NWD Constitution Codes.")
                .help("NWD Constitution Codes")
                .default_value("NA")
                .required(false),
        )
        .arg(
            Arg::with_name("stable_reln_file")
                .long("stable-reln-file")
                .value_name("Stable Relation File Path.")
                .help("Path to Stable Relation File")
                .required(true),
        )
        .arg(
            Arg::with_name("nwd_residual_days_limit")
                .long("nwd-residual-days-limit")
                .value_name("NWD Residual Days Limit.")
                .help("NWD Residual Days Limit.")
                .required(true),
        )
        .arg(
            Arg::with_name("ca_file")
                .long("ca-file")
                .value_name("CA Input file")
                .help("Path to CA Input File")
                .required(true),
        )
        .arg(
            Arg::with_name("tbl_dep_comp_def_file")
                .long("tbl-dep-comp-def-file")
                .value_name("TBL Deposits Computation Def file.")
                .help("Path to TBL Deposits Computation Def File")
                .required(true),
        )
        .arg(
            Arg::with_name("sa_file")
                .long("sa-file")
                .value_name("SA Input file")
                .help("Path to SA Input File")
                .required(true),
        )
        .arg(
            Arg::with_name("td_file")
                .long("td-file")
                .value_name("TD Input file")
                .help("Path to TD Input File")
                .required(true),
        )
        .arg(
            Arg::with_name("rd_file")
                .long("rd-file")
                .value_name("rd Input file")
                .help("Path to rd Input File")
                .required(true),
        )
        .arg(
            Arg::with_name("ca_metadata_file")
                .long("ca-metadata-file")
                .value_name("CA metadata file")
                .help("Path to CA metadata File")
                .required(true),
        )
        .arg(
            Arg::with_name("sa_metadata_file")
                .long("sa-metadata-file")
                .value_name("SA metadata file")
                .help("Path to SA metadata File")
                .required(true),
        )
        .arg(
            Arg::with_name("td_metadata_file")
                .long("td-metadata-file")
                .value_name("TD metadata file")
                .help("Path to TD metadata File")
                .required(true),
        )
        .arg(
            Arg::with_name("rd_metadata_file")
                .long("rd-metadata-file")
                .value_name("rd metadata file")
                .help("Path to rd metadata File")
                .required(true),
        )
        .arg(
            Arg::with_name("salary_pension_reln_file")
                .long("salary-pension-reln-file")
                .value_name("Salary Pension Relationship file")
                .help("Path to Salary Pension Relationship File")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("ca_balm_rule_file_path")
                .long("ca-balm-rule-file")
                .value_name("CA BALM RULE FILE")
                .help("Path to the ca balm rule file")
                .required(true),
        )
        .arg(
            Arg::with_name("sa_balm_rule_file_path")
                .long("sa-balm-rule-file")
                .value_name("SA BALM RULE FILE")
                .help("Path to the sa balm rule file")
                .required(true),
        )
        .arg(
            Arg::with_name("td_balm_rule_file_path")
                .long("td-balm-rule-file")
                .value_name("TD BALM RULE FILE")
                .help("Path to the td balm rule file")
                .required(true),
        )
        .arg(
            Arg::with_name("rd_balm_rule_file_path")
                .long("rd-balm-rule-file")
                .value_name("RD BALM RULE FILE")
                .help("Path to the rd balm rule file")
                .required(true),
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true),
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
            Arg::with_name("log_file")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file")
                .value_name("Required Fields file")
                .help("Path to Required Fields File.")
                .required(true),
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
