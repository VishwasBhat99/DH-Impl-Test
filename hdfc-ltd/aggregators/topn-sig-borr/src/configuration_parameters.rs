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
    input_file_path: String,
    output_file_path: String,
    ex_rate_file_path: String,
    base_ccy: String,
    country_code: String,
    as_on_date: NaiveDate,
    sig_perc: f64,
    liab_val: f64,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "ex_rate_file: {}", self.ex_rate_file_path());
        info!(logger, "base_ccy: {}", self.base_ccy());
        info!(logger, "country_code: {}", self.country_code());
        info!(logger, "sig_perc: {}", self.sig_perc());
        info!(logger, "liab_val: {}", self.liab_val());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let ex_rate_file_path = matches
            .value_of("ex_rate_file")
            .expect("Error while getting Exchange Rate file path.")
            .to_string();
        let sig_perc = matches
            .value_of("sig_perc")
            .expect("Error in getting `sig_perc` value.")
            .parse::<f64>()
            .expect("Error in parsing significant percentage.");
        let liab_val = matches
            .value_of("liab_val")
            .expect("Error in getting `liab_val` value.")
            .parse::<f64>()
            .expect("Error in parsing liability amount value.");
        let base_ccy = matches
            .value_of("base_ccy")
            .expect("Error while getting `base currency`.")
            .to_string();
        let country_code = matches
            .value_of("country_code")
            .expect("Error while getting `country code`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            output_file_path,
            ex_rate_file_path,
            base_ccy,
            country_code,
            as_on_date,
            sig_perc,
            liab_val,
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
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn base_ccy(&self) -> &str {
        &self.base_ccy
    }
    pub fn country_code(&self) -> &str {
        &self.country_code
    }
    pub fn ex_rate_file_path(&self) -> &str {
        &self.ex_rate_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn sig_perc(&self) -> &f64 {
        &self.sig_perc
    }
    pub fn liab_val(&self) -> &f64 {
        &self.liab_val
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
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .version("1.0.0")
        .about("This program generates Significant TopN Borrowings account report")
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("FILE")
                .help("Path to TD input file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::new("ex_rate_file")
                .long("ex-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::new("base_ccy")
                .long("base-ccy")
                .value_name("Base CCY")
                .help("Value of base currency.")
                .required(true)
        )
        .arg(
            Arg::new("country_code")
                .long("country-code")
                .value_name("Country Code")
                .help("Value of country code.")
                .required(true)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false),
        )
        .arg(
            Arg::new("sig_perc")
                .long("sig-perc")
                .help("Count for significant percentage")
                .value_name("significant percentage")
                .required(true)
        )
        .arg(
            Arg::new("liab_val")
                .long("liab-val")
                .help("Liability Amount value")
                .value_name("liability_value")
                .required(true)
        )
        .get_matches()
}
