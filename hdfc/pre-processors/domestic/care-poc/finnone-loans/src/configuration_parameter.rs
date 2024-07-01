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
    stg_non_sec_exposure_fn: String,
    finnone_extract: String,
    finnone_npa: String,
    finnone_master: String,
    stg_company_details: String,
    restructured_merged: String,
    fn_collateral: String,
    output_file_path: String,
    rep_sheet_name: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "stg_non_sec_exposure_fn_input_file: {}", self.stg_non_sec_exposure_fn());
        info!(logger, "finnone_extract_input_file: {}", self.finnone_extract());
        info!(logger, "finnone_npa_input_file: {}", self.finnone_npa());
        info!(logger, "finnone_master_input_file: {}", self.finnone_master());
        info!(logger, "stg_company_details_input_file: {}", self.stg_company_details());
        info!(logger, "restructured_merged_input_file: {}", self.restructured_merged());
        info!(logger, "fn_collateral_input_file: {}", self.fn_collateral());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "ref_shee_name: {}", self.rep_sheet_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let stg_non_sec_exposure_fn = matches
            .value_of("stg_non_sec_exposure_fn")
            .expect("Error getting `stg_non_sec_exposure_fn`.")
            .to_string();
        let finnone_extract = matches
            .value_of("finnone_extract")
            .expect("Error getting `finnone_extract`.")
            .to_string();
        let finnone_npa = matches
            .value_of("finnone_npa")
            .expect("Error getting `finnone_npa`.")
            .to_string();
        let finnone_master = matches
            .value_of("finnone_master")
            .expect("Error getting `finnone_master`.")
            .to_string();
        let stg_company_details = matches
            .value_of("stg_company_details")
            .expect("Error getting `stg_company_details`.")
            .to_string();
        let restructured_merged = matches
            .value_of("restructured_merged")
            .expect("Error getting `restructured_merged`.")
            .to_string();
        let fn_collateral = matches
            .value_of("fn_collateral")
            .expect("Error getting `fn_collateral`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let rep_sheet_name = matches
            .value_of("rep_sheet_name")
            .expect("Error getting `rep_sheet_name`.")
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
            stg_non_sec_exposure_fn,
            finnone_extract,
            finnone_npa,
            finnone_master,
            stg_company_details,
            restructured_merged,
            fn_collateral,
            output_file_path,
            rep_sheet_name,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

impl ConfigurationParameters {
    pub fn stg_non_sec_exposure_fn(&self) -> &str {
        &self.stg_non_sec_exposure_fn
    }
    pub fn finnone_extract(&self) -> &str {
        &self.finnone_extract
    }
    pub fn finnone_npa(&self) -> &str {
        &self.finnone_npa
    }
    pub fn finnone_master(&self) -> &str {
        &self.finnone_master
    }
    pub fn stg_company_details(&self) -> &str {
        &self.stg_company_details
    }
    pub fn restructured_merged(&self) -> &str {
        &self.restructured_merged
    }
    pub fn fn_collateral(&self) -> &str {
        &self.fn_collateral
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn rep_sheet_name(&self) -> &str {
        &self.rep_sheet_name
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
        .about("Program For Pre Process CARE Finnone Loans!!")
        .version("1.1.5031")
        .arg(
            Arg::with_name("stg_non_sec_exposure_fn")
                .long("stg-non-sec-exposure-fn")
                .value_name("stg_non_sec_exposure_fn")
                .help("path to stg_non_sec_exposure_fn.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_extract")
                .long("finnone-extract")
                .value_name("finnone_extract")
                .help("path to finnone_extract.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_npa")
                .long("finnone-npa")
                .value_name("finnone_npa")
                .help("path to finnone_npa.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_master")
                .long("finnone-master")
                .value_name("finnone_master")
                .help("path to finnone master.")
                .required(true)
        )
        .arg(
            Arg::with_name("stg_company_details")
                .long("stg-company-details")
                .value_name("stg_company_details")
                .help("path to stg_company_details.")
                .required(true)
        )
        .arg(
            Arg::with_name("restructured_merged")
                .long("restructured-merged")
                .value_name("restructured_merged")
                .help("path to restructured_merged.")
                .required(true)
        )
        .arg(
            Arg::with_name("fn_collateral")
                .long("fn-collateral")
                .value_name("fn_collateral")
                .help("path to fn_collateral.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output_file")
                .help("Path to output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rep_sheet_name")
                .long("rep-sheet-name")
                .value_name("rep_sheet_name")
                .help("Path to output file.")
                .required(false)
                .default_value("Sheet1")
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
