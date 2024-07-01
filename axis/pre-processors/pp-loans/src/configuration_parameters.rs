use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_gam: String,
    finacle_finlite_flg: String,
    input_file_lam: String,
    input_file_int_rate: String,
    tbl_code_file_path: String,
    input_file_od_int: String,
    input_file_lrs: String,
    input_file_npa: String,
    input_file_lsp: String,
    input_file_benchmark: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_gam: {}", self.input_file_gam());
        info!(
            logger,
            "finacle_finlite_flg: {}",
            self.finacle_finlite_flg()
        );
        info!(logger, "input_file_lam: {}", self.input_file_lam());
        info!(logger, "input_file_lrs: {}", self.input_file_lrs());
        info!(logger, "input_file_npa: {}", self.input_file_npa());
        info!(logger, "input_file_lsp: {}", self.input_file_lsp());
        info!(logger, "tbl_code_file: {}", self.tbl_code_file_path());
        info!(
            logger,
            "input_file_benchmark: {}",
            self.input_file_benchmark()
        );
        info!(
            logger,
            "input_file_int_rate: {}",
            self.input_file_int_rate()
        );
        info!(logger, "input_file_od_int: {}", self.input_file_od_int());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_gam = matches
            .value_of("input_file_gam")
            .expect("Error getting `input_file_gam`.")
            .to_string();
        let finacle_finlite_flg = matches
            .value_of("finacle_finlite_flg")
            .expect("Error getting `finacle_finlite_flg`.")
            .to_string();
        let input_file_lam = matches
            .value_of("input_file_lam")
            .expect("Error getting `input_file_lam`.")
            .to_string();
        let input_file_lrs = matches
            .value_of("input_file_lrs")
            .expect("Error getting `input_file_lrs`.")
            .to_string();
        let input_file_npa = matches
            .value_of("input_file_npa")
            .expect("Error getting `input_file_npa`.")
            .to_string();
        let tbl_code_file_path = matches
            .value_of("tbl_code_file")
            .expect("Error getting `tbl_code_file` value.")
            .to_string();
        let input_file_lsp = matches
            .value_of("input_file_lsp")
            .expect("Error getting `input_file_lsp`.")
            .to_string();
        let input_file_benchmark = matches
            .value_of("input_file_benchmark")
            .expect("Error getting `input_file_benchmark`.")
            .to_string();
        let input_file_int_rate = matches
            .value_of("input_file_int_rate")
            .expect("Error getting `input_file_int_rate`.")
            .to_string();
        let input_file_od_int = matches
            .value_of("input_file_od_int")
            .expect("Error getting `input_file_od_int`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
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
            input_file_gam,
            finacle_finlite_flg,
            input_file_lam,
            input_file_lrs,
            input_file_npa,
            input_file_lsp,
            tbl_code_file_path,
            input_file_benchmark,
            input_file_int_rate,
            input_file_od_int,
            output_file_path,
            as_on_date,
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
    pub fn input_file_gam(&self) -> &str {
        &self.input_file_gam
    }
    pub fn finacle_finlite_flg(&self) -> &str {
        &self.finacle_finlite_flg
    }
    pub fn input_file_lam(&self) -> &str {
        &self.input_file_lam
    }
    pub fn input_file_lrs(&self) -> &str {
        &self.input_file_lrs
    }
    pub fn input_file_npa(&self) -> &str {
        &self.input_file_npa
    }
    pub fn input_file_lsp(&self) -> &str {
        &self.input_file_lsp
    }
    pub fn input_file_benchmark(&self) -> &str {
        &self.input_file_benchmark
    }
    pub fn input_file_int_rate(&self) -> &str {
        &self.input_file_int_rate
    }
    pub fn input_file_od_int(&self) -> &str {
        &self.input_file_od_int
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn tbl_code_file_path(&self) -> &str {
        &self.tbl_code_file_path
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
        .about("Program For Pre Processing LOAN Data.")
        .version("1.0.4413")
        .arg(
            Arg::with_name("input_file_gam")
                .long("input-file-gam")
                .value_name("Input File GAM")
                .help("GAM Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("finacle_finlite_flg")
                .long("finacle-finlite-flg")
                .value_name("Finacle Finlite Flag.")
                .help("Flag to determine if the Data is finacle or Finlite.")
                .required(true)
                .possible_values(&["FINLITE", "FINACLE"])
        )
        .arg(
            Arg::with_name("tbl_code_file")
                .short("tb")
                .long("tbl-code-file")
                .value_name("TBL_CODE_FILE")
                .help("Path to tbl code file file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_lam")
                .long("input-file-lam")
                .value_name("Input File LAM")
                .help("LAM Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_lrs")
                .long("input-file-lrs")
                .value_name("Input File LRS")
                .help("LRS Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_npa")
                .long("input-file-npa")
                .value_name("Input File NPA")
                .help("NPA Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_lsp")
                .long("input-file-lsp")
                .value_name("Input File LSP")
                .help("LSP Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_benchmark")
                .long("input-file-benchmark")
                .value_name("Input File Benchmark")
                .help("Benchamrk Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_int_rate")
                .long("input-file-int-rate")
                .value_name("Input File INT Rate")
                .help("Int Rate Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_od_int")
                .long("input-file-od-int")
                .value_name("Input File OD Int")
                .help("OD Int Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("output File Path")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
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
        .get_matches()
}
