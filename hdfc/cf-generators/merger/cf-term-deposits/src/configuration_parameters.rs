use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_cf_file_path: String,
    metadata_file_path: String,
    req_fields_file_path: String,
    npa_class_file_path: String,
    common_cust_file_path: String,
    finnone_master_file_path: String,
    risk_weight_file_path: String,
    resid_file_path: String,
    restructure_flag_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    master_llg_file_path: String,
    ora_gl_file_path: String,
    ora_gl_sheet_name: String,
    master_llg_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "metadata_file_path: {}", self.metadata_file_path());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(
            logger,
            "npa_class_file_path: {}",
            self.npa_class_file_path()
        );
        info!(
            logger,
            "common_cust_file_path: {}",
            self.common_cust_file_path()
        );
        info!(
            logger,
            "finnone_master_file_path: {}",
            self.finnone_master_file_path()
        );
        info!(
            logger,
            "risk_weight_file_path: {}",
            self.risk_weight_file_path()
        );
        info!(logger, "resid_file_path: {}", self.resid_file_path());
        info!(
            logger,
            "restructure_flag_file_path: {}",
            self.restructure_flag_file_path()
        );
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_cf_file: {}", self.input_cf_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "ora_gl_file_path: {}", self.ora_gl_file_path());
        info!(logger, "ora_gl_sheet_name: {}", self.ora_gl_sheet_name());
        info!(
            logger,
            "master_llg_sheet_name: {}",
            self.master_llg_sheet_name()
        );
        info!(
            logger,
            "master_llg_file_path: {}",
            self.master_llg_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_cf_file_path = matches
            .value_of("input_cf_file_path")
            .expect("Error getting `input_cf_file_value`.")
            .to_string();
        let metadata_file_path = matches
            .value_of("metadata_file_path")
            .expect("Error getting `metadata_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file_path")
            .expect("Error getting `req_fields_file_path`.")
            .to_string();
        let npa_class_file_path = matches
            .value_of("npa_class_file_path")
            .expect("Error getting `npa_class_file_path`.")
            .to_string();
        let common_cust_file_path = matches
            .value_of("common_cust_file_path")
            .expect("Error getting `common_cust_file_path`.")
            .to_string();
        let finnone_master_file_path = matches
            .value_of("finnone_master_file_path")
            .expect("Error getting `finnone_master_file_path`.")
            .to_string();
        let risk_weight_file_path = matches
            .value_of("risk_weight_file_path")
            .expect("Error getting `risk_weight_file_path`.")
            .to_string();
        let resid_file_path = matches
            .value_of("resid_file_path")
            .expect("Error getting `resid_file_path`.")
            .to_string();
        let restructure_flag_file_path = matches
            .value_of("restructure_flag_file_path")
            .expect("Error getting `restructure_flag_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");
        let ora_gl_file_path = matches
            .value_of("ora_gl_file_path")
            .expect("Error getting `ora_gl_file_path`.")
            .to_string();
        let ora_gl_sheet_name = matches
            .value_of("ora_gl_sheet_name")
            .expect("Error getting `ora_gl_sheet_name`.")
            .to_string();
        let master_llg_file_path = matches
            .value_of("master_llg_file_path")
            .expect("Error getting `master_llg_file_path`.")
            .to_string();
        let master_llg_sheet_name = matches
            .value_of("master_llg_sheet_name")
            .expect("Error getting `master_llg_sheet_name`.")
            .to_string();

        ConfigurationParameters {
            input_cf_file_path,
            metadata_file_path,
            req_fields_file_path,
            npa_class_file_path,
            finnone_master_file_path,
            common_cust_file_path,
            risk_weight_file_path,
            restructure_flag_file_path,
            resid_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            ora_gl_file_path,
            ora_gl_sheet_name,
            master_llg_file_path,
            master_llg_sheet_name,
        }
    }
}

// pub lic getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_cf_file_path(&self) -> &str {
        &self.input_cf_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn finnone_master_file_path(&self) -> &str {
        &self.finnone_master_file_path
    }
    pub fn npa_class_file_path(&self) -> &str {
        &self.npa_class_file_path
    }
    pub fn common_cust_file_path(&self) -> &str {
        &self.common_cust_file_path
    }
    pub fn risk_weight_file_path(&self) -> &str {
        &self.risk_weight_file_path
    }
    pub fn restructure_flag_file_path(&self) -> &str {
        &self.restructure_flag_file_path
    }
    pub fn resid_file_path(&self) -> &str {
        &self.resid_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn ora_gl_file_path(&self) -> &str {
        &self.ora_gl_file_path
    }
    pub fn master_llg_file_path(&self) -> &str {
        &self.master_llg_file_path
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
        .about("Cashflow generation for Term deposits!!")
        .version("1.0.4030")
        .author("srinivas644 <srinivas.r@surya-soft.com>")
        .arg(
            Arg::with_name("input_cf_file_path")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("metadata_file_path")
                .long("metadata-file-path")
                .value_name("Metadata File Path")
                .help("Path to the metadata file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file_path")
                .long("req-fields-file-path")
                .value_name("Required Fields File Path")
                .help("Path to the required fields file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_master_file_path")
                .long("finnone-master-file-path")
                .value_name("Finnone Master File Path")
                .help("Path to the finnone master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("risk_weight_file_path")
                .long("risk-weight-file-path")
                .value_name("Risk Weight File Path")
                .help("Path to the risk weight file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("resid_file_path")
                .long("resid-file-path")
                .value_name("Resid File Path")
                .help("Path to the resid file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("restructure_flag_file_path")
                .long("restructure-flag-file-path")
                .value_name("Restructure Flag File Path")
                .help("Path to the restructure flag file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("common_cust_file_path")
                .long("common-cust-file-path")
                .value_name("Common Cust File Path")
                .help("Path to the common cust file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_class_file_path")
                .long("npa-class-file-path")
                .value_name("NPA Class File Path")
                .help("Path to the npa cust file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file")
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
                .value_name("Diagnostics Log File")
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
            Arg::with_name("ora_gl_file_path")
                .long("ora-gl-file-path")
                .value_name("ORA GL File")
                .help("Path to the ORA GL file.")
                .required(true),
        )
        .arg(
            Arg::with_name("master_llg_file_path")
                .long("master-llg-file-path")
                .value_name("Master LLG File")
                .help("Path to the Master LLG file.")
                .required(true),
        )
        .arg(
            Arg::with_name("ora_gl_sheet_name")
                .long("ora-gl-sheet-name")
                .value_name("ORA GL Sheet Name")
                .help("Name of the ORA GL sheet.")
                .required(true),
        )
        .arg(
            Arg::with_name("master_llg_sheet_name")
                .long("master-llg-sheet-name")
                .value_name("Master LLG Sheet Name")
                .help("Name of the Master LLG sheet.")
                .required(true),
        )
        .get_matches()
}
