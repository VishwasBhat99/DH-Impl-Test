use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub mis1_desc_file_path: String,
    pub bdp_coa_file_path: String,
    pub core_master: String,
    pub core_master_sheet_name: String,
    pub ora_gl_master: String,
    pub ora_gl_sheet_name: String,
    pub amb_file_path: String,
    pub amb_file_delimeter: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub weaker_sec_master_path: String,
    pub ews_weaker_master_path: String,
    pub weaker_sec_sheet_name: String,
    pub ews_master_sheet_name: String,
    pub bdp_coa_sheet_name: String,
    pub mis1_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "core_master: {}", self.core_master());
        info!(
            logger,
            "core_master_sheet_name: {}",
            self.core_master_sheet_name()
        );
        info!(logger, "mis1_desc_file_path: {}", self.mis1_desc_file_path);
        info!(logger, "bdp_coa_file_path: {}", self.bdp_coa_file_path);
        info!(logger, "ora_gl_master: {}", self.ora_gl_master());
        info!(logger, "bdp_coa_sheet_name: {}", self.bdp_coa_sheet_name());
        info!(logger, "mis1_sheet_name: {}", self.mis1_sheet_name());
        info!(logger, "ora_gl_sheet_name: {}", self.ora_gl_sheet_name());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "amb_file_path: {}", self.amb_file_path());
        info!(logger, "amb_file_delimeter: {}", self.amb_file_delimeter());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "weaker_sec_master_file: {}",
            self.weaker_sec_master_path()
        );
        info!(
            logger,
            "ews_weaker_master_path: {}",
            self.ews_weaker_master_path()
        );
        info!(
            logger,
            "weaker_sec_sheet_name: {}",
            self.weaker_sec_sheet_name()
        );
        info!(
            logger,
            "ews_master_sheet_name: {}",
            self.ews_master_sheet_name()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let amb_file_path = matches
            .value_of("amb_file_path")
            .expect("Error getting `amb_file_path` value.")
            .to_string();
        let amb_file_delimeter = matches
            .value_of("amb_file_delimeter")
            .expect("Error getting `amb_file_delimeter` value.")
            .to_string();
        let mis1_desc_file_path = matches
            .value_of("mis1_desc_file_path")
            .expect("Error getting `mis1_desc_file_path` value.")
            .to_string();
        let bdp_coa_file_path = matches
            .value_of("bdp_coa_file_path")
            .expect("Error getting `bdp_coa_file_path` value.")
            .to_string();
        let bdp_coa_sheet_name = matches
            .value_of("bdp_coa_sheet_name")
            .expect("Error getting `bdp_coa_sheet_name` value.")
            .to_string();
        let mis1_sheet_name = matches
            .value_of("mis1_sheet_name")
            .expect("Error getting `mis1_sheet_name` value.")
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
        let core_master = matches
            .value_of("core_master")
            .expect("Error getting `core_master` value.")
            .to_string();
        let core_master_sheet_name = matches
            .value_of("core_master_sheet_name")
            .expect("Error getting `core_master_sheet_name` value.")
            .to_string();
        let weaker_sec_master_path = matches
            .value_of("weaker_sec_master_path")
            .expect("Error getting `weaker_sec_master_path` value.")
            .to_string();
        let ews_weaker_master_path = matches
            .value_of("ews_weaker_master_path")
            .expect("Error getting `ews_weaker_master_path` value.")
            .to_string();
        let weaker_sec_sheet_name = matches
            .value_of("weaker_sec_sheet_name")
            .expect("Error getting `weaker_sec_sheet_name` value.")
            .to_string();
        let ews_master_sheet_name = matches
            .value_of("ews_master_sheet_name")
            .expect("Error getting `ews_master_sheet_name` value.")
            .to_string();
        let ora_gl_master = matches
            .value_of("ora_gl_master")
            .expect("Error getting `ora_gl_master` value.")
            .to_string();
        let ora_gl_sheet_name = matches
            .value_of("ora_gl_sheet_name")
            .expect("Error getting `ora_gl_sheet_name` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            core_master,
            core_master_sheet_name,
            ora_gl_master,
            mis1_desc_file_path,
            bdp_coa_file_path,
            mis1_sheet_name,
            bdp_coa_sheet_name,
            ora_gl_sheet_name,
            amb_file_path,
            amb_file_delimeter,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            weaker_sec_master_path,
            ews_weaker_master_path,
            weaker_sec_sheet_name,
            ews_master_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn ora_gl_master(&self) -> &str {
        &self.ora_gl_master
    }
    pub fn ora_gl_sheet_name(&self) -> &str {
        &self.ora_gl_sheet_name
    }
    pub fn mis1_desc_file_path(&self) -> &str {
        &self.mis1_desc_file_path
    }
    pub fn bdp_coa_file_path(&self) -> &str {
        &self.bdp_coa_file_path
    }
    pub fn bdp_coa_sheet_name(&self) -> &str {
        &self.bdp_coa_sheet_name
    }
    pub fn mis1_sheet_name(&self) -> &str {
        &self.mis1_sheet_name
    }
    pub fn weaker_sec_master_path(&self) -> &str {
        &self.weaker_sec_master_path
    }
    pub fn ews_weaker_master_path(&self) -> &str {
        &self.ews_weaker_master_path
    }
    pub fn ews_master_sheet_name(&self) -> &str {
        &self.ews_master_sheet_name
    }
    pub fn weaker_sec_sheet_name(&self) -> &str {
        &self.weaker_sec_sheet_name
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn amb_file_path(&self) -> &str {
        &self.amb_file_path
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
    pub fn core_master(&self) -> &str {
        &self.core_master
    }
    pub fn core_master_sheet_name(&self) -> &str {
        &self.core_master_sheet_name
    }
    pub fn amb_file_delimeter(&self) -> &str {
        &self.amb_file_delimeter
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
    .version("2.2.4961")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app modifies data to conform with the input requirements of OD CFGen!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("mis1_desc_file_path")
                .long("mis1-desc")
                .value_name("MIS1 Desc  File Path")
                .help("MIS1 File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("bdp_coa_file_path")
                .long("bdp-coa-file")
                .value_name("Bdp Coa File Path")
                .help("Bdp Coa File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("mis1_sheet_name")
                .long("mis1-sheet-name")
                .value_name("MIS1 SHEET Name")
                .help("mis1 file sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("bdp_coa_sheet_name")
                .long("bdp-coa-sheet-name")
                .value_name("Bdp Coa Sheet name")
                .help("Bdp Coa sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("amb_file_path")
                .long("amb-file")
                .value_name("AMB File Path")
                .help("AMB File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("amb_file_delimeter")
                .long("amb-file-delimeter")
                .value_name("AMB File Delimeter")
                .help("AMB File Delimeter.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Diagnostics log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
            Arg::with_name("core_master")
                .long("core-master")
                .value_name("core_master")
                .help("Master file for core / non-core determination.")
                .required(true)
        )
        .arg(
            Arg::with_name("core_master_sheet_name")
                .long("core-master-sheet-name")
                .value_name("core_master_sheet_name")
                .help("Sheet name for Core Master file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("weaker_sec_master_path")
                .long("weaker-sec-master")
                .value_name("WEAKER SECTION MASTER FILE")
                .help("Path to the Weaker section master file")
                .required(true)
        )
        .arg(
            Arg::with_name("ews_weaker_master_path")
                .long("ews-weaker-master")
                .value_name("EWS WEAKER MASTER FILE")
                .help("Path to the EWS Weaker master file")
                .required(true)
        )
        .arg(
            Arg::with_name("weaker_sec_sheet_name")
                .long("weaker-sec-sheet-name")
                .value_name("WEAKER SECTION MASTER SHEET NAME")
                .help("Weaker section master sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("ews_master_sheet_name")
                .long("ews-weaker-sheet-name")
                .value_name("EWS WEAKER MASTER SHEET NAME")
                .help("EWS Weaker master sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("ora_gl_master")
                .long("ora-gl-master")
                .value_name("ora_gl_master")
                .help("Master file for two point concat determination.")
                .required(true)
        )
        .arg(
            Arg::with_name("ora_gl_sheet_name")
                .long("ora-gl-sheet-name")
                .value_name("ora_gl_sheet_name")
                .help("Sheet name for ORA GL Master file.")
                .default_value("Sheet1")
                .required(false)
        )
        .get_matches()
}
