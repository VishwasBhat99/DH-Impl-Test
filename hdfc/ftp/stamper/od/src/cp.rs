use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_cp(app_name: &str) -> CP {
    let matches = get_args_for_app(app_name);

    let parameters = CP::new_from_matches(matches);
    return parameters;
}

pub struct CP {
    from_date: NaiveDate,
    to_date: NaiveDate,
    as_on_date: NaiveDate,
    input_file_path: String,
    meta_data_file_path: String,
    output_file_path: String,
    m_rule_file_path: String,
    bc_rule_file_path: String,
    fix_adj_rule_file_path: String,
    var_adj_rule_file_path: String,
    bc_file_path: String,
    adj_rate_file_path: String,
    default_method: i32,
    default_basecurve: i32,
    fixed_adj_count: i32,
    var_adj_count: i32,
    exch_rate_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    rate_precision: i8,
    bal_precision: i8,
    is_closed: bool,
    req_fields_file: String,
    amb_file_path: String,
    adjustment_config_file_path: String,
}

impl CP {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "from_date:{}", self.from_date());
        info!(logger, "to_date:{}", self.to_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "meta_data_file: {}", self.meta_data_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "m_rule_file: {}", self.m_rule_file_path());
        info!(logger, "bc_rule_file: {}", self.bc_rule_file_path());
        info!(logger, "adj_rule_file: {}", self.fix_adj_rule_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "var_adj_rule_file: {}",
            self.var_adj_rule_file_path()
        );
        info!(logger, "bc_file: {}", self.bc_file_path());
        info!(logger, "adj_rate_file_path: {}", self.adj_rate_file_path());
        info!(logger, "default_method: {}", self.default_method());
        info!(logger, "default_basecurve: {}", self.default_basecurve());
        info!(logger, "fixed_adj_count: {}", self.fixed_adj_count());
        info!(logger, "var_adj_count: {}", self.var_adj_count());
        info!(logger, "exch_rate_file: {}", self.exch_rate_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "rate_precision: {}", self.rate_precision());
        info!(logger, "bal_precision: {}", self.bal_precision());
        info!(logger, "is_closed: {}", self.is_closed());
        info!(logger, "req_fields_file: {}", self.req_fields_file());
        info!(
            logger,
            "adjustment_config_file_path: {}",
            self.adjustment_config_file_path()
        );
        info!(logger, "amb_file_path: {}", self.amb_file_path());
    }
}

