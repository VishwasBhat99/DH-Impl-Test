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
    pub ltv_file_path: String,
    pub extra_fields_file_path: String,
    pub alm_master: String,
    pub alm_master_sheet_name: String,
    pub care_cust_mapping_file_path: String,
    pub care_cust_mapping_sheet_name: String,
    pub care_acc_mapping_file_path: String,
    pub care_acc_mapping_sheet_name: String,
    pub cust_master: String,
    pub npa_file_path: String,
    pub npa_sheet_name: String,
    pub schedule_file_path: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub concat_file_path: String,
    pub rec_output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub loan_additional_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "ltv_file: {}", self.ltv_file_path());
        info!(
            logger,
            "extra_fields_file: {}",
            self.extra_fields_file_path()
        );
        info!(logger, "alm_master: {}", self.alm_master());
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(
            logger,
            "care_cust_mapping_file_path: {}",
            self.care_cust_mapping_file_path()
        );
        info!(
            logger,
            "care_cust_mapping_sheet_name: {}",
            self.care_cust_mapping_sheet_name()
        );
        info!(
            logger,
            "care_acc_mapping_file_path: {}",
            self.care_acc_mapping_file_path()
        );
        info!(
            logger,
            "care_acc_mapping_sheet_name: {}",
            self.care_acc_mapping_sheet_name()
        );
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "concat_file: {}", self.concat_file_path());
        info!(logger, "cust_master: {}", self.cust_master());
        info!(logger, "npa_file_path: {}", self.npa_file_path());
        info!(logger, "npa_sheet_name: {}", self.npa_sheet_name());
        info!(logger, "schedule_file_path: {}", self.schedule_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "rec_output_file: {}", self.rec_output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "loan_additional_file_path: {}",
            self.loan_additional_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();

        let ltv_file_path = matches
            .value_of("ltv_file")
            .expect("Error getting `ltv_file` value.")
            .to_string();

        let extra_fields_file_path = matches
            .value_of("extra_fields_file_path")
            .expect("Error getting `extra_fields_file_path` value.")
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
        let cust_master = matches
            .value_of("cust_master")
            .expect("Error getting `cust_master` value.")
            .to_string();
        let npa_file_path = matches
            .value_of("npa_file")
            .expect("Error getting `npa_file_path` value.")
            .to_string();
        let npa_sheet_name = matches
            .value_of("npa_sheet_name")
            .expect("Error getting `npa_sheet_name` value.")
            .to_string();
        let schedule_file_path = matches
            .value_of("schedule_file")
            .expect("Error getting `schedule_file_path` value.")
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
        let alm_master = matches
            .value_of("alm_master")
            .expect("Error getting `alm_master` value.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        let care_acc_mapping_file_path = matches
            .value_of("care_acc_mapping_file_path")
            .expect("Error getting `care_acc_mapping_file_path` value.")
            .to_string();
        let care_acc_mapping_sheet_name = matches
            .value_of("care_acc_mapping_sheet_name")
            .expect("Error getting `care_acc_mapping_sheet_name` value.")
            .to_string();
        let care_cust_mapping_file_path = matches
            .value_of("care_cust_mapping_file_path")
            .expect("Error getting `care_cust_mapping_file_path` value.")
            .to_string();
        let care_cust_mapping_sheet_name = matches
            .value_of("care_cust_mapping_sheet_name")
            .expect("Error getting `care_cust_mapping_sheet_name` value.")
            .to_string();
        let loan_additional_file_path = matches
            .value_of("loan_additional_file_path")
            .expect("Error getting `loan_additional_file_path` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            ltv_file_path,
            extra_fields_file_path,
            alm_master,
            alm_master_sheet_name,
            care_cust_mapping_file_path,
            care_cust_mapping_sheet_name,
            care_acc_mapping_file_path,
            care_acc_mapping_sheet_name,
            cust_master,
            npa_file_path,
            npa_sheet_name,
            schedule_file_path,
            as_on_date,
            output_file_path,
            concat_file_path,
            rec_output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            loan_additional_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn ltv_file_path(&self) -> &str {
        &self.ltv_file_path
    }
    pub fn extra_fields_file_path(&self) -> &str {
        &self.extra_fields_file_path
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
    pub fn cust_master(&self) -> &str {
        &self.cust_master
    }
    pub fn npa_file_path(&self) -> &str {
        &self.npa_file_path
    }
    pub fn npa_sheet_name(&self) -> &str {
        &self.npa_sheet_name
    }
    pub fn schedule_file_path(&self) -> &str {
        &self.schedule_file_path
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
    pub fn alm_master(&self) -> &str {
        &self.alm_master
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn care_cust_mapping_file_path(&self) -> &str {
        &self.care_cust_mapping_file_path
    }
    pub fn care_cust_mapping_sheet_name(&self) -> &str {
        &self.care_cust_mapping_sheet_name
    }
    pub fn care_acc_mapping_file_path(&self) -> &str {
        &self.care_acc_mapping_file_path
    }
    pub fn care_acc_mapping_sheet_name(&self) -> &str {
        &self.care_acc_mapping_sheet_name
    }
    pub fn loan_additional_file_path(&self) -> &str {
        &self.loan_additional_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of Loans CFGen!")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ltv_file")
                .long("ltv-file")
                .value_name("LTV File")
                .help("Path to LTV file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("extra_fields_file_path")
                .long("extra-fields-file-path")
                .value_name("Extra Fields File")
                .help("Path to extra fields file that needs to be processed.")
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
                .value_name("Concat File")
                .help("Path to the concat file for the gls which are not present in master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_master")
                .long("cust-master")
                .value_name("Murex Master File")
                .help("Path to the Murex Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_file")
                .long("npa-file")
                .value_name("NPA File")
                .help("Path to the NPA file.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_sheet_name")
                .long("npa-sheet-name")
                .value_name("NPA Sheet Name")
                .help("NPA File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("schedule_file")
                .long("schedule-file")
                .value_name("Schedule File")
                .help("Path to the Schedule file.")
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
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
                .help("Diagnostics log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("Log Level")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("Diagnostics Flag")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("Date")
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
            Arg::with_name("care_cust_mapping_file_path")
                .long("care-cust-mapping-file-path")
                .value_name("care_cust_mapping_file_path")
                .help("Care cust mapping file path.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("care_cust_mapping_sheet_name")
                .long("care-cust-mapping-sheet-name")
                .value_name("care_cust_mapping_sheet_name")
                .help("Care cust mapping sheet name.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("care_acc_mapping_file_path")
                .long("care-acc-mapping-file-path")
                .value_name("care_acc_mapping_file_path")
                .help("Care acc mapping file path.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("care_acc_mapping_sheet_name")
                .long("care-acc-mapping-sheet-name")
                .value_name("care_acc_mapping_sheet_name")
                .help("Care acc mapping sheet name.")
                .default_value("NA")
                .required(false)
        )
        .arg(
            Arg::with_name("loan_additional_file_path")
                .long("loan-additional-file-path")
                .value_name("loan_additional_file_path")
                .help("Loan Additional file path.")
                .required(true)
        )
        .get_matches()
}
