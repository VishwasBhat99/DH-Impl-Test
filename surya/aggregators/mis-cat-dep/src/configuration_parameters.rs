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
    as_on_date: NaiveDate,
    input_file_path: String,
    cust_master_file_path: String,
    cust_master_delimiter: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    display_ccy: String,
    is_perf_diagnostics_enabled: bool,
    rbi_cat_def_file_path: String,
    rbi_cat_map_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "cust_master_file: {}", self.cust_master_file_path());
        info!(
            logger,
            "cust_master_delimiter: {}",
            self.cust_master_delimiter()
        );
        info!(logger, "display_ccy: {}", self.display_ccy());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "rbi_cat_def_file_path: {}",
            self.rbi_cat_def_file_path()
        );
        info!(
            logger,
            "rbi_cat_map_file_path: {}",
            self.rbi_cat_map_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let display_ccy = matches
            .value_of("display_ccy")
            .expect("Error getting `display ccy`.")
            .to_string();
        let cust_master_file_path = matches
            .value_of("cust_master_file_path")
            .expect("Error getting `cust_master_file_path`.")
            .to_string();
        let cust_master_delimiter = matches
            .value_of("cust_master_delimiter")
            .expect("Error getting `cust_master_delimiter`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
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
        let rbi_cat_def_file_path = matches
            .value_of("rbi_cat_def_file_path")
            .expect("Error getting `rbi_cat_def_file_path`.")
            .to_string();
        let rbi_cat_map_file_path = matches
            .value_of("rbi_cat_map_file_path")
            .expect("Error getting `rbi_cat_map_file_path`.")
            .to_string();

        ConfigurationParameters {
            as_on_date,
            input_file_path,
            cust_master_file_path,
            cust_master_delimiter,
            output_file_path,
            display_ccy,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            rbi_cat_def_file_path,
            rbi_cat_map_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn cust_master_file_path(&self) -> &str {
        &self.cust_master_file_path
    }
    pub fn cust_master_delimiter(&self) -> &str {
        &self.cust_master_delimiter
    }
    pub fn display_ccy(&self) -> &str {
        &self.display_ccy
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn rbi_cat_def_file_path(&self) -> &str {
        &self.rbi_cat_def_file_path
    }
    pub fn rbi_cat_map_file_path(&self) -> &str {
        &self.rbi_cat_map_file_path
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Category of deposits program.")
        .version("1.1.4811")
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path of input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_master_file_path")
                .long("cust-master-file-path")
                .value_name("Cust Master File Path")
                .help("Path of cust master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_master_delimiter")
                .long("cust-master-delimiter")
                .value_name("Cust Master Delimiter")
                .help("Cust Master File Separator/Delimiter.")
                .default_value("~#~")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path of output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("display_ccy")
                .long("display-ccy")
                .value_name("Currency file path")
                .help("Display currency.")
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
        .arg(
            Arg::with_name("rbi_cat_def_file_path")
                .long("rbi-cat-def-file-path")
                .value_name("RBI Cat Def File Path")
                .help("Path to RBI Cat Def File.")
                .required(true)
        )
        .arg(
            Arg::with_name("rbi_cat_map_file_path")
                .long("rbi-cat-map-file-path")
                .value_name("RBI Cat Map File Path")
                .help("Path to RBI Cat Map File.")
                .required(true)
        )
        .get_matches()
}
