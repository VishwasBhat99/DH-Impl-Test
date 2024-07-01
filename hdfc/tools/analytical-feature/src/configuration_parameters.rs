use clap;
use clap::{App, Arg};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    retail_input_file: String,
    non_retail_input_file: String,
    output_file: String,
    edw_alm_customer_file: String,
    biu_file: String,
    master_prod_cust_type_file: String,
    master_prod_cust_type_file_sheet: String,
    runoff_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "retail_input_file: {}", self.retail_input_file());
        info!(
            logger,
            "non_retail_input_file: {}",
            self.non_retail_input_file()
        );
        info!(logger, "output_file: {}", self.output_file());
        info!(
            logger,
            "edw_alm_customer_file: {}",
            self.edw_alm_customer_file()
        );
        info!(logger, "biu_file: {}", self.biu_file());
        info!(
            logger,
            "master_prod_cust_type_file: {}",
            self.master_prod_cust_type_file()
        );
        info!(
            logger,
            "master_prod_cust_type_file_sheet: {}",
            self.master_prod_cust_type_file_sheet()
        );
        info!(logger, "runoff_file: {}", self.runoff_file());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(
            logger,
            "diagnostics_log_file: {}",
            self.diagnostics_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let retail_input_file = matches
            .value_of("retail_input_file")
            .expect("Error getting `retail_input_file`.")
            .to_string();
        let non_retail_input_file = matches
            .value_of("non_retail_input_file")
            .expect("Error getting `non_retail_input_file`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let edw_alm_customer_file = matches
            .value_of("edw_alm_customer_file")
            .expect("Error getting `edw_alm_customer_file`.")
            .to_string();
        let biu_file = matches
            .value_of("biu_file")
            .expect("Error getting `biu_file`.")
            .to_string();
        let master_prod_cust_type_file = matches
            .value_of("master_prod_cust_type_file")
            .expect("Error getting `master_prod_cust_type_file`.")
            .to_string();
        let master_prod_cust_type_file_sheet = matches
            .value_of("master_prod_cust_type_file_sheet")
            .expect("Error getting `master_prod_cust_type_file_sheet`.")
            .to_string();
        let runoff_file = matches
            .value_of("runoff_file")
            .expect("Error getting `runoff_file`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            retail_input_file,
            non_retail_input_file,
            output_file,
            edw_alm_customer_file,
            biu_file,
            master_prod_cust_type_file,
            master_prod_cust_type_file_sheet,
            runoff_file,
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
    pub fn retail_input_file(&self) -> &str {
        &self.retail_input_file
    }
    pub fn non_retail_input_file(&self) -> &str {
        &self.non_retail_input_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn edw_alm_customer_file(&self) -> &str {
        &self.edw_alm_customer_file
    }
    pub fn biu_file(&self) -> &str {
        &self.biu_file
    }
    pub fn master_prod_cust_type_file(&self) -> &str {
        &self.master_prod_cust_type_file
    }
    pub fn master_prod_cust_type_file_sheet(&self) -> &str {
        &self.master_prod_cust_type_file_sheet
    }
    pub fn runoff_file(&self) -> &str {
        &self.runoff_file
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
        .about("a dashboard to view the details of a particular customer")
        .arg(
            Arg::with_name("retail_input_file")
                .long("retail_input_file")
                .value_name("retail_input_file_path")
                .help("retail input file path")
                .required(true)
        )
        .arg(
            Arg::with_name("non_retail_input_file")
                .long("non_retail_input_file")
                .value_name("non_retail_input_file_path")
                .help("non retail input file path")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output_file")
                .value_name("output_file_path")
                .help("output file path")
                .required(true)
        )
        .arg(
            Arg::with_name("edw_alm_customer_file")
                .long("edw_alm_customer_file")
                .value_name("edw_alm_customer_file_path")
                .help("edw alm customer file")
                .required(true)
        )
        .arg(
            Arg::with_name("biu_file")
                .long("biu_file")
                .value_name("biu_file_path")
                .help("biu file path")
                .required(true)
        )
        .arg(
            Arg::with_name("master_prod_cust_type_file")
                .long("master_prod_cust_type_file")
                .value_name("master_prod_cust_type_file_path")
                .help("master prod cust type file path")
                .required(true)
        )
        .arg(
            Arg::with_name("master_prod_cust_type_file_sheet")
                .long("master_prod_cust_type_file_sheet")
                .value_name("master_prod_cust_type_file_sheet_name")
                .help("master prod cust type file sheet name")
                .required(true)
        )
        .arg(
            Arg::with_name("runoff_file")
                .long("runoff_file")
                .value_name("runoff_file_path")
                .help("runoff file path")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log_file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics_log_file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log_level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics_flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
