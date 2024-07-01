use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use sdb_day_convention::conventions::Conventions;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_sec_all_file_path: String,
    input_sec_cblo_file_path: String,
    input_sec_ccil_file_path: String,
    input_sec_repo_file_path: String,
    input_manual_file_path: String,
    input_hqla_file_path: String,
    output_file_path: String,
    accrued_day_convention: Conventions,
    as_on_date: NaiveDate,
    default_repo_mat_date: NaiveDate,
    country_id: String,
    currency: String,
    input_delimiter: String,
    input_date_formats: Vec<String>,
    output_date_format: String,
    book_categories: Vec<String>,
    required_manual_fields_file: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "default_repo_mat_date: {}",
            self.default_repo_mat_date()
        );
        info!(logger, "country_id: {}", self.country_id());
        info!(
            logger,
            "accrued_day_convention: {:?}",
            self.accrued_day_convention()
        );
        info!(logger, "currency: {}", self.currency());
        info!(logger, "input_delimiter: {}", self.input_delimiter());
        info!(
            logger,
            "input_date_formats: {:?}",
            self.input_date_formats()
        );
        info!(logger, "output_date_format: {}", self.output_date_format());
        info!(
            logger,
            "input_sec_all_file_path: {}",
            self.input_sec_all_file_path()
        );
        info!(
            logger,
            "input_sec_cblo_file_path: {}",
            self.input_sec_cblo_file_path()
        );
        info!(
            logger,
            "input_sec_ccil_file_path: {}",
            self.input_sec_ccil_file_path()
        );
        info!(
            logger,
            "input_sec_repo_file_path: {}",
            self.input_sec_repo_file_path()
        );
        info!(
            logger,
            "input_manual_file_path: {}",
            self.input_manual_file_path()
        );
        info!(
            logger,
            "input_hqla_file_path: {}",
            self.input_hqla_file_path()
        );
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "book_categories: {:?}", self.book_categories());
        info!(
            logger,
            "required_manual_fields_file: {:?}",
            self.required_manual_fields_file_path()
        );
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_sec_all_file_path = matches
            .value_of("input_sec_all_file")
            .expect("Error getting `input sec all file`.")
            .to_string();
        let input_sec_cblo_file_path = matches
            .value_of("input_sec_cblo_file")
            .expect("Error getting `input sec cblo file`.")
            .to_string();
        let input_sec_ccil_file_path = matches
            .value_of("input_sec_ccil_file")
            .expect("Error getting `input sec ccil file`.")
            .to_string();
        let accrued_day_convention = {
            let conv = matches
                .value_of("accrued_day_convention")
                .expect("Error getting `accrued_day_convention` value.");
            match conv {
                "Accrued30/360" => Conventions::AccruedThirtyby360,
                _ => {
                    panic!(
                        "Incorrect accrued day convention parameter passed. Must be Accrued30/360."
                    )
                }
            }
        };
        let country_id = matches
            .value_of("country_id")
            .expect("Error getting `country_id`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let input_sec_repo_file_path = matches
            .value_of("input_sec_repo_file")
            .expect("Error getting `input sec repo file`.")
            .to_string();
        let input_manual_file_path = matches
            .value_of("input_manual_file")
            .expect("Error getting `input manual file`.")
            .to_string();
        let input_hqla_file_path = matches
            .value_of("input_hqla_file")
            .expect("Error getting `input hqla file`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `Output file path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `General log file path`.")
            .to_string();
        // set this as false
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        // If the date flag wasn't set, we'll use a random string and the parser will
        // return today's date.
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `As on date`."),
        );
        let default_repo_mat_date = date_parser.parse(
            matches
                .value_of("default_repo_mat_date")
                .expect("Error getting `default_repo_mat_date`."),
        );
        let input_delimiter = matches
            .value_of("input_delimiter")
            .expect("Error getting `input_delimiter`.")
            .to_string();
        let input_date_formats = matches
            .value_of("input_date_formats")
            .expect("Error getting `input_date_formats`.")
            .to_string()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let output_date_format = matches
            .value_of("output_date_format")
            .expect("Error getting `output_date_format`.")
            .to_string();
        let book_categories = matches
            .value_of("book_categories")
            .expect("Error getting `book_categories`.")
            .to_string()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let required_manual_fields_file_path = matches
            .value_of("required_manual_fields_file")
            .expect("Error getting `required_manual_fields_file`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `Diagnostics log file path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error while getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error while getting ``.")
            .parse::<bool>()
            .expect("Error while parsing `is perf diagnostics enabled` as bool.");

        ConfigurationParameters {
            input_sec_all_file_path,
            input_sec_cblo_file_path,
            input_sec_ccil_file_path,
            input_sec_repo_file_path,
            input_manual_file_path,
            input_hqla_file_path,
            output_file_path,
            accrued_day_convention,
            as_on_date,
            default_repo_mat_date,
            country_id,
            currency,
            input_delimiter,
            input_date_formats,
            output_date_format,
            book_categories,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            required_manual_fields_file: required_manual_fields_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_sec_all_file_path(&self) -> &str {
        &self.input_sec_all_file_path
    }
    pub fn input_sec_cblo_file_path(&self) -> &str {
        &self.input_sec_cblo_file_path
    }
    pub fn accrued_day_convention(&self) -> &Conventions {
        &self.accrued_day_convention
    }
    pub fn input_sec_ccil_file_path(&self) -> &str {
        &self.input_sec_ccil_file_path
    }
    pub fn input_sec_repo_file_path(&self) -> &str {
        &self.input_sec_repo_file_path
    }
    pub fn input_manual_file_path(&self) -> &str {
        &self.input_manual_file_path
    }
    pub fn input_hqla_file_path(&self) -> &str {
        &self.input_hqla_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn default_repo_mat_date(&self) -> &NaiveDate {
        &self.default_repo_mat_date
    }
    pub fn country_id(&self) -> &str {
        &self.country_id
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn input_date_formats(&self) -> &Vec<String> {
        &self.input_date_formats
    }
    pub fn output_date_format(&self) -> &str {
        &self.output_date_format
    }
    pub fn input_delimiter(&self) -> &str {
        &self.input_delimiter
    }
    pub fn book_categories(&self) -> &Vec<String> {
        &self.book_categories
    }
    pub fn required_manual_fields_file_path(&self) -> &str {
        &self.required_manual_fields_file
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
        .about("HQLA Aggregator.")
        .version("1.2.2749")
        .author("NPunyashree <punyashree.n@surya-soft.com>")        
        .arg(
            Arg::with_name("input_sec_all_file")
                .long("input-sec-all-file")
                .value_name("FILE")
                .help("Path to the input securities all file")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sec_cblo_file")
                .long("input-sec-cblo-file")
                .value_name("FILE")
                .help("Path to the securities cblo file")
                .required(true)
        )
        .arg(
            Arg::with_name("accrued_day_convention")
                .long("accrued-day-convention")
                .value_name("Accrued day convention")
                .help("Accrued Day Convention to be used.")
                .required(true)
                .possible_values(&["Accrued30/360"])
        )
        .arg(
            Arg::with_name("input_sec_ccil_file")
                .long("input-sec-ccil-file")
                .value_name("FILE")
                .help("Path to the input securities ccil file")
                .required(true)
        )
        .arg(
            Arg::with_name("input_sec_repo_file")
                .long("input-sec-repo-file")
                .value_name("FILE")
                .help("Path to the input securities repo file")
                .required(true)
        )
        .arg(
            Arg::with_name("input_manual_file")
                .long("input-manual-file")
                .value_name("FILE")
                .help("Path to the input manual file")
                .required(true)
        )
        .arg(
            Arg::with_name("input_hqla_file")
                .long("input-hqla-file")
                .value_name("FILE")
                .help("Path to the input hqla file")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("default_repo_mat_date")
                .long("default-repo-mat-date")
                .value_name("DATE")
                .help("The date the program assumes as default for 'repo-mat-date'.")
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
            Arg::with_name("country_id")
                .long("country-id")
                .value_name("SOURCE COUNTRY")
                .help("The source country.")
                .default_value("INDIA")
                .required(false)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("currency")
                .help("The currency.")
                .default_value("CCY")
                .required(false)
        )
        .arg(
            Arg::with_name("input_date_formats")
                .long("input-date-formats")
                .value_name("DATE FORMATS")
                .help("Possible formats of date coming in input.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_date_format")
                .long("output-date-format")
                .value_name("DATE FORMAT")
                .help("The format of date to be used in output.")
                .default_value("%d-%m-%Y")
                .required(false)
        )
        .arg(
            Arg::with_name("input_delimiter")
                .long("input-delimiter")
                .value_name("INPUT DELIMITER")
                .help("The delimiter used in input file.")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::with_name("book_categories")
                .long("book-categories")
                .value_name("BOOK CATEGORIES")
                .help("Book Categories coming in input.")
                .required(true)
        )
        .arg(
            Arg::with_name("required_manual_fields_file")
                .long("required-manual-fields-file")
                .value_name("MANUAL DATA FIELD NAMES")
                .help("Path to config file having required manual field names.")
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
