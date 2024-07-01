use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    bkt_def_file_path: String,
    mapping_master_file_path: String,
    mapping_master_sheet_name: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "bkt_def_file: {}", self.bkt_def_file_path());
        info!(
            logger,
            "mapping_master_file: {}",
            self.mapping_master_file_path()
        );
        info!(
            logger,
            "mapping_master_sheet_name: {}",
            self.mapping_master_sheet_name()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let bkt_def_file_path = matches
            .value_of("bkt_def_file_path")
            .expect("Error getting `bkt_def_file_path`.")
            .to_string();
        let mapping_master_file_path = matches
            .value_of("mapping_master_file_path")
            .expect("Error getting `mapping_master_file_path`.")
            .to_string();
        let mapping_master_sheet_name = matches
            .value_of("mapping_master_sheet_name")
            .expect("Error getting `mapping_master_sheet_name`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
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
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            bkt_def_file_path,
            mapping_master_file_path,
            mapping_master_sheet_name,
            as_on_date,
            output_file_path,
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
    pub fn bkt_def_file_path(&self) -> &str {
        &self.bkt_def_file_path
    }
    pub fn mapping_master_file_path(&self) -> &str {
        &self.mapping_master_file_path
    }
    pub fn mapping_master_sheet_name(&self) -> &str {
        &self.mapping_master_sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Cashflow derivation for TL Prepayment.")
        .author("ravindar-01 <ravindar.sr@surya-soft.com>")
        .version("1.0.4196")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the Input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("bkt_def_file_path")
                .long("bkt-def-file")
                .value_name("Bkt Def file")
                .help("Path to the Bkt Def file.")
                .required(true)
        )
        .arg(
            Arg::with_name("mapping_master_file_path")
                .long("mapping-master-file")
                .value_name("Mapping Master file ")
                .help("Path to the Mapping Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("mapping_master_sheet_name")
                .long("mapping-master-sheet-name")
                .value_name("Mapping Master Sheet Name")
                .help("Path to the Mapping Master Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
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
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
