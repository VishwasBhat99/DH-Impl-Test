use clap;
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
    pub reference_file_path_1: String,
    pub reference_file_path_2: String,
    pub ref2_sheet_name: String,
    pub asset_types: String,
    pub reference_file_path_3: String,
    pub ta_config_file: String,
    pub dlod_cashflow_file: String,
    pub dlod_cashflow_file_2: String,
    pub odfd_cashflow_file: String,
    pub rtl_cashflow_file: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub reference_1_delim: String,
    pub reference_2_delim: String,
    pub reference_3_delim: String,
    pub ta_config_delim: String,
    pub dlod_delim: String,
    pub dlod_date_format: String,
    pub dlod_2_delim: String,
    pub dlod_2_date_format: String,
    pub odfd_delim: String,
    pub rtl_delim: String,
    pub rtl_date_format: String,
    pub crm_master_file_path: String,
    pub cust_entity_master_file_path: String,
    pub crm_file_delim: String,
    pub cust_entity_delim: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "reference_file_1: {}", self.reference_file_path_1());
        info!(logger, "reference_file_2: {}", self.reference_file_path_2());
        info!(logger, "ref2_sheet_name: {}", self.ref2_sheet_name());
        info!(logger, "asset_types: {}", self.asset_types());
        info!(logger, "reference_file_3: {}", self.reference_file_path_3());
        info!(logger, "dlod_cashflow_file: {}", self.dlod_cashflow_file());
        info!(logger, "rtl_cashflow_file: {}", self.rtl_cashflow_file());
        info!(
            logger,
            "dlod_cashflow_file_2: {}",
            self.dlod_cashflow_file_2()
        );
        info!(logger, "odfd_cashflow_file: {}", self.odfd_cashflow_file());
        info!(logger, "ta_config_file: {}", self.ta_config_file());
        info!(logger, "reference_1_delim: {}", self.reference_1_delim());
        info!(logger, "reference_2_delim: {}", self.reference_2_delim());
        info!(logger, "reference_3_delim: {}", self.reference_3_delim());
        info!(logger, "ta_config_delim: {}", self.ta_config_delim());
        info!(logger, "dlod_delim: {}", self.dlod_delim());
        info!(logger, "dlod_2_delim: {}", self.dlod_2_delim());
        info!(logger, "odfd_delim: {}", self.odfd_delim());
        info!(logger, "rtl_delim: {}", self.rtl_delim());
        info!(logger, "crm_file_delim: {}", self.crm_file_delim());
        info!(logger, "cust_entity_delim: {}", self.cust_entity_delim());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "crm_master_file_path: {}",
            self.crm_master_file_path()
        );
        info!(
            logger,
            "cust_entity_master_file_path: {}",
            self.cust_entity_master_file_path()
        );
        info!(logger, "dlod_date_format: {}", self.dlod_date_format());
        info!(logger, "dlod_2_date_format: {}", self.dlod_2_date_format());
        info!(logger, "rtl_date_format: {}", self.rtl_date_format());
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
        let cust_entity_master_file_path = matches
            .value_of("cust_entity_master_file_path")
            .expect("Error getting `cust_entity_master_file_path` value.")
            .to_string();
        let crm_master_file_path = matches
            .value_of("crm_master_file_path")
            .expect("Error getting `crm_master_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();

        let reference_file_path_1 = matches
            .value_of("reference_file_1")
            .expect("Error getting `reference_file_1` value.")
            .to_string();

        let reference_file_path_2 = matches
            .value_of("reference_file_2")
            .expect("Error getting `reference_file_2` value.")
            .to_string();
        let ref2_sheet_name = matches
            .value_of("ref2_sheet_name")
            .expect("Error getting `ref2_sheet_name` value.")
            .to_string();
        let asset_types = matches
            .value_of("asset_types")
            .expect("Error getting `asset_types` value.")
            .to_string();
        let reference_file_path_3 = matches
            .value_of("reference_file_3")
            .expect("Error getting `reference_file_3` value.")
            .to_string();
        let dlod_cashflow_file = matches
            .value_of("dlod_cashflow_file")
            .expect("Error getting `dlod_cashflow_file` value.")
            .to_string();
        let dlod_cashflow_file_2 = matches
            .value_of("dlod_cashflow_file_2")
            .expect("Error getting `dlod_cashflow_file_2` value.")
            .to_string();
        let rtl_cashflow_file = matches
            .value_of("rtl_cashflow_file")
            .expect("Error getting `rtl_cashflow_file` value.")
            .to_string();
        let odfd_cashflow_file = matches
            .value_of("odfd_cashflow_file")
            .expect("Error getting `odfd_cashflow_file` value.")
            .to_string();
        let ta_config_file = matches
            .value_of("ta_config_file")
            .expect("Error getting `ta_config_file` value.")
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
        let reference_1_delim = matches
            .value_of("reference_1_delim")
            .expect("Error getting `reference_1_delim` value.")
            .to_string();
        let reference_2_delim = matches
            .value_of("reference_2_delim")
            .expect("Error getting `reference_2_delim` value.")
            .to_string();
        let reference_3_delim = matches
            .value_of("reference_3_delim")
            .expect("Error getting `reference_3_delim` value.")
            .to_string();
        let ta_config_delim = matches
            .value_of("ta_config_delim")
            .expect("Error getting `ta_config_delim` value.")
            .to_string();
        let dlod_delim = matches
            .value_of("dlod_delim")
            .expect("Error getting `dlod_delim` value.")
            .to_string();
        let dlod_2_delim = matches
            .value_of("dlod_2_delim")
            .expect("Error getting `dlod_2_delim` value.")
            .to_string();
        let odfd_delim = matches
            .value_of("odfd_delim")
            .expect("Error getting `odfd_delim` value.")
            .to_string();
        let rtl_delim = matches
            .value_of("rtl_delim")
            .expect("Error getting `rtl_delim` value.")
            .to_string();
        let crm_file_delim = matches
            .value_of("crm_file_delim")
            .expect("Error getting `crm_file_delim` value.")
            .to_string();
        let cust_entity_delim = matches
            .value_of("cust_entity_delim")
            .expect("Error getting `cust_entity_delim` value.")
            .to_string();
        let dlod_date_format = matches
            .value_of("dlod_date_format")
            .expect("Error getting `dlod_date_format` value.")
            .to_string();
        let dlod_2_date_format = matches
            .value_of("dlod_2_date_format")
            .expect("Error getting `dlod_2_date_format` value.")
            .to_string();
        let rtl_date_format = matches
            .value_of("rtl_date_format")
            .expect("Error getting `rtl_date_format` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            as_on_date,
            output_file_path,
            reference_file_path_1,
            reference_file_path_2,
            ref2_sheet_name,
            asset_types,
            reference_file_path_3,
            dlod_cashflow_file,
            rtl_cashflow_file,
            dlod_cashflow_file_2,
            odfd_cashflow_file,
            ta_config_file,
            log_file_path,
            diagnostics_file_path,
            log_level,
            cust_entity_master_file_path,
            crm_master_file_path,
            is_perf_diagnostics_enabled,
            reference_1_delim,
            reference_2_delim,
            reference_3_delim,
            ta_config_delim,
            dlod_delim,
            dlod_2_delim,
            odfd_delim,
            rtl_delim,
            dlod_date_format,
            dlod_2_date_format,
            rtl_date_format,
            crm_file_delim,
            cust_entity_delim,
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
    pub fn reference_file_path_1(&self) -> &str {
        &self.reference_file_path_1
    }
    pub fn reference_file_path_2(&self) -> &str {
        &self.reference_file_path_2
    }
    pub fn ref2_sheet_name(&self) -> &str {
        &self.ref2_sheet_name
    }
    pub fn asset_types(&self) -> &str {
        &self.asset_types
    }
    pub fn reference_file_path_3(&self) -> &str {
        &self.reference_file_path_3
    }
    pub fn dlod_cashflow_file(&self) -> &str {
        &self.dlod_cashflow_file
    }
    pub fn dlod_cashflow_file_2(&self) -> &str {
        &self.dlod_cashflow_file_2
    }
    pub fn odfd_cashflow_file(&self) -> &str {
        &self.odfd_cashflow_file
    }
    pub fn rtl_cashflow_file(&self) -> &str {
        &self.rtl_cashflow_file
    }
    pub fn ta_config_file(&self) -> &str {
        &self.ta_config_file
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
    pub fn reference_1_delim(&self) -> &str {
        &self.reference_1_delim
    }
    pub fn reference_2_delim(&self) -> &str {
        &self.reference_2_delim
    }
    pub fn reference_3_delim(&self) -> &str {
        &self.reference_3_delim
    }
    pub fn ta_config_delim(&self) -> &str {
        &self.ta_config_delim
    }
    pub fn dlod_delim(&self) -> &str {
        &self.dlod_delim
    }
    pub fn dlod_2_delim(&self) -> &str {
        &self.dlod_2_delim
    }
    pub fn odfd_delim(&self) -> &str {
        &self.odfd_delim
    }
    pub fn rtl_delim(&self) -> &str {
        &self.rtl_delim
    }
    pub fn crm_file_delim(&self) -> &str {
        &self.crm_file_delim
    }
    pub fn cust_entity_delim(&self) -> &str {
        &self.cust_entity_delim
    }
    pub fn cust_entity_master_file_path(&self) -> &str {
        &self.cust_entity_master_file_path
    }
    pub fn crm_master_file_path(&self) -> &str {
        &self.crm_master_file_path
    }
    pub fn dlod_date_format(&self) -> &str {
        &self.dlod_date_format
    }
    pub fn dlod_2_date_format(&self) -> &str {
        &self.dlod_2_date_format
    }
    pub fn rtl_date_format(&self) -> &str {
        &self.rtl_date_format
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates pre-processor output for DLOD")
        .version("1.0.4101")
        .author("harsh8501 <harsh.sk@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("reference_file_1")
                .long("reference-file-1")
                .value_name("REFERENCE_FILE_1")
                .help("Path to the Reference File 1.")
                .required(true)
        )
        .arg(
            Arg::with_name("reference_file_2")
                .long("reference-file-2")
                .value_name("REFERENCE_FILE_2")
                .help("Path to the Reference File 2.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref2_sheet_name")
                .long("ref2-sheet-name")
                .value_name("REF2_SHEET_NAME")
                .help("Sheet Name in reference file 2")
                .required(false)
                .default_value("Sheet1")
        )
        .arg(
            Arg::with_name("asset_types")
                .long("asset-type")
                .value_name("ASSET TYPE")
                .help("Asset type to consider from reference file file")
                .required(true)
        )
        .arg(
            Arg::with_name("reference_file_3")
                .long("reference-file-3")
                .value_name("REFERENCE_FILE_3")
                .help("Path to the Reference File 3.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_cashflow_file")
                .long("dlod-cashflow-file")
                .value_name("dlod_cashflow_file")
                .help("Path to the DLOD cashflow file.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_cashflow_file_2")
                .long("dlod-cashflow-file-2")
                .value_name("dlod_cashflow_file_2")
                .help("Path to the DLOD cashflow file 2.")
                .required(true)
        )
        .arg(
            Arg::with_name("odfd_cashflow_file")
                .long("odfd-cashflow-file")
                .value_name("odfd_cashflow_file")
                .help("Path to the ODFD cashflow file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rtl_cashflow_file")
                .long("rtl-cashflow-file")
                .value_name("rtl_cashflow_file")
                .help("Path to the RTL cashflow file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ta_config_file")
                .long("ta-config-file")
                .value_name("ta_config_file")
                .help("Path to the TA Config File.")
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
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics log.")
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
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("reference_1_delim")
                .long("reference-1-delim")
                .value_name("Reference File 1 Delimiter")
                .help("Delimiter of reference file 1.")
                .required(true)
        )
        .arg(
            Arg::with_name("reference_2_delim")
                .long("reference-2-delim")
                .value_name("Reference File 2 Delimiter")
                .help("Delimiter of reference file 2.")
                .required(true)
        )
        .arg(
            Arg::with_name("reference_3_delim")
                .long("reference-3-delim")
                .value_name("Reference File 3 Delimiter")
                .help("Delimiter of reference file 3.")
                .required(true)
        )
        .arg(
            Arg::with_name("ta_config_delim")
                .long("ta-config-delim")
                .value_name("TA Config file delimiter")
                .help("Delimiter of TA Config file.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_delim")
                .long("dlod-delim")
                .value_name("DLOD Delim")
                .help("Delimiter of DLOD file.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_2_delim")
                .long("dlod-2-delim")
                .value_name("DLOD 2 Delim")
                .help("Delimiter of DLOD 2 file.")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_date_format")
                .long("dlod-date-format")
                .value_name("DLOD date Format")
                .help("Date format of DLOD file.")
                .default_value("dd-mm-yyyy")
                .required(true)
        )
        .arg(
            Arg::with_name("dlod_2_date_format")
                .long("dlod-2-date-format")
                .value_name("DLOD 2 Date Format")
                .help("Date format of DLOD 2 file.")
                .default_value("dd-mm-yyyy")
                .required(true)
        )
        .arg(
            Arg::with_name("odfd_delim")
                .long("odfd-delim")
                .value_name("ODFD Delim")
                .help("Delimiter of ODFD file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_entity_master_file_path")
                .long("cust-entity-master-file")
                .value_name("cust_entity_master_file_path")
                .help("Path to Cust Entity Master file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("crm_master_file_path")
                .long("crm-master-file")
                .value_name("crm_master_file_path")
                .help("Path to CRM Master file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("rtl_delim")
                .long("rtl-delim")
                .value_name("RTL Delimiter")
                .help("Delimiter of RTL file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rtl_date_format")
                .long("rtl-date-format")
                .value_name("RTL Date Format")
                .help("Date format of RTL file.")
                .default_value("dd-mm-yyyy")
                .required(true)
        )
        .arg(
            Arg::with_name("crm_file_delim")
                .long("crm-file-delim")
                .value_name("crm-file Delim")
                .help("Delimiter of crm_file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_entity_delim")
                .long("cust-entity-delim")
                .value_name("cust-entity Delim")
                .help("Delimiter of Cust Entity file.")
                .required(true)
        )
        .get_matches()
}
