use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub asset_class_path: String,
    pub lcr_cat_path: String,
    pub v_src_system_ids: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub as_on_date: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub delimiter: String,
    pub lcr_cat_sheet_name: String,
    pub asset_class_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "asset_class: {}", self.asset_class_path());
        info!(logger, "lcr_cat: {}", self.lcr_cat_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "v_src_system_ids: {}", self.v_src_system_ids());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "delimiter: {}", self.delimiter());
        info!(logger, "lcr_cat_sheet_name: {}", self.lcr_cat_sheet_name());
        info!(
            logger,
            "asset_class_sheet_name: {}",
            self.asset_class_sheet_name()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let asset_class_path = matches
            .value_of("asset_class_path")
            .expect("Error getting `asset_class_path` value.")
            .to_string();
        let lcr_cat_path = matches
            .value_of("lcr_cat_path")
            .expect("Error getting `lcr_cat_path` value.")
            .to_string();
        let v_src_system_ids = matches
            .value_of("v_src_system_ids")
            .expect("Error getting `v_src_system_ids` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let as_on_date = matches
            .value_of("as_on_date")
            .expect("Error getting `as_on_date` value.")
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
        let delimiter = matches
            .value_of("delimiter")
            .expect("Error getting `delimiter` value.")
            .to_string();
        let lcr_cat_sheet_name = matches
            .value_of("lcr_cat_sheet_name")
            .expect("Error getting `lcr_cat_sheet_name` value.")
            .to_string();
        let asset_class_sheet_name = matches
            .value_of("asset_class_sheet_name")
            .expect("Error getting `asset_class_sheet_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            asset_class_path,
            lcr_cat_path,
            v_src_system_ids,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            as_on_date,
            log_level,
            is_perf_diagnostics_enabled,
            delimiter,
            lcr_cat_sheet_name,
            asset_class_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn asset_class_path(&self) -> &str {
        &self.asset_class_path
    }
    pub fn lcr_cat_path(&self) -> &str {
        &self.lcr_cat_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn v_src_system_ids(&self) -> &str {
        &self.v_src_system_ids
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn as_on_date(&self) -> &str {
        &self.as_on_date
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
    pub fn lcr_cat_sheet_name(&self) -> &str {
        &self.lcr_cat_sheet_name
    }
    pub fn asset_class_sheet_name(&self) -> &str {
        &self.asset_class_sheet_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of Undrawn Classification CFGen!")
        .version("1.1.5128")
        .author("Sougata Bhattacharjee <sougata.b@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("asset_class_path")
                .long("asset-class-path")
                .value_name("ASSET_CLASS_PATH")
                .help("Path to the reference files: Master Asset Class.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcr_cat_path")
                .long("lcr-cat-path")
                .value_name("LCR_CAT_PATH")
                .help("Path to the reference files: LCR Category Master.")
                .required(true)
        )
        .arg(
            Arg::with_name("v_src_system_ids")
                .long("v-src-system-ids")
                .value_name("V_SRC_SYSTEM_IDS")
                .help("Comma separated v_src_system_ids to be matched.")
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
                .long("lcr-sheet-name")
                .value_name("LCR CAT SHEET NAME")
                .help("The sheet name of LCR category.")
                .required(true)
        )
        .arg(
            Arg::with_name("asset_class_sheet_name")
                .long("asset-sheet-name")
                .value_name("ASSET CLASS SHEET NAME")
                .help("The sheet name of Asset class.")
                .required(true)
        )
        .get_matches()
}
