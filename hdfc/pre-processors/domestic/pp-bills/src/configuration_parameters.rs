use chrono::Local;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
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
    pub alm_master_sheet_name: String,
    pub as_on_date: NaiveDate,
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
    pub sma_file_path: String,
    pub data_src_name: String,
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
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
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
        info!(logger, "sma_file_path: {}", self.sma_file_path());
        info!(logger, "data_src_name: {}", self.data_src_name());
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
        let concat_file_path = matches
            .value_of("concat_file")
            .expect("Error getting `concat_file` value.")
            .to_string();
        let rec_output_file_path = matches
            .value_of("rec_output_file")
            .expect("Error getting `rec_output_file_path` value.")
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
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
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
        let sma_file_path = matches
            .value_of("sma_file_path")
            .expect("Error getting `sma_file_path` value.")
            .to_string();
        let data_src_name = matches
            .value_of("data_src_name")
            .expect("Error getting `data_src_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            ref_file_path_1,
            ref_file_path_2,
            ref_file_path_3,
            ref_file_path_4,
            ref_file_path_5,
            alm_master_sheet_name,
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
            sma_file_path,
            data_src_name,
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
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
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
    pub fn data_src_name(&self) -> &str {
        &self.data_src_name
    }
}

fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("Pre-processor for UBS Bills (Domestic).")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_1")
                .long("ref-file-1")
                .value_name("REF_FILE_1")
                .help("Path to the reference files: R1.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_2")
                .long("ref-file-2")
                .value_name("REF_FILE_2")
                .help("Path to the reference files: R2.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_3")
                .long("ref-file-3")
                .value_name("REF_FILE_3")
                .help("Path to the reference files: R3.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_4")
                .long("ref-file-4")
                .value_name("REF_FILE_4")
                .help("Path to the reference files: R4.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_5")
                .long("ref-file-5")
                .value_name("REF_FILE_5")
                .help("Path to the reference files: R5.")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("alm_master_sheet_name")
                .help("Alm Master File Sheet Name.")
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
            Arg::with_name("concat_file")
                .long("concat-file")
                .value_name("Concat File Path")
                .help("Path to the concat file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rec_output_file")
                .long("rec-output-file")
                .value_name("Reconcilation Output File")
                .help("Path to the reconcilation output file.")
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
            Arg::with_name("sma_file_path")
                .long("sma-file-path")
                .value_name("SMA_FILE")
                .help("Path to the SMA File")
                .required(true)
        )
        .arg(
            Arg::with_name("data_src_name")
                .long("data-src-name")
                .value_name("DATA_SRC_NAME")
                .help("Data Source Name")
                .required(true)
        )
        .get_matches()
}
