use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file: String,
    pub mis_input_file: String,
    pub path_sep: String,
    pub input_cashflow_file: String,
    pub mapping_master_file: String,
    pub cashflow_sheet: String,
    pub master_sheet: String,
    pub as_on_date: rbdate::NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "mis_input_file: {}", self.mis_input_file());
        info!(logger, "path_sep: {}", self.path_sep());
        info!(
            logger,
            "input_cashflow_file: {}",
            self.input_cashflow_file()
        );
        info!(
            logger,
            "mapping_master_file: {}",
            self.mapping_master_file()
        );
        info!(logger, "cashflow_sheet: {:?}", self.cashflow_sheet());
        info!(logger, "master_sheet: {:?}", self.master_sheet());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();
        let mis_input_file = matches
            .value_of("mis_input_file")
            .expect("Error getting `mis_input_file` value.")
            .to_string();
        let path_sep = matches
            .value_of("path_sep")
            .expect("Error getting `path_sep` value.")
            .to_string();
        let input_cashflow_file = matches
            .value_of("input_cashflow_file")
            .expect("Error getting `input_cashflow_file` value.")
            .to_string();
        let mapping_master_file = matches
            .value_of("mapping_master_file")
            .expect("Error getting `mapping_master_file` value.")
            .to_string();
        let cashflow_sheet = matches
            .value_of("cashflow_sheet")
            .expect("Error getting `cashflow_sheet` value.")
            .to_string();
        let master_sheet = matches
            .value_of("master_sheet")
            .expect("Error getting `master_sheet` value.")
            .to_string();

        ConfigurationParameters {
            input_file,
            mis_input_file,
            path_sep,
            input_cashflow_file,
            mapping_master_file,
            cashflow_sheet,
            as_on_date,
            master_sheet,
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
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn mis_input_file(&self) -> &str {
        &self.mis_input_file
    }
    pub fn path_sep(&self) -> &str {
        &self.path_sep
    }
    pub fn input_cashflow_file(&self) -> &str {
        &self.input_cashflow_file
    }
    pub fn mapping_master_file(&self) -> &str {
        &self.mapping_master_file
    }
    pub fn cashflow_sheet(&self) -> &str {
        &self.cashflow_sheet
    }
    pub fn master_sheet(&self) -> &str {
        &self.master_sheet
    }
    pub fn as_on_date(&self) -> rbdate::NaiveDate {
        self.as_on_date
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

fn get_eligible_arguments_for_command(command_name: &str) -> clap::ArgMatches {
    Command::new(command_name)
        .about("Pre Processor for Contractual Outflow!")
        .version("1.1.2892")
        .author("Saurabh Singh <saurabh.s@surya-soft.com>")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("input_file")
                .help("Path to Master Input File.")
                .required(true)
        )
        .arg(
            Arg::new("mis_input_file")
                .long("mis-input-file")
                .value_name("mis_input_file")
                .help("Path to Mis Input File.")
                .required(true)
        )
        .arg(
            Arg::new("path_sep")
                .long("path-sep")
                .value_name("path_sep")
                .help("Path sep for Input File.")
                .possible_values(&["\\","/"])
                .default_value("/")
                .required(false)
        )
        .arg(
            Arg::new("input_cashflow_file")
                .long("input-cf-file")
                .value_name("input_cashflow_file")
                .help("Path to Input Cashflow File.")
                .required(true)
        )
        .arg(
            Arg::new("mapping_master_file")
                .long("mapping-master-file")
                .value_name("mapping_master_file")
                .help("Path to Mapping Master Input File.")
                .required(true)
        )
        .arg(
            Arg::new("cashflow_sheet")
                .long("cashflow-sheet")
                .value_name("cashflow_sheet")
                .help("Path to Cashflow Input Sheet.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("master_sheet")
                .long("master-sheet")
                .value_name("master_sheet")
                .help("Path to Master Input Sheet.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to the Output File.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs to")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics to")
                .required(true)
        )
        .arg(
            Arg::new("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
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
        .get_matches()
}
