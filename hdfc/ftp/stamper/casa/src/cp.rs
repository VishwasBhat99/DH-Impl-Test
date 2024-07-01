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
    ftprunid: i64,
    from_date: NaiveDate,
    to_date: NaiveDate,
    input_file_path: String,
    meta_data_file_path: String,
    output_file_path: String,
    m_rule_file_path: String,
    bc_rule_file_path: String,
    adj_rule_file_path: String,
    bc_file_path: String,
    exch_rate_file_path: String,
    ftp_rates_file_path: String,
    adj_rate_file_path: String,
    default_method: i32,
    default_basecurve: i32,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    rate_precision: i8,
    bal_precision: i8,
}

impl CP {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "ftp_run_id:{}", self.ftprunid());
        info!(logger, "from_date:{}", self.from_date());
        info!(logger, "to_date:{}", self.to_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "meta_data_file: {}", self.meta_data_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "m_rule_file: {}", self.m_rule_file_path());
        info!(logger, "bc_rule_file: {}", self.bc_rule_file_path());
        info!(logger, "adj_rule_file: {}", self.adj_rule_file_path());
        info!(logger, "bc_file: {}", self.bc_file_path());
        info!(logger, "exch_rate_file: {}", self.exch_rate_file_path());
        info!(logger, "ftp_rates_file: {}", self.ftp_rates_file_path());
        info!(logger, "adj_rate_file_path: {}", self.adj_rate_file_path());
        info!(logger, "default_method: {}", self.default_method());
        info!(logger, "default_basecurve: {}", self.default_basecurve());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "rate_precision: {}", self.rate_precision());
        info!(logger, "bal_precision: {}", self.bal_precision());
    }
}

impl CP {
    fn new_from_matches(matches: clap::ArgMatches) -> CP {
        // TODO: `unwrap()`s need proper error messages.

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), true);
        let ftprunid = matches
            .value_of("ftprunid")
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap();
        let from_date = date_parser.parse(matches.value_of("from_date").unwrap());
        let to_date = date_parser.parse(matches.value_of("to_date").unwrap());

        let input_file_path = matches.value_of("input_file").unwrap().to_string();
        let meta_data_file_path = matches.value_of("meta_data_file").unwrap().to_string();
        let m_rule_file_path = matches.value_of("m_rule_file").unwrap().to_string();
        let bc_rule_file_path = matches.value_of("bc_rule_file").unwrap().to_string();
        let adj_rule_file_path = matches.value_of("adj_rule_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .unwrap()
            .to_string();
        let bc_file_path = matches.value_of("bc_file").unwrap().to_string();
        let exch_rate_file_path = matches.value_of("exch_rate_file_path").unwrap().to_string();
        let ftp_rates_file_path = matches.value_of("ftp_rates_file_path").unwrap().to_string();
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
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
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

        CP {
            ftprunid,
            from_date,
            to_date,
            input_file_path,
            meta_data_file_path,
            output_file_path,
            m_rule_file_path,
            bc_rule_file_path,
            adj_rule_file_path,
            bc_file_path,
            exch_rate_file_path,
            ftp_rates_file_path,
            adj_rate_file_path,
            default_method,
            default_basecurve,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            rate_precision,
            bal_precision,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl CP {
    pub fn ftprunid(&self) -> i64 {
        return self.ftprunid;
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
    pub fn ftp_rates_file_path(&self) -> &str {
        return &self.ftp_rates_file_path;
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
    pub fn rate_precision(&self) -> i8 {
        return self.rate_precision;
    }
    pub fn bal_precision(&self) -> i8 {
        return self.bal_precision;
    }
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.3.2913")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app helps convert inputs to outputs at lightning speed!")
        .arg(
            Arg::with_name("ftprunid")
                .long("ftp-runid")
                .value_name("FILE")
                .help("FTP run Id")
                .required(true)
        )
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
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("m_rule_file")
                .long("m-rule-file")
                .value_name("FILE")
                .help("Path to the method rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_rule_file")
                .long("bc-rule-file")
                .value_name("FILE")
                .help("Path to the basecurve rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("adj_rule_file")
                .long("adj-rule-file")
                .value_name("FILE")
                .help("Path to the adjustment rules file")
                .required(true)
        )
        .arg(
            Arg::with_name("bc_file")
                .long("bc-file")
                .value_name("FILE")
                .help("Path to the basecurve file")
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
            Arg::with_name("ftp_rates_file_path")
                .long("ftp-rates-file")
                .value_name("FILE")
                .help("Path to the FTP Rates file")
                .required(true)
        )
         .arg(
            Arg::with_name("adj_rate_file_path")
                .long("adj-rates-file")
                .value_name("FILE")
                .help("Path to the adjustment Rates file")
                .required(true)
        )
        .arg(
            Arg::with_name("default_method")
                .long("default-method")
                .value_name("Default Method")
                .help("Default method for Finnone Loans")
                .required(true)
        )
        .arg(
            Arg::with_name("default_basecurve")
                .long("default-basecurve")
                .value_name("Default Basecurve")
                .help("Default basecurve for Finnone Loans")
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
        .get_matches()
}
