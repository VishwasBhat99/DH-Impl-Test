use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    slabs_file_path: String,
    cust_master_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    cust_file_path: String,
    cust_id_file_path: String,
    biu_file_path: String,
    class_file_path: String,
    cust_master_sep: String,
    cust_sep: String,
    cust_id_sep: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    amount_type: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "slabs_file: {}", self.slabs_file_path());
        info!(logger, "cust_file_path: {}", self.cust_file_path());
        info!(logger, "cust_id_file_path: {}", self.cust_id_file_path());
        info!(logger, "biu_file_path: {}", self.biu_file_path());
        info!(logger, "class_file_path: {}", self.class_file_path());
        info!(logger, "cust_master_sep: {}", self.cust_master_sep());
        info!(logger, "cust_sep: {}", self.cust_sep());
        info!(logger, "cust_id_sep: {}", self.cust_id_sep());
        info!(logger, "amount_type: {}", self.amount_type());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "cust_master_file: {}", self.cust_master_file_path());
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

        let slabs_file_path = matches
            .value_of("slabs_file")
            .expect("Error getting `slabs_file_value`.")
            .to_string();

        let cust_master_file_path = matches
            .value_of("cust_master_file")
            .expect("Error getting `cust_master_file_value`.")
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
        let cust_master_sep = matches
            .value_of("cust_master_sep")
            .expect("Error getting `cust_master_sep`.")
            .to_string();
        let cust_sep = matches
            .value_of("cust_sep")
            .expect("Error getting `cust_sep`.")
            .to_string();
        let cust_id_sep = matches
            .value_of("cust_id_sep")
            .expect("Error getting `cust_id_sep`.")
            .to_string();
        let amount_type = matches
            .value_of("amount_type")
            .expect("Error getting `amount_type`.")
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
            slabs_file_path,
            cust_master_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            cust_file_path,
            cust_id_file_path,
            biu_file_path,
            class_file_path,
            cust_master_sep,
            cust_sep,
            amount_type,
            cust_id_sep,
            as_on_date,
            output_file_path,
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
    pub fn slabs_file_path(&self) -> &str {
        &self.slabs_file_path
    }
    pub fn cust_master_file_path(&self) -> &str {
        &self.cust_master_file_path
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
    pub fn amount_type(&self) -> &str {
        &self.amount_type
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
    pub fn cust_master_sep(&self) -> &str {
        &self.cust_master_sep
    }
    pub fn cust_sep(&self) -> &str {
        &self.cust_sep
    }
    pub fn cust_id_sep(&self) -> &str {
        &self.cust_id_sep
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.0.3618")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("BIU Required fields stamper.")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file.")
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
            Arg::with_name("cust_master_sep")
                .long("cust-master-sep")
                .value_name("Customer Master Seperator")
                .help("Customer Master Seperator.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_sep")
                .long("cust-sep")
                .value_name("Customer Seperator")
                .help("Customer Seperator.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_id_sep")
                .long("cust-id-sep")
                .value_name("Customer ID Seperator")
                .help("Customer ID Seperator.")
                .required(true)
        )
        .arg(
            Arg::with_name("amount_type")
                .long("amount-type")
                .value_name("Amount Type Flag")
                .help("Interest/Principal Amount")
                .required(false)
                .possible_values(&["PRIN","INT"])
                .default_value("PRIN")
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
