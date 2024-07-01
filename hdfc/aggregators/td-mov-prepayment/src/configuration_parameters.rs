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
    as_on_date: NaiveDate,
    input_file_path: String,
    output_file_path: String,
    edw_alm_td_file: String,
    ora_gl_master: String,
    ora_sheet: String,
    mis_desc_file: String,
    mis_sheet: String,
    llg_master_file: String,
    llg_sheet: String,
    tenor_desc_file: String,
    tenor_sheet: String,
    cust_cat_master: String,
    cust_sheet: String,
    lcr_master: String,
    wd_nwd_master: String,
    wd_nwd_sheet: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "edw_alm_td_file: {}", self.edw_alm_td_file());
        info!(logger, "ora_gl_master: {}", self.ora_gl_master());
        info!(logger, "ora_sheet: {}", self.ora_sheet());
        info!(logger, "mis_desc_file: {}", self.mis_desc_file());
        info!(logger, "mis_sheet: {}", self.mis_sheet());
        info!(logger, "llg_master_file: {}", self.llg_master_file());
        info!(logger, "llg_sheet: {}", self.llg_sheet());
        info!(logger, "tenor_desc_file: {}", self.tenor_desc_file());
        info!(logger, "tenor_sheet: {}", self.tenor_sheet());
        info!(logger, "cust_cat_master: {}", self.cust_cat_master());
        info!(logger, "cust_sheet: {}", self.cust_sheet());
        info!(logger, "lcr_master: {}", self.lcr_master());
        info!(logger, "wd_nwd_master: {}", self.wd_nwd_master());
        info!(logger, "wd_nwd_sheet: {}", self.wd_nwd_sheet());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches.value_of("input_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let edw_alm_td_file = matches
            .value_of("edw_alm_td_file")
            .expect("Error getting `edw_alm_td_file`.")
            .to_string();
        let ora_gl_master = matches
            .value_of("ora_gl_master")
            .expect("Error getting `ora_gl_master`.")
            .to_string();
        let ora_sheet = matches
            .value_of("ora_sheet")
            .expect("Error getting `ora_sheet`.")
            .to_string();
        let mis_desc_file = matches
            .value_of("mis_desc_file")
            .expect("Error getting `mis_desc_file`.")
            .to_string();
        let mis_sheet = matches
            .value_of("mis_sheet")
            .expect("Error getting `mis_sheet`.")
            .to_string();
        let llg_master_file = matches
            .value_of("llg_master_file")
            .expect("Error getting `llg_master_file`.")
            .to_string();
        let llg_sheet = matches
            .value_of("llg_sheet")
            .expect("Error getting `llg_sheet`.")
            .to_string();
        let tenor_desc_file = matches
            .value_of("tenor_desc_file")
            .expect("Error getting `tenor_desc_file`.")
            .to_string();
        let tenor_sheet = matches
            .value_of("tenor_sheet")
            .expect("Error getting `tenor_sheet`.")
            .to_string();
        let cust_cat_master = matches
            .value_of("cust_cat_master")
            .expect("Error getting `cust_cat_master`.")
            .to_string();
        let cust_sheet = matches
            .value_of("cust_sheet")
            .expect("Error getting `cust_sheet`.")
            .to_string();
        let lcr_master = matches
            .value_of("lcr_master")
            .expect("Error getting `lcr_master`.")
            .to_string();
        let wd_nwd_master = matches
            .value_of("wd_nwd_master")
            .expect("Error getting `wd_nwd_master`.")
            .to_string();
        let wd_nwd_sheet = matches
            .value_of("wd_nwd_sheet")
            .expect("Error getting `wd_nwd_sheet`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
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
            as_on_date,
            input_file_path,
            output_file_path,
            edw_alm_td_file,
            ora_gl_master,
            ora_sheet,
            mis_desc_file,
            mis_sheet,
            llg_master_file,
            llg_sheet,
            tenor_desc_file,
            tenor_sheet,
            cust_cat_master,
            cust_sheet,
            lcr_master,
            wd_nwd_master,
            wd_nwd_sheet,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn edw_alm_td_file(&self) -> &str {
        &self.edw_alm_td_file
    }
    pub fn ora_gl_master(&self) -> &str {
        &self.ora_gl_master
    }
    pub fn ora_sheet(&self) -> &str {
        &self.ora_sheet
    }
    pub fn mis_desc_file(&self) -> &str {
        &self.mis_desc_file
    }
    pub fn mis_sheet(&self) -> &str {
        &self.mis_sheet
    }
    pub fn llg_master_file(&self) -> &str {
        &self.llg_master_file
    }
    pub fn llg_sheet(&self) -> &str {
        &self.llg_sheet
    }
    pub fn tenor_desc_file(&self) -> &str {
        &self.tenor_desc_file
    }
    pub fn tenor_sheet(&self) -> &str {
        &self.tenor_sheet
    }
    pub fn cust_cat_master(&self) -> &str {
        &self.cust_cat_master
    }
    pub fn cust_sheet(&self) -> &str {
        &self.cust_sheet
    }
    pub fn lcr_master(&self) -> &str {
        &self.lcr_master
    }
    pub fn wd_nwd_master(&self) -> &str {
        &self.wd_nwd_master
    }
    pub fn wd_nwd_sheet(&self) -> &str {
        &self.wd_nwd_sheet
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
        .about("Generates reports for TD-Movement-Prepayment.")
        .version("1.2.3212")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("log_file")
                .long("log-file-path")
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
                .default_value("none")
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
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("input file")
                .help("Path to the input file")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output file")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On Date")
                .help("The date for which the program has to be processed.")
                .required(true),
        )
        .arg(
            Arg::with_name("edw_alm_td_file")
                .long("edw-alm-td-file")
                .value_name("EDW_ALM_TD_CLOSED_DDMMYYYY.CSV")
                .help("Path to reference file: EDW_ALM_TD_CLOSED_DDMMYYYY.csv.")
                .required(true),
        )
        .arg(
            Arg::with_name("ora_gl_master")
                .long("ora-gl-master")
                .value_name("ORA-GL Master file")
                .help("Path to reference file: ORA-GL Master.")
                .required(true),
        )
        .arg(
            Arg::with_name("ora_sheet")
                .long("ora-gl-sheet-name")
                .value_name("ORA-GL Master sheet name.")
                .help("Sheet name for: ORA-GL Master.")
                .required(true),
        )
        .arg(
            Arg::with_name("mis_desc_file")
                .long("mis-desc-file")
                .value_name("MIS_DESC Master file.")
                .help("Path to reference file: MIS_DESC Master.")
                .required(true),
        )
        .arg(
            Arg::with_name("mis_sheet")
                .long("mis-sheet-name")
                .value_name("MIS DESC Master sheet name.")
                .help("Sheet name for: MIS DESC Master.")
                .required(true),
        )
        .arg(
            Arg::with_name("llg_master_file")
                .long("llg-master-file")
                .value_name("Master_LLG Updated file.")
                .help("Path to: Master_LLG Updated file.")
                .required(true),
        )
        .arg(
            Arg::with_name("llg_sheet")
                .long("llg-sheet-name")
                .value_name("Master LLG Updated sheet name.")
                .help("Sheet name for: Master LLG Updated file.")
                .required(true),
        )
        .arg(
            Arg::with_name("tenor_desc_file")
                .long("tenor-desc-file")
                .value_name("tenor desc file")
                .help("Path to reference file: Tenor Desc.")
                .required(true),
        )
        .arg(
            Arg::with_name("tenor_sheet")
                .long("tenor-sheet-name")
                .value_name("Master Tenor Desc sheet.")
                .help("Sheet name for: Tenor Desc file.")
                .required(true),
        )
        .arg(
            Arg::with_name("cust_cat_master")
                .long("cust-cat-master")
                .value_name("Cust Category Master.")
                .help("Path to Cust Category Master file.")
                .required(true),
        )
        .arg(
            Arg::with_name("cust_sheet")
                .long("cust-sheet-name")
                .value_name("Cust Category sheet name.")
                .help("Sheet name for: Cust Category file.")
                .required(true),
        )
        .arg(
            Arg::with_name("lcr_master")
                .long("lcr-master-name")
                .value_name("LCR Master.")
                .help("Path to LCR Master file.")
                .required(true),
        )
        .arg(
            Arg::with_name("wd_nwd_master")
                .long("wd-nwd-master")
                .value_name("WD/NWD Master.")
                .help("Path to WD/NWD Master file.")
                .required(true),
        )
        .arg(
            Arg::with_name("wd_nwd_sheet")
                .long("wd-nwd-sheet")
                .value_name("WD/NWD Sheet.")
                .help("Sheet name for: WD/NWD Master file.")
                .required(true),
        )
        .get_matches()
}
