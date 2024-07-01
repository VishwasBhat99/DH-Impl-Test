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
    input_cd_details: String,
    input_cd_price_details: String,
    output_file_path: String,
    req_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    as_on_date: NaiveDate,
    cd_details_sheetname: String,
    cd_pricedetails_sheetname: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_cd_details: {}", self.input_cd_details());
        info!(logger, "input_cd_price_details: {}", self.input_cd_price_details());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "req_file: {}", self.req_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "cd_details_sheetname: {}", self.cd_details_sheetname());
        info!(logger, "cd_pricedetails_sheetname: {}", self.cd_pricedetails_sheetname());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_cd_details = matches
            .value_of("input_cd_details")
            .expect("Error getting `input_cd_details`.")
            .to_string();
        let input_cd_price_details = matches
            .value_of("input_cd_price_details")
            .expect("Error getting `input_cd_price_details`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let req_file_path = matches
            .value_of("req_file")
            .expect("Error getting `req_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let cd_details_sheetname = matches
            .value_of("cd_details_sheetname")
            .expect("Error getting `cd_details_sheetname`.")
            .to_string();
        let cd_pricedetails_sheetname = matches
            .value_of("cd_pricedetails_sheetname")
            .expect("Error getting `cd_pricedetails_sheetname`.")
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
            input_cd_details,
            input_cd_price_details,
            output_file_path,
            req_file_path,
            log_file_path,
            diagnostics_file_path,
            as_on_date,
            cd_details_sheetname,
            cd_pricedetails_sheetname,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_cd_details(&self) -> &str {
        &self.input_cd_details
    }
    pub fn input_cd_price_details(&self) -> &str {
        &self.input_cd_price_details
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn req_file_path(&self) -> &str {
        &self.req_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn cd_details_sheetname(&self) -> &str {
        &self.cd_details_sheetname
    }
    pub fn cd_pricedetails_sheetname(&self) -> &str {
        &self.cd_pricedetails_sheetname
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
        .about("Program for non equity-bond")
        .arg(
            Arg::with_name("input_cd_details")
                .long("input-cd-details")
                .value_name("Input CD Details")
                .help("Input CD Details file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_cd_price_details")
                .long("input-cd-price-details")
                .value_name("Input CD Price Details")
                .help("Input CD Price Details file path.")
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
            Arg::with_name("req_file")
                .long("req-file")
                .value_name("required File Path")
                .help("Path to output file.")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("cd_details_sheetname")
                .long("cd_details_sheetname")
                .value_name("cd_details_sheetname")
                .help("Name of the input sheet.")
                .required(true)
        )
        .arg(
            Arg::with_name("cd_pricedetails_sheetname")
                .long("cd_pricedetails_sheetname")
                .value_name("cd_pricedetails_sheetname")
                .help("Name of the input sheet.")
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
