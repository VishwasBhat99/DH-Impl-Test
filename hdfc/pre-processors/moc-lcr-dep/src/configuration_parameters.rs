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
    sls_file: String,
    nwd_file: String,
    tot_dep_bal_file: String,
    output_file: String,
    currency: String,
    country: String,
    amount: String,
    as_on_date: NaiveDate,
    delimeter: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "sls_file: {}", self.sls_file());
        info!(logger, "nwd_file: {}", self.nwd_file());
        info!(logger, "tot_dep_bal_file: {}", self.tot_dep_bal_file());
        info!(
            logger,
            "output_file: {}",
            self.output_file()
        );
        info!(logger, "currency: {}", self.currency());
        info!(logger, "country: {}", self.country());
        info!(logger, "amount: {}", self.amount());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "delimeter: {}", self.delimeter());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let sls_file = matches
            .value_of("sls_file")
            .expect("Error getting `sls_file`.")
            .to_string();
        let nwd_file = matches
            .value_of("nwd_file")
            .expect("Error getting `nwd_file`.")
            .to_string();
        let tot_dep_bal_file = matches
            .value_of("tot_dep_bal_file")
            .expect("Error getting `tot_dep_bal_file`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let country = matches
            .value_of("country")
            .expect("Error getting `country`.")
            .to_string();
        let amount = matches
            .value_of("amount")
            .expect("Error getting `amount`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let delimeter = matches
            .value_of("delimeter")
            .expect("Error getting `delimeter`.")
            .to_string();
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
            sls_file,
            nwd_file,
            tot_dep_bal_file,
            output_file,
            currency,
            country,
            amount,
            as_on_date,
            delimeter,
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
    pub fn sls_file(&self) -> &str {
        &self.sls_file
    }
    pub fn nwd_file(&self) -> &str {
        &self.nwd_file
    }
    pub fn tot_dep_bal_file(&self) -> &str {
        &self.tot_dep_bal_file
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn amount(&self) -> &str {
        &self.amount
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn delimeter(&self) -> &str {
        &self.delimeter
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
        .about("MOC LCR Deposit Program!!")
        .version("1.0.3265")
        .author("harsh8501 <harsh.sk@surya-soft.com>")
        .arg(
            Arg::with_name("sls_file")
                .long("sls-file")
                .value_name("SLS File")
                .help("SLS File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("nwd_file")
                .long("nwd-file")
                .value_name("NWD File")
                .help("NWD File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("tot_dep_bal_file")
                .long("tot-dep-bal-file")
                .value_name("Total Deposit Balance File")
                .help("Total Deposit Balance File path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to Output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("currency")
                .help("Currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("country")
                .long("country")
                .value_name("country")
                .help("Country.")
                .required(true)
        )
        .arg(
            Arg::with_name("amount")
                .long("amount")
                .value_name("amount")
                .help("Amount.")
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
            Arg::with_name("delimeter")
                .long("delimeter")
                .value_name("Delimeter")
                .help("The delimeter for which the program has to run.")
                .default_value("|")
                .required(false)
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
