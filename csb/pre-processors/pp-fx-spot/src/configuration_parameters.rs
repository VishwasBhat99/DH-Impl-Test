use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub alm_master: String,
    pub alm_master_sheet_name: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub concat_file_path: String,
    pub log_file_path: String,
    pub rec_output_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub diagnostics_flag: bool,
    pub gl_mapping_file: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "alm_master: {}", self.alm_master());
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "concat_file_path: {}", self.concat_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "diagnostics_flag: {}", self.diagnostics_flag());
        info!(
            logger,
            "rec_output_file_path: {}",
            self.rec_output_file_path()
        );
        info!(
            logger,
            "diagnostics_file_path: {}",
            self.diagnostics_file_path()
        );
        info!(logger, "gl_mapping_file: {}", self.gl_mapping_file());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let alm_master = matches
            .value_of("alm_master")
            .expect("Error getting `alm_master` value.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let concat_file_path = matches
            .value_of("concat_file_path")
            .expect("Error getting `concat_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file_path")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let rec_output_file_path = matches
            .value_of("rec_output_file_path")
            .expect("Error getting `rec_output_file_path` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_file_path")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let diagnostics_flag = matches
            .value_of("diagnostics_flag")
            .expect("Error getting `diagnostics_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `diagnostics_flag` value as bool.");
        let gl_mapping_file = matches
            .value_of("gl_mapping_file")
            .expect("Error getting `gl_mapping_file` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            alm_master,
            alm_master_sheet_name,
            as_on_date,
            output_file_path,
            concat_file_path,
            log_file_path,
            rec_output_file_path,
            diagnostics_file_path,
            log_level,
            diagnostics_flag,
            gl_mapping_file,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn alm_master(&self) -> &str {
        &self.alm_master
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn concat_file_path(&self) -> &str {
        &self.concat_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn diagnostics_flag(&self) -> bool {
        self.diagnostics_flag
    }
    pub fn rec_output_file_path(&self) -> &str {
        &self.rec_output_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn gl_mapping_file(&self) -> &str {
        &self.gl_mapping_file
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of FX Spot PP!")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("concat_file_path")
                .long("concat-file-path")
                .value_name("Concat File")
                .help("Path to the concat file for the gls which are not present in master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rec_output_file_path")
                .long("rec-output-file-path")
                .value_name("Reconcilation Output File")
                .help("Path to the reconcilation output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_file_path")
                .long("diagnostics-file-path")
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
            Arg::with_name("diagnostics_flag")
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
                .help("The date for which program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_master")
                .long("alm-master")
                .value_name("alm_master")
                .help("Master file for llg determination.")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("alm_master_sheet_name")
                .help("Sheet name for Master LLG file.")
                .required(true)
        )
        .arg(
            Arg::with_name("gl_mapping_file")
                .long("gl-mapping-file")
                .value_name("GL Map File")
                .help("Path to the GL Map file.")
                .required(true)
        )
        .get_matches()
}