impl CP {
    fn new_from_matches(matches: clap::ArgMatches) -> CP {
        // TODO: `unwrap()`s need proper error messages.

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);
        let from_date = date_parser.parse(matches.value_of("from_date").unwrap());
        let to_date = date_parser.parse(matches.value_of("to_date").unwrap());
        let as_on_date = date_parser.parse(matches.value_of("as_on_date").unwrap());
        let adjustment_config_file_path = matches
            .value_of("adjustment_config_file_path")
            .unwrap()
            .to_string();
        let amb_file_path = matches.value_of("amb_file_path").unwrap().to_string();
        let input_file_path = matches.value_of("input_file").unwrap().to_string();
        let meta_data_file_path = matches.value_of("meta_data_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .unwrap()
            .to_string();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let m_rule_file_path = matches.value_of("m_rule_file").unwrap().to_string();
        let bc_rule_file_path = matches.value_of("bc_rule_file").unwrap().to_string();
        let fix_adj_rule_file_path = matches.value_of("fix_adj_rule_file").unwrap().to_string();
        let var_adj_rule_file_path = matches.value_of("var_adj_rule_file").unwrap().to_string();
        let bc_file_path = matches.value_of("bc_file").unwrap().to_string();
        let exch_rate_file_path = matches.value_of("exch_rate_file_path").unwrap().to_string();
        let adj_rate_file_path = matches.value_of("adj_rate_file_path").unwrap().to_string();
        let default_method = matches
            .value_of("default_method")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let default_basecurve = matches
            .value_of("default_basecurve")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let fixed_adj_count = matches
            .value_of("fixed_adj_count")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let var_adj_count = matches
            .value_of("var_adj_count")
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let rate_precision = matches
            .value_of("rate_precision")
            .unwrap()
            .parse::<i8>()
            .unwrap();
        let bal_precision = matches
            .value_of("bal_precision")
            .unwrap()
            .parse::<i8>()
            .unwrap();
        let is_closed = matches
            .value_of("is_closed")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let req_fields_file = matches.value_of("req_fields_file").unwrap().to_string();

        CP {
            from_date,
            to_date,
            as_on_date,
            input_file_path,
            meta_data_file_path,
            output_file_path,
            m_rule_file_path,
            bc_rule_file_path,
            amb_file_path,
            fix_adj_rule_file_path,
            var_adj_rule_file_path,
            bc_file_path,
            adjustment_config_file_path,
            adj_rate_file_path,
            default_method,
            default_basecurve,
            fixed_adj_count,
            var_adj_count,
            exch_rate_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            rate_precision,
            bal_precision,
            is_closed,
            req_fields_file,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl CP {
    pub fn from_date(&self) -> &NaiveDate {
        return &self.from_date;
    }
    pub fn to_date(&self) -> &NaiveDate {
        return &self.to_date;
    }
    pub fn input_file_path(&self) -> &str {
        return &self.input_file_path;
    }
    pub fn meta_data_file_path(&self) -> &str {
        return &self.meta_data_file_path;
    }
    pub fn output_file_path(&self) -> &str {
        return &self.output_file_path;
    }
    pub fn exch_rate_file_path(&self) -> &str {
        return &self.exch_rate_file_path;
    }
    pub fn log_file_path(&self) -> &str {
        return &self.log_file_path;
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
    pub fn m_rule_file_path(&self) -> &str {
        return &self.m_rule_file_path;
    }
    pub fn bc_rule_file_path(&self) -> &str {
        return &self.bc_rule_file_path;
    }
    pub fn fix_adj_rule_file_path(&self) -> &str {
        return &self.fix_adj_rule_file_path;
    }
    pub fn var_adj_rule_file_path(&self) -> &str {
        return &self.var_adj_rule_file_path;
    }
    pub fn bc_file_path(&self) -> &str {
        return &self.bc_file_path;
    }
    pub fn adj_rate_file_path(&self) -> &str {
        return &self.adj_rate_file_path;
    }
    pub fn default_method(&self) -> i32 {
        return self.default_method;
    }
    pub fn default_basecurve(&self) -> i32 {
        return self.default_basecurve;
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        return &self.as_on_date;
    }
    pub fn fixed_adj_count(&self) -> i32 {
        return self.fixed_adj_count;
    }
    pub fn var_adj_count(&self) -> i32 {
        return self.var_adj_count;
    }
    pub fn rate_precision(&self) -> i8 {
        return self.rate_precision;
    }
    pub fn bal_precision(&self) -> i8 {
        return self.bal_precision;
    }
    pub fn is_closed(&self) -> bool {
        return self.is_closed;
    }
    pub fn amb_file_path(&self) -> &str {
        return &self.amb_file_path;
    }
    pub fn adjustment_config_file_path(&self) -> &str {
        return &self.adjustment_config_file_path;
    }
    pub fn req_fields_file(&self) -> &str {
        return &self.req_fields_file;
    }
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("2.5.4961")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("from_date")
                .long("from-date")
                .value_name("DATE")
                .help("Start date of the FTP process date range")
                .required(true)
        )
        .arg(
            Arg::with_name("to_date")
                .long("to-date")
                .value_name("DATE")
                .help("End date of the FTP process date range")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("AS ON DATE")
                .help("as on date")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("meta_data_file")
                .long("meta-data-file")
                .value_name("FILE")
                .help("Path to meta data file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("amb_file_path")
                .long("amb-file-path")
                .value_name("AMB FILE")
                .help("Path to amb files")
                .required(true)
        )
        .arg(
            Arg::with_name("adjustment_config_file_path")
                .long("adj-config-file-path")
                .value_name("ADJUSTMENT FILE")
                .help("Path to adjustment file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("exch_rate_file_path")
                .long("exch-rate-file")
                .value_name("FILE")
                .help("Path to the Exchange rate file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")     // 'l', 'o' taken; 'g' doesn't make sense; 'v' stands for 'verbose'
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
            Arg::with_name("m_rule_file")
                .short("i")
                .long("m-rule-file")
                .value_name("FILE")
                .help("Path to the method rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_rule_file")
                .short("i")
                .long("bc-rule-file")
                .value_name("FILE")
                .help("Path to the basecurve rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("fix_adj_rule_file")
                .short("i")
                .long("fix-adj-rule-file")
                .value_name("FILE")
                .help("Path to the fixed adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("var_adj_rule_file")
                .short("i")
                .long("var-adj-rule-file")
                .value_name("FILE")
                .help("Path to the variable adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_file")
                .short("i")
                .long("bc-file")
                .value_name("FILE")
                .help("Path to the basecurve file")
                .required(true)
        )
         .arg(
            Arg::with_name("adj_rate_file_path")
                .short("i")
                .long("adj-rates-file")
                .value_name("FILE")
                .help("Path to the adjustment Rates file")
                .required(true)
        )
        .arg(
            Arg::with_name("default_method")
                .short("d")
                .long("default-method")
                .value_name("Default Method")
                .help("Default method for Finnone Loans")
                .required(true)
        )
        .arg(
            Arg::with_name("default_basecurve")
                .short("d")
                .long("default-basecurve")
                .value_name("Default Basecurve")
                .help("Default basecurve for Finnone Loans")
                .required(true)
        )
        .arg(
            Arg::with_name("fixed_adj_count")
                .short("d")
                .long("fixed-adjustments-count")
                .value_name("fixed adjustments count")
                .help("count of fixed adjustments")
                .required(true)
        )
        .arg(
            Arg::with_name("var_adj_count")
                .short("d")
                .long("var-adjustments-count")
                .value_name("Variable adjustments count")
                .help("Count of variable adjustments")
                .required(true)
        )
        .arg(
            Arg::with_name("rate_precision")
                .long("rate-precision")
                .value_name("Rate Precision")
                .help("The flag that decides the round off factor for rate fields.")
                .default_value("4")
                .required(false)
        )
        .arg(
            Arg::with_name("bal_precision")
                .long("bal-precision")
                .value_name("Balance Precision")
                .help("The flag that decides the round off factor for balance fields.")
                .default_value("4")
                .required(false)
        )
        .arg(
            Arg::with_name("is_closed")
                .long("is-closed")
                .value_name("Is Closed")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether accounts are closed or open.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .short("r")
                .long("req-fields-file")
                .value_name("REQ_FIELDS_FILE")
                .help("Path to req fields file that is used in fields mapping from metadata")
                .required(true)
        )
        .get_matches()
}
