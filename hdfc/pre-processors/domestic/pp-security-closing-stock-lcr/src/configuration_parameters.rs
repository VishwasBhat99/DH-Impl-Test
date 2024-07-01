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
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub murex_inv_master: String,
    pub isin_master: String,
    pub ora_gl: String,
    pub master_llg: String,
    pub master_llg_sheet_name: String,
    pub ora_gl_sheet_name: String,
    pub murex_inv_sheet_name: String,
    pub rec_output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub entity: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "murex_inv_master: {}", self.murex_inv_master());
        info!(
            logger,
            "murex_inv_sheet_name: {}",
            self.murex_inv_sheet_name()
        );
        info!(logger, "isin_master: {}", self.isin_master());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "rec_output_file: {}", self.rec_output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "entity: {}", self.entity());
        info!(logger, "ora_gl: {}", self.ora_gl());
        info!(logger, "master_llg: {}", self.master_llg());
        info!(
            logger,
            "master_llg_sheet_name: {}",
            self.master_llg_sheet_name()
        );
        info!(logger, "ora_gl_sheet_name: {}", self.ora_gl_sheet_name());
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
        let murex_inv_master = matches
            .value_of("murex_inv_master")
            .expect("Error getting `murex_inv_master` value.")
            .to_string();
        let murex_inv_sheet_name = matches
            .value_of("murex_inv_sheet_name")
            .expect("Error getting `murex_inv_sheet_name` value.")
            .to_string();
        let isin_master = matches
            .value_of("isin_master")
            .expect("Error getting `isin_master` value.")
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
        let entity = matches
            .value_of("entity")
            .expect("Error getting `entity` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let ora_gl = matches
            .value_of("ora_gl")
            .expect("Error getting `ora_gl` value.")
            .to_string();
        let master_llg = matches
            .value_of("master_llg")
            .expect("Error getting `master_llg` value.")
            .to_string();
        let master_llg_sheet_name = matches
            .value_of("master_llg_sheet_name")
            .expect("Error getting `master_llg_sheet_name` value.")
            .to_string();
        let ora_gl_sheet_name = matches
            .value_of("ora_gl_sheet_name")
            .expect("Error getting `ora_gl_sheet_name` value.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            as_on_date,
            output_file_path,
            murex_inv_master,
            isin_master,
            ora_gl,
            master_llg,
            master_llg_sheet_name,
            ora_gl_sheet_name,
            murex_inv_sheet_name,
            rec_output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            entity,
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
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn murex_inv_master(&self) -> &str {
        &self.murex_inv_master
    }
    pub fn murex_inv_sheet_name(&self) -> &str {
        &self.murex_inv_sheet_name
    }
    pub fn isin_master(&self) -> &str {
        &self.isin_master
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
    pub fn entity(&self) -> &str {
        &self.entity
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn ora_gl(&self) -> &str {
        &self.ora_gl
    }
    pub fn master_llg(&self) -> &str {
        &self.master_llg
    }
    pub fn master_llg_sheet_name(&self) -> &str {
        &self.master_llg_sheet_name
    }
    pub fn ora_gl_sheet_name(&self) -> &str {
        &self.ora_gl_sheet_name
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of Murex SecurityClosing Stock LCR CFGen!")
        .version("1.0.3772")
        .author("Srinivas Reddy <srinivas.r@surya-soft.com>")
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
            Arg::with_name("murex_inv_master")
                .long("murex-inv-master")
                .value_name("Murex Master File")
                .help("Path to the Murex Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("murex_inv_sheet_name")
                .long("murex-sheet-name")
                .value_name("Murex Master Sheet Name")
                .help("murex inv sheet name that has to be processed.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("entity")
                .long("entity")
                .value_name("Entity")
                .help("Country code that has to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ora_gl")
                .long("ora-gl")
                .value_name("ora_gl")
                .help("ora gl file that has to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("master_llg")
                .long("master-llg")
                .value_name("master_llg")
                .help("master llg file that has to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("master_llg_sheet_name")
                .long("master-llg-sheet-name")
                .value_name("master_llg_sheet_name")
                .help("master llg sheet name that has to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ora_gl_sheet_name")
                .long("ora-gl-sheet-name")
                .value_name("ora_gl_sheet_name")
                .help("ora gl sheet name that has to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("isin_master")
                .long("isin-master")
                .value_name("ISIN Master File")
                .help("Path to the ISIN Master file.")
                .required(true)
        )
        .get_matches()
}
