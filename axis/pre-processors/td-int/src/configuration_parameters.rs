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
    input_file_tam: String,
    input_file_itc: String,
    input_file_icv: String,
    input_file_tvs: String,
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
        info!(logger, "input_file_tam: {}", self.input_file_tam());
        info!(logger, "input_file_itc: {}", self.input_file_itc());
        info!(logger, "input_file_icv: {}", self.input_file_icv());
        info!(logger, "input_file_tvs: {}", self.input_file_tvs());
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
        let input_file_tam = matches
            .value_of("input_file_tam")
            .expect("Error getting `input_file_tam`.")
            .to_string();
        let input_file_itc = matches
            .value_of("input_file_itc")
            .expect("Error getting `input_file_itc`.")
            .to_string();
        let input_file_icv = matches
            .value_of("input_file_icv")
            .expect("Error getting `input_file_icv`.")
            .to_string();
        let input_file_tvs = matches
            .value_of("input_file_tvs")
            .expect("Error getting `input_file_tvs`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
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
            input_file_tam,
            input_file_itc,
            input_file_icv,
            input_file_tvs,
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
    pub fn input_file_tam(&self) -> &str {
        &self.input_file_tam
    }
    pub fn input_file_itc(&self) -> &str {
        &self.input_file_itc
    }
    pub fn input_file_icv(&self) -> &str {
        &self.input_file_icv
    }
    pub fn input_file_tvs(&self) -> &str {
        &self.input_file_tvs
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
        .about("Program For Pre Process TD!!")
        .arg(
            Arg::with_name("input_file_gam")
                .long("input-file-gam")
                .value_name("Input File GAM")
                .help("GAM Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_tam")
                .long("input-file-tam")
                .value_name("Input File TAM")
                .help("TAM Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_itc")
                .long("input-file-itc")
                .value_name("Input File ITC")
                .help("ITC Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_icv")
                .long("input-file-icv")
                .value_name("Input File ICV")
                .help("ICV Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_tvs")
                .long("input-file-tvs")
                .value_name("Input File TVS")
                .help("TVS Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
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
