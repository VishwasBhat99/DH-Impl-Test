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
    aggr_file_path: String,
    amb_file_path: String,
    exch_rate_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
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
        info!(logger, "adj_rule_file: {}", self.aggr_file_path());
        info!(logger, "amb_file: {}", self.amb_file_path());
        info!(logger, "exch_rate_file: {}", self.exch_rate_file_path());
        info!(logger, "log_level: {}", self.log_level());
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
        let aggr_file_path = matches.value_of("aggr_file_path").unwrap().to_string();
        let amb_file_path = matches.value_of("amb_file_path").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let log_file_path = matches.value_of("log_file").unwrap().to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .unwrap()
            .to_string();
        let exch_rate_file_path = matches.value_of("exch_rate_file_path").unwrap().to_string();
        let log_level = matches.value_of("log_level").unwrap().to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap();

        CP {
            ftprunid,
            from_date,
            to_date,
            input_file_path,
            meta_data_file_path,
            output_file_path,
            aggr_file_path,
            amb_file_path,
            exch_rate_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
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
    pub fn aggr_file_path(&self) -> &str {
        return &self.aggr_file_path;
    }
    pub fn amb_file_path(&self) -> &str {
        return &self.amb_file_path;
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
}

fn get_args_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app helps convert inputs to outputs at lightning speed!")
        .version("1.1.2913")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
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
            Arg::with_name("aggr_file_path")
                .long("aggr-file-path")
                .value_name("FILE")
                .help("Path to the aggreated balance file")
                .required(true)
        )
        .arg(
            Arg::with_name("amb_file_path")
                .long("amb-file-path")
                .value_name("FILE")
                .help("Path to the average balance file")
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
        .get_matches()
}
