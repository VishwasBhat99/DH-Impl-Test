use chrono::{Local, NaiveDate};
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub ref_file_path_1: String,
    pub ref_file_path_2: String,
    pub ref_file_path_3: String,
    pub ref_file_path_4: String,
    pub ref_file_path_5: String,
    pub ref_file_path_6: String,
    pub ref_file_path_7: String,
    pub ref_file_path_8: String,
    pub ref_file_path_9: String,
    pub ref_file_path_10: String,
    pub cust_type_file_path: String,
    pub alm_sheet_name: String,
    pub as_on_date: String,
    pub output_file_path: String,
    pub concat_file_path: String,
    pub rec_output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub weaker_sec_master_path: String,
    pub ews_weaker_master_path: String,
    pub weaker_sec_sheet_name: String,
    pub ews_master_sheet_name: String,
    pub mis2_master_sheet_name: String,
    pub concat_yieldgrp_sheet_name: String,
    pub master_ubs_coa_sheet_name: String,
    pub sma_file_path: String,
    pub data_source_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "ref_file_path_1: {}", self.ref_file_path_1());
        info!(logger, "ref_file_path_2: {}", self.ref_file_path_2());
        info!(logger, "ref_file_path_3: {}", self.ref_file_path_3());
        info!(logger, "ref_file_path_4: {}", self.ref_file_path_4());
        info!(logger, "ref_file_path_5: {}", self.ref_file_path_5());
        info!(logger, "ref_file_path_6: {}", self.ref_file_path_6());
        info!(logger, "ref_file_path_7: {}", self.ref_file_path_7());
        info!(logger, "ref_file_path_8: {}", self.ref_file_path_8());
        info!(logger, "ref_file_path_9: {}", self.ref_file_path_9());
        info!(logger, "ref_file_path_10: {}", self.ref_file_path_10());
        info!(
            logger,
            "cust_type_file_path: {}",
            self.cust_type_file_path()
        );
        info!(logger, "alm_sheet_name: {:?}", self.alm_sheet_name());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "concat_file: {}", self.concat_file_path());
        info!(logger, "rec_output_file: {}", self.rec_output_file_path());
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
        info!(
            logger,
            "mis2_master_sheet_name: {}",
            self.mis2_master_sheet_name()
        );
        info!(
            logger,
            "concat_yieldgrp_sheet_name: {}",
            self.concat_yieldgrp_sheet_name()
        );
        info!(
            logger,
            "master_ubs_coa_sheet_name: {}",
            self.master_ubs_coa_sheet_name()
        );
        info!(logger, "sma_file_path: {}", self.sma_file_path());
        info!(logger, "data_source_name: {}", self.data_source_name());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let as_on_date = matches
            .value_of("as_on_date")
            .expect("Error getting `as_on_date` value.")
            .to_string();
        let alm_sheet_name = matches
            .value_of("alm_sheet_name")
            .expect("Error getting `alm_sheet_name` value.")
            .to_string();
        let cust_type_file_path = matches
            .value_of("cust_type_file_path")
            .expect("Error getting `cust_type_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let concat_file_path = matches
            .value_of("concat_file")
            .expect("Error getting `concat_file` value.")
            .to_string();
        let rec_output_file_path = matches
            .value_of("rec_output_file")
            .expect("Error getting `rec_output_file` value.")
            .to_string();
        let timestamp = Local::now()
            .naive_local()
            .format("%d%m%Y_%H%M%S")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        log_file_path = log_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let mut diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        diagnostics_file_path = diagnostics_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let ref_file_path_1 = matches
            .value_of("ref_file_1")
            .expect("Error getting `ref_file_1` value.")
            .to_string();
        let ref_file_path_2 = matches
            .value_of("ref_file_2")
            .expect("Error getting `ref_file_2` value.")
            .to_string();
        let ref_file_path_3 = matches
            .value_of("ref_file_3")
            .expect("Error getting `ref_file_3` value.")
            .to_string();
        let ref_file_path_4 = matches
            .value_of("ref_file_4")
            .expect("Error getting `ref_file_4` value.")
            .to_string();
        let ref_file_path_5 = matches
            .value_of("ref_file_5")
            .expect("Error getting `ref_file_5` value.")
            .to_string();
        let ref_file_path_6 = matches
            .value_of("ref_file_6")
            .expect("Error getting `ref_file_6` value.")
            .to_string();
        let ref_file_path_7 = matches
            .value_of("ref_file_7")
            .expect("Error getting `ref_file_7` value.")
            .to_string();
        let ref_file_path_8 = matches
            .value_of("ref_file_8")
            .expect("Error getting `ref_file_8` value.")
            .to_string();
        let ref_file_path_9 = matches
            .value_of("ref_file_9")
            .expect("Error getting `ref_file_9` value.")
            .to_string();
        let ref_file_path_10 = matches
            .value_of("ref_file_10")
            .expect("Error getting `ref_file_10` value.")
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
        let mis2_master_sheet_name = matches
            .value_of("mis2_master_sheet_name")
            .expect("Error getting `mis2_master_sheet_name` value.")
            .to_string();
        let concat_yieldgrp_sheet_name = matches
            .value_of("concat_yieldgrp_sheet_name")
            .expect("Error getting `concat_yieldgrp_sheet_name` value.")
            .to_string();
        let master_ubs_coa_sheet_name = matches
            .value_of("master_ubs_coa_sheet_name")
            .expect("Error getting `master_ubs_coa_sheet_name` value.")
            .to_string();
        let sma_file_path = matches
            .value_of("sma_file_path")
            .expect("Error getting `sma_file_path` value.")
            .to_string();
        let data_source_name = matches
            .value_of("data_source_name")
            .expect("Error getting `data_source_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            ref_file_path_1,
            ref_file_path_2,
            ref_file_path_3,
            ref_file_path_4,
            ref_file_path_5,
            ref_file_path_6,
            ref_file_path_7,
            ref_file_path_8,
            ref_file_path_9,
            ref_file_path_10,
            cust_type_file_path,
            alm_sheet_name,
            as_on_date,
            output_file_path,
            concat_file_path,
            rec_output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            weaker_sec_master_path,
            ews_weaker_master_path,
            weaker_sec_sheet_name,
            ews_master_sheet_name,
            mis2_master_sheet_name,
            concat_yieldgrp_sheet_name,
            master_ubs_coa_sheet_name,
            sma_file_path,
            data_source_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn weaker_sec_master_path(&self) -> &str {
        &self.weaker_sec_master_path
    }
    pub fn ews_master_sheet_name(&self) -> &str {
        &self.ews_master_sheet_name
    }
    pub fn weaker_sec_sheet_name(&self) -> &str {
        &self.weaker_sec_sheet_name
    }
    pub fn cust_type_file_path(&self) -> &str {
        &self.cust_type_file_path
    }
    pub fn ews_weaker_master_path(&self) -> &str {
        &self.ews_weaker_master_path
    }
    pub fn ref_file_path_1(&self) -> &str {
        &self.ref_file_path_1
    }
    pub fn ref_file_path_2(&self) -> &str {
        &self.ref_file_path_2
    }
    pub fn ref_file_path_3(&self) -> &str {
        &self.ref_file_path_3
    }
    pub fn ref_file_path_4(&self) -> &str {
        &self.ref_file_path_4
    }
    pub fn ref_file_path_5(&self) -> &str {
        &self.ref_file_path_5
    }
    pub fn ref_file_path_6(&self) -> &str {
        &self.ref_file_path_6
    }
    pub fn ref_file_path_7(&self) -> &str {
        &self.ref_file_path_7
    }
    pub fn ref_file_path_8(&self) -> &str {
        &self.ref_file_path_8
    }
    pub fn ref_file_path_9(&self) -> &str {
        &self.ref_file_path_9
    }
    pub fn ref_file_path_10(&self) -> &str {
        &self.ref_file_path_10
    }
    pub fn mis2_master_sheet_name(&self) -> &str {
        &self.mis2_master_sheet_name
    }
    pub fn concat_yieldgrp_sheet_name(&self) -> &str {
        &self.concat_yieldgrp_sheet_name
    }
    pub fn alm_sheet_name(&self) -> &str {
        &self.alm_sheet_name
    }
    pub fn master_ubs_coa_sheet_name(&self) -> &str {
        &self.master_ubs_coa_sheet_name
    }
    pub fn as_on_date(&self) -> &str {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn concat_file_path(&self) -> &str {
        &self.concat_file_path
    }
    pub fn rec_output_file_path(&self) -> &str {
        &self.rec_output_file_path
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
    pub fn sma_file_path(&self) -> &str {
        &self.sma_file_path
    }
    pub fn data_source_name(&self) -> &str {
        &self.data_source_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of UBS Loans CFGen!")
        .version("1.3.4810")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_1")
                .long("ref-file-1")
                .value_name("REF_FILE_1")
                .help("Path to the reference files: MIS1.")
                .required(true)
        )
        .arg(
            Arg::new("cust_type_file_path")
                .long("cust-type-file-path")
                .value_name("CUST_TYPE_FILE_PATH")
                .help("Path to the cust-type file")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_2")
                .long("ref-file-2")
                .value_name("REF_FILE_2")
                .help("Path to the reference files: Rate Code Lookup File.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_3")
                .long("ref-file-3")
                .value_name("REF_FILE_3")
                .help("Path to the reference files: Ora GL Lookup File.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_4")
                .long("ref-file-4")
                .value_name("REF_FILE_4")
                .help("Path to the reference files: ALM line Lookup File.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_5")
                .long("ref-file-5")
                .value_name("REF_FILE_5")
                .help("Path to the reference files: ORA PROD.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_6")
                .long("ref-file-6")
                .value_name("REF_FILE_6")
                .help("Path to the reference files: Spread File.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_7")
                .long("ref-file-7")
                .value_name("REF_FILE_7")
                .help("Path to the reference files: NPA Master File.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_8")
                .long("ref-file-8")
                .value_name("REF_FILE_8")
                .help("Path to the reference files: MIS 2.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_9")
                .long("ref-file-9")
                .value_name("REF_FILE_9")
                .help("Path to the reference files: Concat YieldGrp_WR.")
                .required(true)
        )
        .arg(
            Arg::new("ref_file_10")
                .long("ref-file-10")
                .value_name("REF_FILE_10")
                .help("Path to the reference files: Master_UBS_COA.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("concat_file")
                .long("concat-file")
                .value_name("Concat File Path")
                .help("Path to the concat file.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs file.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs file.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
        .arg(
            Arg::new("alm_sheet_name")
                .long("alm-sheet-name")
                .value_name("ALM SHEET NAME")
                .help("The sheet name of ALM Master reference file.")
                .default_value("Sheet1")
                .required(true)
        )
        .arg(
            Arg::new("rec_output_file")
                .long("rec-output-file")
                .value_name("FILE")
                .help("Path to the reconciliation output file")
                .required(true)
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
            Arg::with_name("mis2_master_sheet_name")
                .long("mis2-master-sheet-name")
                .value_name("MIS2 MASTER SHEET NAME")
                .help("MIS2 Weaker master sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("concat_yieldgrp_sheet_name")
                .long("concat-yield-grp-sheet-name")
                .value_name("CONCAT YIELD-GRP MASTER SHEET NAME")
                .help("Concat YieldGrp master sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("master_ubs_coa_sheet_name")
                .long("master-ubs-coa-sheet-name")
                .value_name("UBS COA MASTER SHEET NAME")
                .help("UBS COA master sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("sma_file_path")
                .long("sma-file")
                .value_name("sma-file")
                .help("to read the contents of sma file")
                .required(true)
        )
        .arg(
            Arg::with_name("data_source_name")
                .long("data-source-name")
                .value_name("data-source-name")
                .help("to compare with second field from sma-file")
                .required(true)
        )
        .get_matches()
}
