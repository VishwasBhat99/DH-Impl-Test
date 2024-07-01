use chrono::Local;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub ovrd_llg: String,
    pub liq_rpt_kwd: String,
    pub liq_rpt_fcy: String,
    pub liq_rpt_con: String,
    pub buc_def_out_file: String,
    pub ovrd_kwd_out: String,
    pub ovrd_fcy_out: String,
    pub ovrd_con_out: String,
    pub buc_moc_kwd_out: String,
    pub buc_moc_fcy_out: String,
    pub buc_moc_con_out: String,
    pub ovrd_llg_sheet_name: String,
    pub liq_rpt_kwd_sheet_name: String,
    pub liq_rpt_fcy_sheet_name: String,
    pub liq_rpt_con_sheet_name: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub output_sheet_name: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "ovrd_llg: {}", self.ovrd_llg());
        info!(logger, "liq_rpt_kwd: {}", self.liq_rpt_kwd());
        info!(logger, "liq_rpt_fcy: {}", self.liq_rpt_fcy());
        info!(logger, "liq_rpt_con: {}", self.liq_rpt_kwd());
        info!(logger, "buc_def_out_file: {}", self.buc_def_out_file());
        info!(logger, "ovrd_kwd_out: {}", self.ovrd_kwd_out());
        info!(logger, "ovrd_fcy_out: {}", self.ovrd_fcy_out());
        info!(logger, "ovrd_con_out: {}", self.ovrd_con_out());
        info!(logger, "buc_moc_kwd_out: {}", self.buc_moc_kwd_out());
        info!(logger, "buc_moc_fcy_out: {}", self.buc_moc_fcy_out());
        info!(logger, "buc_moc_con_out: {}", self.buc_moc_con_out());
        info!(
            logger,
            "ovrd_llg_sheet_name: {}",
            self.ovrd_llg_sheet_name()
        );
        info!(
            logger,
            "liq_rpt_kwd_sheet_name: {}",
            self.liq_rpt_kwd_sheet_name()
        );
        info!(
            logger,
            "liq_rpt_fcy_sheet_name: {}",
            self.liq_rpt_fcy_sheet_name()
        );
        info!(
            logger,
            "liq_rpt_con_sheet_name: {}",
            self.liq_rpt_con_sheet_name()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "output_sheet_name: {}", self.output_sheet_name());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );

        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let output_sheet_name = matches
            .value_of("output_sheet_name")
            .expect("Error getting `output_sheet_name` value.")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let mut diagnostics_file_path = matches
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
        let ovrd_llg = matches
            .value_of("ovrd_llg")
            .expect("Error getting `ovrd_llg` value.")
            .to_string();
        let liq_rpt_kwd = matches
            .value_of("liq_rpt_kwd")
            .expect("Error getting `liq_rpt_kwd` value.")
            .to_string();
        let liq_rpt_fcy = matches
            .value_of("liq_rpt_fcy")
            .expect("Error getting `liq_rpt_fcy` value.")
            .to_string();
        let liq_rpt_con = matches
            .value_of("liq_rpt_con")
            .expect("Error getting `liq_rpt_con` value.")
            .to_string();
        let buc_def_out_file = matches
            .value_of("buc_def_out_file")
            .expect("Error getting `buc_def_out_file` value.")
            .to_string();
        let ovrd_kwd_out = matches
            .value_of("ovrd_kwd_out")
            .expect("Error getting `ovrd_kwd_out` value.")
            .to_string();
        let ovrd_fcy_out = matches
            .value_of("ovrd_fcy_out")
            .expect("Error getting `ovrd_fcy_out` value.")
            .to_string();
        let ovrd_con_out = matches
            .value_of("ovrd_con_out")
            .expect("Error getting `ovrd_con_out` value.")
            .to_string();
        let buc_moc_kwd_out = matches
            .value_of("buc_moc_kwd_out")
            .expect("Error getting `buc_moc_kwd_out` value.")
            .to_string();
        let buc_moc_fcy_out = matches
            .value_of("buc_moc_fcy_out")
            .expect("Error getting `buc_moc_fcy_out` value.")
            .to_string();
        let buc_moc_con_out = matches
            .value_of("buc_moc_con_out")
            .expect("Error getting `buc_moc_con_out` value.")
            .to_string();
        let ovrd_llg_sheet_name = matches
            .value_of("ovrd_llg_sheet_name")
            .expect("Error getting `ovrd_llg` value.")
            .to_string();
        let liq_rpt_kwd_sheet_name = matches
            .value_of("liq_rpt_kwd_sheet_name")
            .expect("Error getting `liq_rpt_kwd_sheet_name` value.")
            .to_string();
        let liq_rpt_fcy_sheet_name = matches
            .value_of("liq_rpt_fcy_sheet_name")
            .expect("Error getting `liq_rpt_fcy_sheet_name` value.")
            .to_string();
        let liq_rpt_con_sheet_name = matches
            .value_of("liq_rpt_con_sheet_name")
            .expect("Error getting `liq_rpt_con_sheet_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            ovrd_llg,
            liq_rpt_kwd,
            liq_rpt_fcy,
            liq_rpt_con,
            buc_def_out_file,
            ovrd_kwd_out,
            ovrd_fcy_out,
            ovrd_con_out,
            buc_moc_kwd_out,
            buc_moc_fcy_out,
            buc_moc_con_out,
            ovrd_llg_sheet_name,
            liq_rpt_kwd_sheet_name,
            liq_rpt_fcy_sheet_name,
            liq_rpt_con_sheet_name,
            as_on_date,
            output_file_path,
            output_sheet_name,
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
    pub fn ovrd_llg(&self) -> &str {
        &self.ovrd_llg
    }
    pub fn liq_rpt_kwd(&self) -> &str {
        &self.liq_rpt_kwd
    }
    pub fn liq_rpt_fcy(&self) -> &str {
        &self.liq_rpt_fcy
    }
    pub fn liq_rpt_con(&self) -> &str {
        &self.liq_rpt_con
    }
    pub fn buc_def_out_file(&self) -> &str {
        &self.buc_def_out_file
    }
    pub fn ovrd_kwd_out(&self) -> &str {
        &self.ovrd_kwd_out
    }
    pub fn ovrd_fcy_out(&self) -> &str {
        &self.ovrd_fcy_out
    }
    pub fn ovrd_con_out(&self) -> &str {
        &self.ovrd_con_out
    }
    pub fn buc_moc_kwd_out(&self) -> &str {
        &self.buc_moc_kwd_out
    }
    pub fn buc_moc_fcy_out(&self) -> &str {
        &self.buc_moc_fcy_out
    }
    pub fn buc_moc_con_out(&self) -> &str {
        &self.buc_moc_con_out
    }
    pub fn ovrd_llg_sheet_name(&self) -> &str {
        &self.ovrd_llg_sheet_name
    }
    pub fn liq_rpt_kwd_sheet_name(&self) -> &str {
        &self.liq_rpt_kwd_sheet_name
    }
    pub fn liq_rpt_fcy_sheet_name(&self) -> &str {
        &self.liq_rpt_fcy_sheet_name
    }
    pub fn liq_rpt_con_sheet_name(&self) -> &str {
        &self.liq_rpt_con_sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn output_sheet_name(&self) -> &str {
        &self.output_sheet_name
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
        .about("Pre-adjuster for special bucket moc")
        .version("1.0.4138")
        .author("Ankur Gangwar <ankur.g@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ovrd_llg")
                .long("ovrd-llg")
                .value_name("REF_FILE_2")
                .help("Path to the reference files: R2.")
                .required(true)
        )
        .arg(
            Arg::with_name("liq_rpt_kwd")
                .long("liq-rpt-kwd")
                .value_name("REF_FILE_3")
                .help("Path to the reference files: R3.")
                .required(true)
        )
        .arg(
            Arg::with_name("liq_rpt_fcy")
                .long("liq-rpt-fcy")
                .value_name("REF_FILE_4")
                .help("Path to the reference files: R4.")
                .required(true)
        )
        .arg(
            Arg::with_name("liq_rpt_con")
                .long("liq-rpt-con")
                .value_name("REF_FILE_5")
                .help("Path to the reference files: R5.")
                .required(true)
        )
        .arg(
            Arg::with_name("buc_def_out_file")
                .long("buc-def-out-file")
                .value_name("BUC_DEF_OUT_FILE")
                .help("Path to the ")
                .required(true)
        )
        .arg(
            Arg::with_name("ovrd_llg_sheet_name")
                .long("ovrd-llg-sheet-name")
                .value_name("ovrd_llg_sheet_name")
                .help("ovrd_llg File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("liq_rpt_kwd_sheet_name")
                .long("liq-rpt-kwd-sheet-name")
                .value_name("liq_rpt_kwd_sheet_name")
                .help("liq_rpt_kwd File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("liq_rpt_fcy_sheet_name")
                .long("liq-rpt-fcy-sheet-name")
                .value_name("ref4_sheet_name")
                .help("ref-4 File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("liq_rpt_con_sheet_name")
                .long("liq-rpt-con-sheet-name")
                .value_name("ref5_sheet_name")
                .help("ref-5 File Sheet Name.")
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
            Arg::with_name("output_sheet_name")
                .long("output-sheet-name")
                .value_name("Output sheet name")
                .help("Output file sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("ovrd_kwd_out")
                .long("ovrd-kwd-out")
                .value_name("Output File")
                .help("Path to the ovrd_kwd_out file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ovrd_fcy_out")
                .long("ovrd-fcy-out")
                .value_name("Output File")
                .help("Path to the ovrd_fcy_out file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ovrd_con_out")
                .long("ovrd-con-out")
                .value_name("Output File")
                .help("Path to the ovrd_con_out file.")
                .required(true)
        )
        .arg(
            Arg::with_name("buc_moc_kwd_out")
                .long("buc-moc-kwd-out")
                .value_name("Output File")
                .help("Path to the buc_moc_kwd_out file.")
                .required(true)
        )
        .arg(
            Arg::with_name("buc_moc_fcy_out")
                .long("buc-moc-fcy-out")
                .value_name("Output File")
                .help("Path to the buc_moc_fcy_out file.")
                .required(true)
        )
        .arg(
            Arg::with_name("buc_moc_con_out")
                .long("buc-moc-con-out")
                .value_name("Output File")
                .help("Path to the buc_moc_con_out file.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
