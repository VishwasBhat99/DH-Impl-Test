use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    ea_master_file_path: String,
    slabs_file_path: String,
    cust_master_file_path: String,
    rw_master_file_path: String,
    restructured_flag_file_path: String,
    residential_mortgage_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    cust_file_path: String,
    cust_id_file_path: String,
    biu_file_path: String,
    class_file_path: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    has_cashflows: bool,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "ea_master_file: {}", self.ea_master_file_path());
        info!(logger, "slabs_file: {}", self.slabs_file_path());
        info!(logger, "cust_file_path: {}", self.cust_file_path());
        info!(logger, "cust_id_file_path: {}", self.cust_id_file_path());
        info!(logger, "biu_file_path: {}", self.biu_file_path());
        info!(logger, "class_file_path: {}", self.class_file_path());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "has_cashflows: {}", self.has_cashflows());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "cust_master_file: {}", self.cust_master_file_path());
        info!(logger, "rw_master_file: {}", self.rw_master_file_path());
        info!(
            logger,
            "restructured_flag_file_path: {}",
            self.restructured_flag_file_path()
        );
        info!(
            logger,
            "residential_mortgage_file_path: {}",
            self.residential_mortgage_file_path()
        );
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_value`.")
            .to_string();
        let ea_master_file_path = matches
            .value_of("ea_master_file")
            .expect("Error getting `ea_master_file_value`.")
            .to_string();
        let slabs_file_path = matches
            .value_of("slabs_file")
            .expect("Error getting `slabs_file_value`.")
            .to_string();

        let cust_master_file_path = matches
            .value_of("cust_master_file")
            .expect("Error getting `cust_master_file_value`.")
            .to_string();

        let rw_master_file_path = matches
            .value_of("rw_master_file")
            .expect("Error getting `rw_master_file_value`.")
            .to_string();

        let restructured_flag_file_path = matches
            .value_of("restructured_flag_file_path")
            .expect("Error getting `restructured_flag_file_path`.")
            .to_string();

        let residential_mortgage_file_path = matches
            .value_of("residential_mortgage_file_path")
            .expect("Error getting `residential_mortgage_file_path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `Req fields file path`.")
            .to_string();

        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `Account metadata file path`.")
            .to_string();

        let cust_file_path = matches
            .value_of("cust_file_path")
            .expect("Error getting `cust_file_path`.")
            .to_string();

        let cust_id_file_path = matches
            .value_of("cust_id_file_path")
            .expect("Error getting `cust_id_file_path`.")
            .to_string();

        let biu_file_path = matches
            .value_of("biu_file_path")
            .expect("Error getting `biu_file_path`.")
            .to_string();

        let class_file_path = matches
            .value_of("class_file_path")
            .expect("Error getting `class_file_path`.")
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
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();

        let has_cashflows = matches
            .value_of("has_cashflows")
            .expect("Error getting has_cashflows as true/false`.")
            .parse::<bool>()
            .expect("Cannot parse `has_cashflows` as bool.");

        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            ea_master_file_path,
            slabs_file_path,
            cust_master_file_path,
            rw_master_file_path,
            restructured_flag_file_path,
            residential_mortgage_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            cust_file_path,
            cust_id_file_path,
            biu_file_path,
            class_file_path,
            as_on_date,
            output_file_path,
            has_cashflows,
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
    pub fn ea_master_file_path(&self) -> &str {
        &self.ea_master_file_path
    }
    pub fn slabs_file_path(&self) -> &str {
        &self.slabs_file_path
    }
    pub fn cust_master_file_path(&self) -> &str {
        &self.cust_master_file_path
    }
    pub fn rw_master_file_path(&self) -> &str {
        &self.rw_master_file_path
    }
    pub fn restructured_flag_file_path(&self) -> &str {
        &self.restructured_flag_file_path
    }
    pub fn residential_mortgage_file_path(&self) -> &str {
        &self.residential_mortgage_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn account_metadata_file_path(&self) -> &str {
        &self.account_metadata_file_path
    }
    pub fn cust_file_path(&self) -> &str {
        &self.cust_file_path
    }
    pub fn cust_id_file_path(&self) -> &str {
        &self.cust_id_file_path
    }
    pub fn biu_file_path(&self) -> &str {
        &self.biu_file_path
    }
    pub fn class_file_path(&self) -> &str {
        &self.class_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn has_cashflows(&self) -> bool {
        self.has_cashflows
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
        .version("1.0.4991")
        .author("ravindar-01<ravindar.sr@surya-soft.com>")
        .about("CF T1 NSFR  ADV Program.")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ea_master_file")
                .long("ea-master-file")
                .value_name("EA Master File")
                .help("Path to Ea Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("slabs_file")
                .long("slabs-file")
                .value_name("Slabs File")
                .help("Path to slabs file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_master_file")
                .long("cust-master-file")
                .value_name("CUST MASTER File")
                .help("Path to cust master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rw_master_file")
                .long("rw-master-file")
                .value_name("RW MASTER File")
                .help("Path to rw master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("restructured_flag_file_path")
                .long("restructured-flag-file-path")
                .value_name("restructured flag file path")
                .help("Path to restructured flag file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("residential_mortgage_file_path")
                .long("residential-mortgage-file-path")
                .value_name("residential mortgage file path")
                .help("Path to residential mortgage file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("req-fields-file")
                .value_name("REQ_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_file_path")
                .long("cust-file")
                .value_name("Customer File Path")
                .help("Customer File Path")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_id_file_path")
                .long("cust-id-file")
                .value_name("Cust Id File Path")
                .help("Cust Id File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("biu_file_path")
                .long("biu-file")
                .value_name("BIU File Path")
                .help("BIU File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("class_file_path")
                .long("class-file")
                .value_name("Class File Path")
                .help("Class File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("has_cashflows")
                .long("has-cashflows")
                .value_name("HAS CASHFLOWS")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether program is a maturity or a non matiruty based.")
                .default_value("true")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
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
        .get_matches()
}
