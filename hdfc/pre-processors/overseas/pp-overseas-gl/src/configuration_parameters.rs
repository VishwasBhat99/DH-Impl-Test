use chrono::Local;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file: String,
    pub ref_file_path_1: String,
    pub alm_master_sheet_name: String,
    pub gl_moc_entry_file: String,
    pub gl_moc_sheet_name: String,
    pub gl_moc_ccy: String,
    pub gl_ex_master: String,
    pub as_on_date: String,
    pub output_file_path: String,
    pub concat_file_path: String,
    pub currency: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "ref_file_path_1: {}", self.ref_file_path_1());
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(logger, "gl_moc_entry_file: {}", self.gl_moc_entry_file());
        info!(logger, "gl_moc_sheet_name: {}", self.gl_moc_sheet_name());
        info!(logger, "gl_moc_ccy: {}", self.gl_moc_ccy());
        info!(logger, "gl_exclude_master: {}", self.gl_ex_master());
        info!(logger, "currency: {}", self.currency());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "concat_file: {}", self.concat_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let as_on_date = matches
            .value_of("as_on_date")
            .expect("Error getting `as_on_date` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let concat_file_path = matches
            .value_of("concat_file")
            .expect("Error getting `concat_file` value.")
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
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let ref_file_path_1 = matches
            .value_of("ref_file_1")
            .expect("Error getting `ref_file_1` value.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        let gl_moc_entry_file = matches
            .value_of("gl_moc_entry_file")
            .expect("Error getting `gl_moc_entry_file` value.")
            .to_string();
        let gl_moc_ccy = matches
            .value_of("gl_moc_ccy")
            .expect("Error getting `gl_moc_ccy` value.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency` value.")
            .to_string();
        let gl_moc_sheet_name = matches
            .value_of("gl_moc_sheet_name")
            .expect("Error getting `gl_moc_sheet_name` value.")
            .to_string();
        let gl_ex_master = matches
            .value_of("gl_ex_master")
            .expect("Error getting `gl_ex_master` value.")
            .to_string();
        ConfigurationParameters {
            input_file,
            ref_file_path_1,
            alm_master_sheet_name,
            gl_moc_entry_file,
            gl_moc_sheet_name,
            gl_moc_ccy,
            gl_ex_master,
            as_on_date,
            output_file_path,
            concat_file_path,
            currency,
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
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn ref_file_path_1(&self) -> &str {
        &self.ref_file_path_1
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn gl_moc_entry_file(&self) -> &str {
        &self.gl_moc_entry_file
    }
    pub fn gl_moc_sheet_name(&self) -> &str {
        &self.gl_moc_sheet_name
    }
    pub fn gl_moc_ccy(&self) -> &str {
        &self.gl_moc_ccy
    }
    pub fn gl_ex_master(&self) -> &str {
        &self.gl_ex_master
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
    pub fn currency(&self) -> &str {
        &self.currency
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
        .about("Pre Processor for Gl Balance!")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .version("1.2.4525")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file.")
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
            Arg::with_name("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("alm_master_sheet_name")
                .help("Sheet name of alm master file.")
                .default_value("Sheet1")
                .required(true)
        )
        .arg(
            Arg::with_name("gl_moc_entry_file")
                .long("gl-moc-file")
                .value_name("gl_moc_entry_file")
                .help("Path to GL MOC Entry input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("gl_moc_sheet_name")
                .long("gl-moc-sheet-name")
                .value_name("gl_moc_sheet_name")
                .help("Sheet name of GL MOC Entry input file.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::with_name("gl_moc_ccy")
                .long("gl-moc-ccy")
                .value_name("gl_moc_ccy")
                .help("GL MOC Currency.")
                .default_value("INR")
                .required(false)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("currency")
                .help("Currency.")
                .default_value("INR")
                .required(false)
        )
        .arg(
            Arg::with_name("gl_ex_master")
                .long("gl-ex-master")
                .value_name("gl_ex_master")
                .help("GL Exclude Master File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File Path")
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
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
