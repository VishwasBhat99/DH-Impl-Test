use clap::{Arg, Command};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}
pub struct ConfigurationParameters {
    pub as_on_date: NaiveDate,
    pub def_comp_mat_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub investment_file_path: String,
    pub bgl_cgl_file_path: String,
    pub map_master_file_path: String,
    pub sheet_name: String,
    pub investment_output_file_path: String,
    pub date_fields: String,
    pub header_rows: String,
    pub delimeter_type: String,
    pub instrument_type_data: Vec<String>,
    pub currency: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}
impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "def_comp_mat_date: {:?}", self.def_comp_mat_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "investment_file_path: {}",
            self.investment_file_path()
        );
        info!(logger, "bgl_cgl_file_path: {}", self.bgl_cgl_file_path());
        info!(
            logger,
            "map_master_file_path: {}",
            self.map_master_file_path()
        );
        info!(
            logger,
            "investment_output_file_path: {}",
            self.investment_output_file_path()
        );
        info!(logger, "sheet_name: {}", self.sheet_name());
        info!(logger, "date_fields: {}", self.date_fields());
        info!(logger, "instrument_type_data: {:?}", self.instrument_type_data());
        info!(logger, "header_rows: {}", self.header_rows());
        info!(logger, "delimeter_type: {}", self.delimeter_type());
        info!(logger, "currency: {}", self.currency());
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
        let def_comp_mat_date = date_parser.parse(
            matches
                .value_of("def_comp_mat_date")
                .expect("`def-comp-mat-date` not well-formatted."),
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
        let investment_file_path = matches
            .value_of("investment_file")
            .expect("Error getting `investment_file_path`.")
            .to_string();
        let bgl_cgl_file_path = matches
            .value_of("bgl_cgl_file")
            .expect("Error getting `bgl_cgl_file_path`.")
            .to_string();
        let map_master_file_path = matches
            .value_of("map_master_file")
            .expect("Error getting `map_master_file_path`.")
            .to_string();
        let investment_output_file_path = matches
            .value_of("investment_output_file")
            .expect("Error getting `investment_output_file_path`.")
            .to_string();
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name`.")
            .to_string();
        let date_fields = matches
            .value_of("date_fields")
            .expect("Error getting `date_fields`.")
            .to_string();
        let instrument_type_data: Vec<String> = matches
            .value_of("instrument_type_data")
            .expect("Error getting `instrument_type_data`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let header_rows = matches
            .value_of("header_rows")
            .expect("Error getting `header_rows`.")
            .to_string();
        let delimeter_type = matches
            .value_of("delimeter_type")
            .expect("Error getting `delimeter_type`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        ConfigurationParameters {
            as_on_date,
            def_comp_mat_date,
            log_file_path,
            diagnostics_file_path,
            investment_file_path,
            map_master_file_path,
            bgl_cgl_file_path,
            investment_output_file_path,
            sheet_name,
            date_fields,
            instrument_type_data,
            delimeter_type,
            currency,
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
    pub fn investment_file_path(&self) -> &str {
        &self.investment_file_path
    }
    pub fn bgl_cgl_file_path(&self) -> &str {
        &self.bgl_cgl_file_path
    }
    pub fn map_master_file_path(&self) -> &str {
        &self.map_master_file_path
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn investment_output_file_path(&self) -> &str {
        &self.investment_output_file_path
    }
    pub fn date_fields(&self) -> &str {
        &self.date_fields
    }
    pub fn instrument_type_data(&self) -> &Vec<String> {
        &self.instrument_type_data
    }
    pub fn header_rows(&self) -> &str {
        &self.header_rows
    }
    pub fn delimeter_type(&self) -> &str {
        &self.delimeter_type
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn def_comp_mat_date(&self) -> &NaiveDate {
        &self.def_comp_mat_date
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
        .about("Pre Processor for Investments (IB IND)")
        .author("Ravindar-01 <ravindar.sr@surya-soft.com>")
        .version("1.0.5307")
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
            Arg::new("investment_file")
                .long("investment-file")
                .value_name("Investment File Path")
                .help("Path to read Invsetment file.")
                .required(true)
        )
        .arg(
            Arg::new("bgl_cgl_file")
                .long("bgl-cgl-file")
                .value_name("BGL CGL File Path")
                .help("Path to write BGL CGL File.")
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
            Arg::new("investment_output_file")
                .long("investment-output-file")
                .value_name("BL Output File Path")
                .help("Path to write BL Output.")
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
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::new("instrument_type_data")
                .long("instrument-type-data")
                .value_name("Instrument Type Data")
                .help("Instrument Type Data")
                .default_value("")
                .required(true)
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
            Arg::new("currency")
                .long("currency")
                .value_name("currency")
                .help("currency")
                .default_value("INR")
                .required(false)
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
            Arg::new("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
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
        .arg(
            Arg::new("def_comp_mat_date")
                .long("def-comp-mat-date")
                .value_name("DEF_COMP_MAT_DATE")
                .help("The default date that is to be stamped for Computed Maturity Date")
                .default_value("31-12-2100")
                .required(false)
        )
        .get_matches()
}
