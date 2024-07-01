use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_args_for_app(app_name);

    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    ftp_run_id: i64,
    from_date: NaiveDate,
    to_date: NaiveDate,
    input_file_path: String,
    avg_bal_file: String,
    req_fields_file_path: String,
    meta_data_file_path: String,
    output_file_path: String,
    m_rule_file_path: String,
    bc_rule_file_path: String,
    adj_rule_file_path: String,
    bc_file_path: String,
    exch_rate_file_path: String,
    source_local_currency: String,
    ftp_rates_file_path: String,
    default_method: i32,
    default_basecurve: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "ftp_run_id:{}", self.ftp_run_id());
        info!(logger, "from_date:{}", self.from_date());
        info!(logger, "to_date:{}", self.to_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "input_file: {}", self.avg_bal_file());
        info!(
            logger,
            "req_fields_file_path: {}",
            self.req_fields_file_path()
        );
        info!(logger, "meta_data_file: {}", self.meta_data_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "m_rule_file: {}", self.m_rule_file_path());
        info!(logger, "bc_rule_file: {}", self.bc_rule_file_path());
        info!(logger, "adj_rule_file: {}", self.adj_rule_file_path());
        info!(logger, "bc_file: {}", self.bc_file_path());
        info!(logger, "exch_rate_file: {}", self.exch_rate_file_path());
        info!(
            logger,
            "source_local_currency: {}",
            self.source_local_currency()
        );
        info!(logger, "ftp_rates_file: {}", self.ftp_rates_file_path());
        info!(logger, "default_method: {}", self.default_method());
        info!(logger, "default_basecurve: {}", self.default_basecurve());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);
        let ftp_run_id = matches
            .value_of("ftp_run_id")
            .expect("Could not read FTP Run ID")
            .to_string()
            .parse::<i64>()
            .expect("Could not parse FTP Run ID as i64");
        let from_date = date_parser.parse(
            matches
                .value_of("from_date")
                .expect("Could not parse from_date"),
        );
        let to_date = date_parser.parse(
            matches
                .value_of("to_date")
                .expect("Could not parse to_date"),
        );
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `Input file path`.")
            .to_string();
        let avg_bal_file = matches
            .value_of("avg_bal_file")
            .expect("Error getting `Average Balance file path`.")
            .to_string();
        let req_fields_file_path = matches
            .value_of("req_fields_file")
            .expect("Error getting `Req fields file path`.")
            .to_string();
        let meta_data_file_path = matches
            .value_of("meta_data_file")
            .expect("Error getting `Meta data file path`.")
            .to_string();
        let m_rule_file_path = matches
            .value_of("m_rule_file")
            .expect("Error getting `Method  rules file path`.")
            .to_string();
        let bc_rule_file_path = matches
            .value_of("bc_rule_file")
            .expect("Error getting `Basecurve rules file path`.")
            .to_string();
        let adj_rule_file_path = matches
            .value_of("adj_rule_file")
            .expect("Error getting `Adjustment rules fields file path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error in getting `Output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `Log file path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `Diagnostic file path`.")
            .to_string();
        let bc_file_path = matches
            .value_of("bc_file")
            .expect("Error getting `Basecurve file path`.")
            .to_string();
        let exch_rate_file_path = matches
            .value_of("exch_rate_file_path")
            .expect("Error getting `Exchange rate file path`.")
            .to_string();
        let source_local_currency = matches
            .value_of("source_local_currency")
            .expect("Error getting `source local currency`.")
            .to_string();
        let ftp_rates_file_path = matches
            .value_of("ftp_rates_file_path")
            .expect("Error getting `FTP Rates file path`.")
            .to_string();
        let default_method = matches
            .value_of("default_method")
            .expect("Could not read `Default Method`.")
            .to_string()
            .parse::<i32>()
            .expect("Could not parse Default Method");
        let default_basecurve = matches
            .value_of("default_basecurve")
            .expect("Could not read `Default Basecurve`.")
            .to_string()
            .parse::<i32>()
            .expect("Could not parse `Default Basecurve`.");
        let log_level = matches
            .value_of("log_level")
            .expect("Could not read `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Could not read `is_perf_diagnostics_enabled`.")
            .parse::<bool>()
            .expect("Could not parse `is_perf_diagnostics_enabled`.");

        ConfigurationParameters {
            ftp_run_id,
            from_date,
            to_date,
            input_file_path,
            avg_bal_file,
            req_fields_file_path,
            meta_data_file_path,
            output_file_path,
            m_rule_file_path,
            bc_rule_file_path,
            adj_rule_file_path,
            bc_file_path,
            exch_rate_file_path,
            source_local_currency,
            ftp_rates_file_path,
            default_method,
            default_basecurve,
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
    pub fn ftp_run_id(&self) -> i64 {
        return self.ftp_run_id;
    }
    pub fn from_date(&self) -> &NaiveDate {
        return &self.from_date;
    }
    pub fn to_date(&self) -> &NaiveDate {
        return &self.to_date;
    }
    pub fn input_file_path(&self) -> &str {
        return &self.input_file_path;
    }
    pub fn avg_bal_file(&self) -> &str {
        return &self.avg_bal_file;
    }
    pub fn req_fields_file_path(&self) -> &str {
        return &self.req_fields_file_path;
    }
    pub fn meta_data_file_path(&self) -> &str {
        return &self.meta_data_file_path;
    }
    pub fn output_file_path(&self) -> &str {
        return &self.output_file_path;
    }
    pub fn m_rule_file_path(&self) -> &str {
        return &self.m_rule_file_path;
    }
    pub fn bc_rule_file_path(&self) -> &str {
        return &self.bc_rule_file_path;
    }
    pub fn adj_rule_file_path(&self) -> &str {
        return &self.adj_rule_file_path;
    }
    pub fn bc_file_path(&self) -> &str {
        return &self.bc_file_path;
    }
    pub fn exch_rate_file_path(&self) -> &str {
        return &self.exch_rate_file_path;
    }
    pub fn source_local_currency(&self) -> &str {
        return &self.source_local_currency;
    }
    pub fn ftp_rates_file_path(&self) -> &str {
        return &self.ftp_rates_file_path;
    }
    pub fn default_method(&self) -> i32 {
        return self.default_method;
    }
    pub fn default_basecurve(&self) -> i32 {
        return self.default_basecurve;
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
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.3.3691")
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("ftp_run_id")
                .short("F")
                .long("ftp-run-id")
                .value_name("FILE")
                .help("FTP run Id")
                .required(true)
        )
        .arg(
            Arg::with_name("from_date")
                .short("f")
                .long("from-date")
                .value_name("DATE")
                .help("Start date of the FTP process date range")
                .required(true)
        )
        .arg(
            Arg::with_name("to_date")
                .short("t")
                .long("to-date")
                .value_name("DATE")
                .help("End date of the FTP process date range")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input-file")
                .value_name("FILE")
                .help("Path to input file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("avg_bal_file")
                .short("a")
                .long("avg-bal-file")
                .value_name("FILE")
                .help("Path to Average Balance file")
                .required(true)
        )
        .arg(
            Arg::with_name("meta_data_file")
                .short("m")
                .long("meta-data-file")
                .value_name("FILE")
                .help("Path to meta data file that needs to be processed")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .short("r")
                .long("req-fields-file")
                .value_name("REQ_FIELDS")
                .help("The aggregator requires some fields (such as interest rate) per account.\nThe known_fields_file parameter is a path to a file that describes the names with which to refer to such fields.")
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
            Arg::with_name("m_rule_file")
                .short("M")
                .long("m-rule-file")
                .value_name("FILE")
                .help("Path to the method rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_rule_file")
                .short("B")
                .long("bc-rule-file")
                .value_name("FILE")
                .help("Path to the basecurve rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("adj_rule_file")
                .short("A")
                .long("adj-rule-file")
                .value_name("FILE")
                .help("Path to the adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_file")
                .short("b")
                .long("bc-file")
                .value_name("FILE")
                .help("Path to the basecurve file")
                .required(true)
        )
        .arg(
            Arg::with_name("exch_rate_file_path")
                .short("e")
                .long("exch-rate-file")
                .value_name("FILE")
                .help("Path to the Exchange rate file")
                .required(true)
        )
        .arg(
            Arg::with_name("source_local_currency")
                .short("s")
                .long("src-local-ccy")
                .value_name("SOURCE LOCAL CURRENCY")
                .help("The source local currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("ftp_rates_file_path")
                .short("r")
                .long("ftp-rates-file")
                .value_name("FILE")
                .help("Path to the FTP Rates file")
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
                .short("B")
                .long("default-basecurve")
                .value_name("Default Basecurve")
                .help("Default basecurve for Finnone Loans")
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
                .short("L") 
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
        .get_matches()
}
