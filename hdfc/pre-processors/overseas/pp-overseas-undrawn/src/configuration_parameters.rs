use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub lcr_cat_path: String,
    pub cd_od_path: String,
    pub funded_path: String,
    pub as_on_date: String,
    pub output_file_path: String,
    pub cust_ref_code: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub delimiter: String,
    pub lcr_master_basel_path: String,
    pub lcr_master_sheet_name: String,
    pub lcr_cat_sheet_name: String,
    pub cd_od_sheet_name: String,
    pub funded_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "lcr_cat_path: {}", self.lcr_cat_path());
        info!(
            logger,
            "lcr_master_basel_path: {}",
            self.lcr_master_basel_path()
        );
        info!(
            logger,
            "lcr_master_sheet_name: {}",
            self.lcr_master_sheet_name()
        );
        info!(logger, "cd_od_path: {}", self.cd_od_path());
        info!(logger, "funded_path: {}", self.funded_path());
        info!(
            logger,
            "lcr_cat_sheet_name: {:?}",
            self.lcr_cat_sheet_name()
        );
        info!(logger, "cd_od_sheet_name: {:?}", self.cd_od_sheet_name());
        info!(logger, "funded_sheet_name: {:?}", self.funded_sheet_name());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "cust_ref_code: {}", self.cust_ref_code());
        info!(logger, "delimiter: {}", self.delimiter());
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
        let lcr_cat_sheet_name = matches
            .value_of("lcr_cat_sheet_name")
            .expect("Error getting `lcr_cat_sheet_name` value.")
            .to_string();
        let cd_od_sheet_name = matches
            .value_of("cd_od_sheet_name")
            .expect("Error getting `cd_od_sheet_name` value.")
            .to_string();
        let funded_sheet_name = matches
            .value_of("funded_sheet_name")
            .expect("Error getting `funded_sheet_name` value.")
            .to_string();
        let lcr_master_sheet_name = matches
            .value_of("lcr_master_sheet_name")
            .expect("Error getting `lcr_master_sheet_name` value.")
            .to_string();

        let lcr_master_basel_path = matches
            .value_of("lcr_master_basel_path")
            .expect("Error getting `lcr_master_basel_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let cust_ref_code = matches
            .value_of("cust_ref_code")
            .expect("Error getting `cust_ref_code` value.")
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
        let lcr_cat_path = matches
            .value_of("lcr_cat_path")
            .expect("Error getting `lcr_cat_path` value.")
            .to_string();
        let cd_od_path = matches
            .value_of("cd_od_path")
            .expect("Error getting `cd_od_path` value.")
            .to_string();
        let funded_path = matches
            .value_of("funded_path")
            .expect("Error getting `funded_path` value.")
            .to_string();
        let delimiter = matches
            .value_of("delimiter")
            .expect("Error getting `delimiter` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            lcr_cat_path,
            cd_od_path,
            funded_path,
            lcr_cat_sheet_name,
            funded_sheet_name,
            cd_od_sheet_name,
            as_on_date,
            output_file_path,
            cust_ref_code,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            delimiter,
            lcr_master_basel_path,
            lcr_master_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn lcr_cat_path(&self) -> &str {
        &self.lcr_cat_path
    }
    pub fn cd_od_path(&self) -> &str {
        &self.cd_od_path
    }
    pub fn funded_path(&self) -> &str {
        &self.funded_path
    }
    pub fn lcr_cat_sheet_name(&self) -> &str {
        &self.lcr_cat_sheet_name
    }
    pub fn funded_sheet_name(&self) -> &str {
        &self.funded_sheet_name
    }
    pub fn cd_od_sheet_name(&self) -> &str {
        &self.cd_od_sheet_name
    }
    pub fn lcr_master_sheet_name(&self) -> &str {
        &self.lcr_master_sheet_name
    }
    pub fn lcr_master_basel_path(&self) -> &str {
        &self.lcr_master_basel_path
    }
    pub fn as_on_date(&self) -> &str {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn cust_ref_code(&self) -> &str {
        &self.cust_ref_code
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
    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of Undrawn Classification CFGen!")
        .version("0.2.4460")
        .author("Prajwal Dhatwalia <prajwal.dh@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcr_cat_path")
                .long("lcr-cat-path")
                .value_name("LCR_CAT_PATH")
                .help("Path to the reference files: Master LCR Category.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcr_master_basel_path")
                .long("lcr-master-basel-path")
                .value_name("LCR_MASTER_BASEL_PATH")
                .help("Path to the reference files: Master LCR BASEL.")
                .required(true)
        )
        .arg(
            Arg::with_name("cd_od_path")
                .long("cd-od-path")
                .value_name("CD_OD_PATH")
                .help("Path to the reference files: CD OD WCDL Line Master.")
                .required(true)
        )
        .arg(
            Arg::with_name("funded_path")
                .long("funded-path")
                .value_name("FUNDED_PATH")
                .help("Path to the reference files: Funded Master.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_ref_code")
                .long("cust-ref-code")
                .value_name("CUST REF CODE")
                .help("Cust Ref Code.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics logs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("delimiter")
                .long("delimiter")
                .value_name("DELIMITER")
                .help("Specifies the input file field separator.")
                .default_value("|")
                .required(false)
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
        .arg(
            Arg::with_name("lcr_cat_sheet_name")
                .long("lcr-cat-sheet")
                .value_name("common SHEET NAME")
                .help("The sheet name of common Master reference file.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcr_master_sheet_name")
                .long("lcr-master-sheet-name")
                .value_name("LCR MASTER SHEET NAME")
                .help("The sheet name of Lcr master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cd_od_sheet_name")
                .long("cd-od-sheet-name")
                .value_name("CD OD Sheet name")
                .help("The sheet name of CD OD .")
                .required(true)
        )
        .arg(
            Arg::with_name("funded_sheet_name")
                .long("funded-sheet-name")
                .value_name("FUNDED sheet Sheet name")
                .help("The sheet name of FUNDED FILE .")
                .required(true)
        )
        .get_matches()
}
