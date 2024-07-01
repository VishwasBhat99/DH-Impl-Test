use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);

    let parameters = ConfigurationParameters::new_from_matches(matches);
    return parameters;
}

pub struct ConfigurationParameters {
    input_file_path: String,
    ref_file: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    src_local_ccy: String,
    display_local_ccy: String,
    consol_ccy: String,
    foreign_consol_ccy: String,
    currency_conversion_file_path: String,
    req_fields_file_path: String,
    account_metadata_file_path: String,
    rules_file_path: String,
    default_llg_code: i32,
    default_overdue_llg_code: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    is_account_level_exchange_rate: bool,
    tenor_file_path: String,
    tenor_flag: bool,
    is_consolidated: bool,
    npa_flag_values: String,
    def_tenor: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "npa_flag_values: {}", self.npa_flag_values());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "src_local_ccy: {}", self.src_local_ccy());
        info!(logger, "display_local_ccy: {}", self.display_local_ccy());
        info!(logger, "consol_ccy: {}", self.consol_ccy());
        info!(logger, "foreign_consol_ccy: {}", self.foreign_consol_ccy());
        info!(
            logger,
            "account_metadata_file_path: {}",
            self.account_metadata_file_path()
        );
        info!(logger, "rules_file_path: {}", self.rules_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "ref_file: {}", self.ref_file());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "default_llg_code: {}", self.default_llg_code());
        info!(
            logger,
            "default_overdue_llg_code: {}",
            self.default_overdue_llg_code()
        );
        info!(
            logger,
            "is_account_level_exchange_rate: {}",
            self.is_account_level_exchange_rate()
        );
        info!(logger, "tenor_file_path: {}", self.tenor_file_path());
        info!(logger, "tenor_flag: {}", self.tenor_flag());
        info!(logger, "def_tenor: {}", self.def_tenor());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        // TODO: `unwrap()`s need proper error messages.
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `Input file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `Output file path`.")
            .to_string();
        let src_local_ccy = matches
            .value_of("src_local_ccy")
            .expect("Error getting `src local ccy`.")
            .to_string();
        let display_local_ccy = matches
            .value_of("display_local_ccy")
            .expect("Error getting `display local ccy`.")
            .to_string();
        let consol_ccy = matches
            .value_of("consol_ccy")
            .expect("Error getting `consol ccy`.")
            .to_string();
        let foreign_consol_ccy = matches
            .value_of("foreign_consol_ccy")
            .expect("Error getting `foreign consol ccy`.")
            .to_string();
        let ref_file = matches
            .value_of("ref_file")
            .expect("Error getting `Reference file`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `General log file path`.")
            .to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `Req fields file path`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `File level exchange rate file path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `Diagnostics log file path`.")
            .to_string();
        let account_metadata_file_path = matches
            .value_of("account_metadata_file_path")
            .expect("Error getting `Account metadata file path`.")
            .to_string();
        let rules_file_path = matches.value_of("rules_file_path").unwrap().to_string();
        let default_llg_code = matches
            .value_of("default_llg_code")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let default_overdue_llg_code = matches
            .value_of("default_overdue_llg_code")
            .unwrap_or("0")
            .to_string()
            .parse::<i32>()
            .unwrap();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let is_account_level_exchange_rate = matches
            .value_of("account_level_exchange_rate")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let tenor_file_path = matches
            .value_of("tenor_file_path")
            .expect("Error getting `Tenor file path`.")
            .to_string();
        let tenor_flag = matches
            .value_of("tenor_flag")
            .expect("Error getting `Tenor flag`.")
            .parse::<bool>()
            .unwrap();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `Is consolidated flag`.")
            .parse::<bool>()
            .unwrap();
        let npa_flag_values = matches
            .value_of("npa_flag_values")
            .expect("Error getting `npa flag values`.")
            .to_string();
        let def_tenor = matches
            .value_of("def_tenor")
            .expect("Error getting `default tenor`.")
            .to_string();
        ConfigurationParameters {
            input_file_path,
            ref_file,
            output_file_path,
            as_on_date,
            src_local_ccy,
            display_local_ccy,
            consol_ccy,
            foreign_consol_ccy,
            currency_conversion_file_path,
            req_fields_file_path,
            account_metadata_file_path,
            rules_file_path,
            default_llg_code,
            default_overdue_llg_code,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            is_account_level_exchange_rate,
            tenor_file_path,
            tenor_flag,
            is_consolidated,
            npa_flag_values,
            def_tenor,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        return &self.input_file_path;
    }
    pub fn ref_file(&self) -> &str {
        return &self.ref_file;
    }
    pub fn output_file_path(&self) -> &str {
        return &self.output_file_path;
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        return &self.as_on_date;
    }
    pub fn src_local_ccy(&self) -> &str {
        return &self.src_local_ccy;
    }
    pub fn display_local_ccy(&self) -> &str {
        return &self.display_local_ccy;
    }
    pub fn consol_ccy(&self) -> &str {
        return &self.consol_ccy;
    }
    pub fn foreign_consol_ccy(&self) -> &str {
        return &self.foreign_consol_ccy;
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        return &self.currency_conversion_file_path;
    }
    pub fn log_file_path(&self) -> &str {
        return &self.log_file_path;
    }
    pub fn req_fields_file_path(&self) -> &str {
        return &self.req_fields_file_path;
    }
    pub fn account_metadata_file_path(&self) -> &str {
        return &self.account_metadata_file_path;
    }
    pub fn rules_file_path(&self) -> &str {
        return &self.rules_file_path;
    }
    pub fn diagnostics_file_path(&self) -> &str {
        return &self.diagnostics_file_path;
    }
    pub fn log_level(&self) -> &str {
        return &self.log_level;
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        return self.is_perf_diagnostics_enabled;
    }
    pub fn is_account_level_exchange_rate(&self) -> bool {
        return self.is_account_level_exchange_rate;
    }
    pub fn default_llg_code(&self) -> i32 {
        return self.default_llg_code;
    }
    pub fn default_overdue_llg_code(&self) -> i32 {
        return self.default_overdue_llg_code;
    }
    pub fn tenor_file_path(&self) -> &str {
        return &self.tenor_file_path;
    }
    pub fn tenor_flag(&self) -> bool {
        return self.tenor_flag;
    }
    pub fn is_consolidated(&self) -> bool {
        return self.is_consolidated;
    }
    pub fn npa_flag_values(&self) -> &str {
        &self.npa_flag_values
    }
    pub fn def_tenor(&self) -> &str {
        &self.def_tenor
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app helps aggregate data for floating point prodcts!")
        .version("0.1.4319")
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file")
                .short("i")
                .long("ref-file")
                .value_name("REF")
                .help("Path to reference file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .short("e")     // 'l', 'o' taken; 'g' doesn't make sense; 'v' stands for 'verbose'
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("account_level_exchange_rate")
                .long("account-level-exchange-rate")
                .value_name("Exchange Rate Flag")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether exchange rate will be taken from file or account level.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .short("E")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .short("R")
                .long("req-fields-file")
                .value_name("REQ_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
                .required(true)
        )
        .arg(
            Arg::with_name("account_metadata_file_path")
                .short("m")
                .long("account-metadata-file")
                .value_name("ACCOUNT_METADATA")
                .help("The aggregator requires account metadata.\nThis parameter is a path to a json file that represents that metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("rules_file_path")
                .short("r")
                .long("rules-file-path")
                .value_name("RULES-FILE-PATH")
                .help("The path to the file that contains rules by which to aggregate accounts.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_llg_code")
                .short("d")
                .long("default-llg-code")
                .value_name("DEFAULT LLG CODE")
                .help("This is the default llg code.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_overdue_llg_code")
                .short("D")
                .long("default-overdue-llg-code")
                .value_name("DEFAULT OVERDUE LLG CODE")
                .help("This is the default overdue llg code.")
                .required(false)
        )
        .arg(
            Arg::with_name("tenor_file_path")
                .short("t")
                .long("tenor-file")
                .value_name("TENOR")
                .help("This is the tenor file.")
                .required(true)
        )
        .arg(
            Arg::with_name("foreign_consol_ccy")
                .short("t")
                .long("foreign-consol-ccy")
                .value_name("foreign_consol_ccy")
                .help("This is the foreign consol ccy.")
                .required(true)
        )
        .arg(
            Arg::with_name("consol_ccy")
                .short("t")
                .long("consol-ccy")
                .value_name("consol_ccy")
                .help("This is the consol ccy.")
                .required(true)
        )
        .arg(
            Arg::with_name("display_local_ccy")
                .short("t")
                .long("display-local-ccy")
                .value_name("display_local_ccy")
                .help("This is the display local ccy.")
                .required(true)
        )
        .arg(
            Arg::with_name("src_local_ccy")
                .short("t")
                .long("src-local-ccy")
                .value_name("src_local_ccy")
                .help("This is the src local ccy.")
                .required(true)
        )
        .arg(
            Arg::with_name("tenor_flag")
                .short("tf")
                .long("tenor-flag")
                .value_name("TENOR FLAG")
                .help("This is the tenor flag.")
                .required(true)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .short("tf")
                .long("is-consolidated")
                .value_name("is consolidated flag")
                .help("This is the is consolidated flag.")
                .default_value("false")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_flag_values")
                .long("npa-flag-values")
                .value_name("npa flag values")
                .help("This is the npa flag values.")
                .required(true)
        )
        .arg(
            Arg::with_name("def_tenor")
                .long("def-tenor")
                .value_name("Default tenor value")
                .help("This is the Default tenor value.")
                .default_value("0")
                .required(false)
        )
        .get_matches()
}
