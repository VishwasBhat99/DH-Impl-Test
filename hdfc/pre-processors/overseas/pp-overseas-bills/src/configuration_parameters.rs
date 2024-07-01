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
    pub ref_file_path_1: String,
    pub ref_file_path_2: String,
    pub ref_file_path_3: String,
    pub ref_file_path_4: String,
    pub ref_file_path_5: String,
    pub alm_master_sheet_name: String,
    pub sheet_name: String,
    pub ref1_sheet_name: String,
    pub ref2_sheet_name: String,
    pub ref4_sheet_name: String,
    pub ref5_sheet_name: String,
    pub gl_type: String,
    pub alm_line: String,
    pub input_file_name: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub output_concat_file_path: String,
    pub rec_output_file_path: String,
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
        info!(logger, "input_file_sheet_name: {}", self.sheet_name());
        info!(logger, "ref1_sheet_name: {}", self.ref1_sheet_name());
        info!(logger, "ref2_sheet_name: {}", self.ref2_sheet_name());
        info!(logger, "ref4_sheet_name: {}", self.ref4_sheet_name());
        info!(logger, "ref5_sheet_name: {}", self.ref5_sheet_name());
        info!(logger, "gl_type: {}", self.gl_type());
        info!(logger, "alm_line: {}", self.alm_line());
        info!(logger, "input_file_name: {}", self.input_file_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(
            logger,
            "output_concat_file_path: {}",
            self.output_concat_file_path()
        );
        info!(logger, "rec_output_file: {}", self.rec_output_file_path());
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
        let output_concat_file_path = matches
            .value_of("output_concat_file")
            .expect("Error getting `output_concat_file` value.")
            .to_string();
        let rec_output_file_path = matches
            .value_of("rec_output_file")
            .expect("Error getting `rec_output_file_path` value.")
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
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name` value.")
            .to_string();
        let ref1_sheet_name = matches
            .value_of("ref1_sheet_name")
            .expect("Error getting `ref1_sheet_name` value.")
            .to_string();
        let ref2_sheet_name = matches
            .value_of("ref2_sheet_name")
            .expect("Error getting `ref2_sheet_name` value.")
            .to_string();
        let ref4_sheet_name = matches
            .value_of("ref4_sheet_name")
            .expect("Error getting `ref4_sheet_name` value.")
            .to_string();
        let ref5_sheet_name = matches
            .value_of("ref5_sheet_name")
            .expect("Error getting `ref5_sheet_name` value.")
            .to_string();
        let gl_type = matches
            .value_of("gl_type")
            .expect("Error getting `gl_type` value.")
            .to_string();
        let alm_line = matches
            .value_of("alm_line")
            .expect("Error getting `alm_line` value.")
            .to_string();
        let input_file_name = matches
            .value_of("input_file_name")
            .expect("Error getting `input_file_name` value.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            ref_file_path_1,
            ref_file_path_2,
            ref_file_path_3,
            ref_file_path_4,
            ref_file_path_5,
            alm_master_sheet_name,
            sheet_name,
            ref1_sheet_name,
            ref2_sheet_name,
            ref4_sheet_name,
            ref5_sheet_name,
            gl_type,
            alm_line,
            input_file_name,
            as_on_date,
            output_file_path,
            output_concat_file_path,
            rec_output_file_path,
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
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn ref1_sheet_name(&self) -> &str {
        &self.ref1_sheet_name
    }
    pub fn ref2_sheet_name(&self) -> &str {
        &self.ref2_sheet_name
    }
    pub fn ref4_sheet_name(&self) -> &str {
        &self.ref4_sheet_name
    }
    pub fn ref5_sheet_name(&self) -> &str {
        &self.ref5_sheet_name
    }
    pub fn gl_type(&self) -> &str {
        &self.gl_type
    }
    pub fn alm_line(&self) -> &str {
        &self.alm_line
    }
    pub fn input_file_name(&self) -> &str {
        &self.input_file_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn output_concat_file_path(&self) -> &str {
        &self.output_concat_file_path
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Pre-processor for Overseas Bills.")
        .version("1.3.2024")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
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
            Arg::with_name("sheet_name")
                .long("sheet-name")
                .value_name("sheet_name")
                .help("Input file sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref1_sheet_name")
                .long("ref1-sheet-name")
                .value_name("Ref1 Sheet Name")
                .help("Sheet name of Ref1 input file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("ref2_sheet_name")
                .long("ref2-sheet-name")
                .value_name("Ref2 Sheet Name")
                .help("Sheet name of Ref2 input file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("ref4_sheet_name")
                .long("ref4-sheet-name")
                .value_name("Ref4 Sheet Name")
                .help("Sheet name of Ref4 input file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("ref5_sheet_name")
                .long("ref5-sheet-name")
                .value_name("Ref5 Sheet Name")
                .help("Sheet name of Ref5 input file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("gl_type")
                .long("gl-type")
                .value_name("gl_type")
                .possible_values(&["BH-Over-Bills", "GC-Over-Bills","HK-Over-Bills"])
                .help("Gl type.")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_line")
                .long("alm-line")
                .value_name("alm_line")
                .help("Alm Line.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_name")
                .long("input-file-name")
                .value_name("input_file_name")
                .help("Input file name.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("OUTPUT_FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_concat_file")
                .long("output-concat-file")
                .value_name("OUTPUT_CONCAT_FILE")
                .help("Path to the output concat file.")
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
                .value_name("LOG_FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAG_LOG_FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG_LEVEL")
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
