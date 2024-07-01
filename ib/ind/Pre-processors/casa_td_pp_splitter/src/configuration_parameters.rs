use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub base_input_file_path: String,
    pub cust_master_file_path: String,
    pub map_master_file_path: String,
    pub td_output_file_path: String,
    pub casatd_identifier_file_path: String,
    pub sb_intrate_file_path: String,
    pub ca_output_file_path: String,
    pub sa_output_file_path: String,
    pub splitter_field: i32,
    pub sheet_name: String,
    pub date_fields: String,
    pub header_rows: String,
    pub delimeter_type: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "base_input_file_path: {}",
            self.base_input_file_path()
        );
        info!(
            logger,
            "cust_master_file_path: {}",
            self.cust_master_file_path()
        );
        info!(
            logger,
            "map_master_file_path: {}",
            self.map_master_file_path()
        );
        info!(
            logger,
            "casatd_identifier_file_path: {}",
            self.casatd_identifier_file_path()
        );
        info!(
            logger,
            "sb_intrate_file_path: {}",
            self.sb_intrate_file_path()
        );
        info!(
            logger,
            "td_output_file_path: {}",
            self.td_output_file_path()
        );
        info!(
            logger,
            "ca_output_file_path: {}",
            self.ca_output_file_path()
        );
        info!(
            logger,
            "sa_output_file_path: {}",
            self.sa_output_file_path()
        );
        info!(logger, "splitter_field: {}", self.splitter_field());
        info!(logger, "sheet_name: {}", self.sheet_name());
        info!(logger, "date_fields: {}", self.date_fields());
        info!(logger, "delimeter_type: {}", self.delimeter_type());
        info!(logger, "header_rows: {}", self.header_rows());
    }
}
impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
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
        let base_input_file_path = matches
            .value_of("base_input_file")
            .expect("Error getting `base_input_file_path`.")
            .to_string();
        let cust_master_file_path = matches
            .value_of("cust_master_file")
            .expect("Error getting `cust_master_file_path`.")
            .to_string();
        let map_master_file_path = matches
            .value_of("map_master_file")
            .expect("Error getting `map_master_file_path`.")
            .to_string();
        let casatd_identifier_file_path = matches
            .value_of("casatd_identifier_file")
            .expect("Error getting `casatd_identifier_file_path`.")
            .to_string();
        let sb_intrate_file_path = matches
            .value_of("sb_intrate_file")
            .expect("Error getting `sb_intrate_file_path`.")
            .to_string();
        let td_output_file_path = matches
            .value_of("td_output_file")
            .expect("Error getting `td_output_file_path`.")
            .to_string();
        let ca_output_file_path = matches
            .value_of("ca_output_file")
            .expect("Error getting `ca_output_file_path`.")
            .to_string();
        let sa_output_file_path = matches
            .value_of("sa_output_file")
            .expect("Error getting `sa_output_file_path`.")
            .to_string();
        let splitter_field = matches
            .value_of("splitter_field")
            .expect("Error getting `splitter_field`.")
            .parse()
            .unwrap_or(0);
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name`.")
            .to_string();
        let date_fields = matches
            .value_of("date_fields")
            .expect("Error getting `date_fields`.")
            .to_string();
        let header_rows = matches
            .value_of("header_rows")
            .expect("Error getting `header_rows`.")
            .to_string();
        let delimeter_type = matches
            .value_of("delimeter_type")
            .expect("Error getting `delimeter_type`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        ConfigurationParameters {
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            base_input_file_path,
            cust_master_file_path,
            map_master_file_path,
            casatd_identifier_file_path,
            sb_intrate_file_path,
            td_output_file_path,
            ca_output_file_path,
            sa_output_file_path,
            splitter_field,
            sheet_name,
            date_fields,
            delimeter_type,
            header_rows,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}
// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn base_input_file_path(&self) -> &str {
        &self.base_input_file_path
    }
    pub fn cust_master_file_path(&self) -> &str {
        &self.cust_master_file_path
    }
    pub fn map_master_file_path(&self) -> &str {
        &self.map_master_file_path
    }
    pub fn ca_output_file_path(&self) -> &str {
        &self.ca_output_file_path
    }
    pub fn sa_output_file_path(&self) -> &str {
        &self.sa_output_file_path
    }
    pub fn casatd_identifier_file_path(&self) -> &str {
        &self.casatd_identifier_file_path
    }
    pub fn sb_intrate_file_path(&self) -> &str {
        &self.sb_intrate_file_path
    }
    pub fn td_output_file_path(&self) -> &str {
        &self.td_output_file_path
    }
    pub fn splitter_field(&self) -> &i32 {
        &self.splitter_field
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn date_fields(&self) -> &str {
        &self.date_fields
    }
    pub fn header_rows(&self) -> &str {
        &self.header_rows
    }
    pub fn delimeter_type(&self) -> &str {
        &self.delimeter_type
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}
fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    Command::new(app_name)
        .about("casa_td_pp_splitter")
        .author("Ravindar-01 <ravinar.sr@surya-soft.com>")
        .version("1.0.3612")
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::new("base_input_file")
                .long("base-input-file")
                .value_name("Base Input File Path")
                .help("Path to read Base Input.")
                .required(true)
        )
        .arg(
            Arg::new("cust_master_file")
                .long("cust-master-file")
                .value_name("Cust Master File Path")
                .help("Path to read Cust Master .")
                .required(true)
        )
        .arg(
            Arg::new("map_master_file")
                .long("map-master-file")
                .value_name("Map Master File Path")
                .help("Path to read Map Master.")
                .required(true)
        )
        .arg(
            Arg::new("ca_output_file")
                .long("ca-output-file")
                .value_name("CA Output File Path")
                .help("Path to write CA Output.")
                .required(true)
        )
        .arg(
            Arg::new("sa_output_file")
                .long("sa-output-file")
                .value_name("SA Output File Path")
                .help("Path to write SA Output.")
                .required(true)
        )
        .arg(
            Arg::new("casatd_identifier_file")
                .long("casatd-identifier-file")
                .value_name("CASATD identifier File Path")
                .help("Path to write CASATD identifier.")
                .required(true)
        )
        .arg(
            Arg::new("sb_intrate_file")
                .long("sb-intrate-file")
                .value_name("SB Int Rate File Path")
                .help("Path to write SB Int Rate.")
                .required(true)
        )
        .arg(
            Arg::new("td_output_file")
                .long("td-output-file")
                .value_name("TD Output File Path")
                .help("Path to write TD Output.")
                .required(true)
        )
        .arg(
            Arg::new("splitter_field")
                .long("splitter-field")
                .value_name("Splitter field")
                .help("Path to write Splitter field.")
                .required(true)
        )
        .arg(
            Arg::new("sheet_name")
                .long("sheet-name")
                .value_name("Sheet Name")
                .help("Path to write Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::new("date_fields")
                .long("date-fields")
                .value_name("Date fields")
                .help("Date Fields")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("header_rows")
                .long("header-rows")
                .value_name("Header rows")
                .help("Header rows")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::new("delimeter_type")
                .long("delimeter-type")
                .value_name("delimeter Type")
                .help("delimeter Type")
                .default_value("|")
                .required(false)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::new("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
