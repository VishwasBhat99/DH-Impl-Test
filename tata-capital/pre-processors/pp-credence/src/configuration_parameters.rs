use clap;
use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub input_file_path: String,
    pub investment_future_file_path: String,
    pub cred_gl_mapping_master_file_path: String,
    pub tcfsl_file_path: String,
    pub output_file_path: String,
    pub tcfsl_sheet_name: String,
    pub investment_future_sheet_name: String,
    pub cred_gl_mapping_master_sheet_name: String,
    pub alm_credence_manual_file_path: String,
    pub alm_credence_manual_sheet_name: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "alm_credence_manual_file_path: {}",
            self.alm_credence_manual_file_path()
        );
        info!(
            logger,
            "investment_future_file_path: {}",
            self.investment_future_file_path()
        );
        info!(
            logger,
            "cred_gl_mapping_master_file_path: {}",
            self.cred_gl_mapping_master_file_path()
        );
        info!(logger, "tcfsl_file_path: {}", self.tcfsl_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "tcfsl_sheet_name: {}", self.tcfsl_sheet_name());
        info!(
            logger,
            "investment_future_sheet_name: {}",
            self.investment_future_sheet_name()
        );
        info!(
            logger,
            "cred_gl_mapping_master_sheet_name: {}",
            self.cred_gl_mapping_master_sheet_name()
        );
        info!(
            logger,
            "alm_credence_manual_sheet_name: {}",
            self.alm_credence_manual_sheet_name()
        );
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
        let alm_credence_manual_file_path = matches
            .value_of("alm_credence_manual_file")
            .expect("Error getting `alm_credence_manual_file_path`.")
            .to_string();
        let investment_future_file_path = matches
            .value_of("investment_future_file")
            .expect("Error getting `investment_future_file_path`.")
            .to_string();
        let cred_gl_mapping_master_file_path = matches
            .value_of("cred_gl_mapping_master_file")
            .expect("Error getting `cred_gl_mapping_master_file_path`.")
            .to_string();
        let tcfsl_file_path = matches
            .value_of("tcfsl_file")
            .expect("Error getting `tcfsl_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let tcfsl_sheet_name = matches
            .value_of("tcfsl_sheet_name")
            .expect("Error getting `tcfsl_sheet_name`.")
            .to_string();
        let alm_credence_manual_sheet_name = matches
            .value_of("alm_credence_manual_sheet_name")
            .expect("Error getting `alm_credence_manual_sheet_name`.")
            .to_string();
        let investment_future_sheet_name = matches
            .value_of("investment_future_sheet_name")
            .expect("Error getting `investment_future_sheet_name`.")
            .to_string();
        let cred_gl_mapping_master_sheet_name = matches
            .value_of("cred_gl_mapping_master_sheet_name")
            .expect("Error getting `cred_gl_mapping_master_sheet_name`.")
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
            investment_future_file_path,
            cred_gl_mapping_master_file_path,
            tcfsl_file_path,
            output_file_path,
            tcfsl_sheet_name,
            alm_credence_manual_file_path,
            alm_credence_manual_sheet_name,
            cred_gl_mapping_master_sheet_name,
            investment_future_sheet_name,
            log_level,
            is_perf_diagnostics_enabled,
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
    pub fn cred_gl_mapping_master_file_path(&self) -> &str {
        &self.cred_gl_mapping_master_file_path
    }
    pub fn investment_future_file_path(&self) -> &str {
        &self.investment_future_file_path
    }
    pub fn alm_credence_manual_file_path(&self) -> &str {
        &self.alm_credence_manual_file_path
    }
    pub fn alm_credence_manual_sheet_name(&self) -> &str {
        &self.alm_credence_manual_sheet_name
    }
    pub fn tcfsl_file_path(&self) -> &str {
        &self.tcfsl_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn tcfsl_sheet_name(&self) -> &str {
        &self.tcfsl_sheet_name
    }
    pub fn investment_future_sheet_name(&self) -> &str {
        &self.investment_future_sheet_name
    }
    pub fn cred_gl_mapping_master_sheet_name(&self) -> &str {
        &self.cred_gl_mapping_master_sheet_name
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
        .about("pp_credence")
        .author("Ravindar-01 <ravinar.sr@surya-soft.com>")
        .version("1.4.3927")
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
                .value_name("Base Input File Path")
                .help("Path to read Base Input.")
                .required(true)
        )
        .arg(
            Arg::new("investment_future_file")
                .long("investment-future-file")
                .value_name("Investment future File Path")
                .help("Path to read Investment future.")
                .required(true)
        )
        .arg(
            Arg::new("alm_credence_manual_file")
                .long("alm-credence-manual-file")
                .value_name("alm-credence-manual File Path")
                .help("Path to read alm-credence-manual.")
                .required(true)
        )
        .arg(
            Arg::new("cred_gl_mapping_master_file")
                .long("cred-gl-mapping-master-file")
                .value_name("Cred Gl Mapping Master File Path")
                .help("Path to read Cred Gl Mapping Master.")
                .required(true)
        )
        .arg(
            Arg::new("tcfsl_file")
                .long("tcfsl-file")
                .value_name("TCFSL File Path")
                .help("Path to read TCFSL.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to write Output.")
                .required(true)
        )
        .arg(
            Arg::new("tcfsl_sheet_name")
                .long("tcfsl-sheet-name")
                .value_name("tcfsl Sheet Name")
                .help("Path to write tcfsl Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("alm_credence_manual_sheet_name")
                .long("alm-credence-sheet-name")
                .value_name("ALM credence Sheet Name")
                .help("Path to write ALM credence Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("investment_future_sheet_name")
                .long("investment-future-sheet-name")
                .value_name("investment_future Sheet Name")
                .help("Path to write investment_future Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("cred_gl_mapping_master_sheet_name")
                .long("cred-gl-mapping-master-sheet-name")
                .value_name("cred_gl_mapping_master Sheet Name")
                .help("Path to write cred_gl_mapping_master Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
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
        .get_matches()
}
