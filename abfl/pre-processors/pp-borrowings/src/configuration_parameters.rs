use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file: String,
    pub input_cashflow_file: String,
    pub benpos_data_file: String,
    pub benpos_mapping_file: String,
    pub floating_mapping_file: String,
    pub benpos_data_sheet: String,
    pub benpos_mapping_sheet: String,
    pub floating_mapping_sheet: String,
    pub borrowing_update_type_master: String,
    pub borrowing_update_type_master_sheet: String,
    pub as_on_date: rbdate::NaiveDate,
    pub benpos_column_count: usize,
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
        info!(
            logger,
            "input_cashflow_file: {}",
            self.input_cashflow_file()
        );
        info!(logger, "benpos_data_file: {}", self.benpos_data_file());
        info!(
            logger,
            "benpos_mapping_file: {}",
            self.benpos_mapping_file()
        );
        info!(
            logger,
            "floating_mapping_file: {}",
            self.floating_mapping_file()
        );
        info!(logger, "benpos_data_sheet: {:?}", self.benpos_data_sheet());
        info!(
            logger,
            "benpos_mapping_sheet: {:?}",
            self.benpos_mapping_sheet()
        );
        info!(
            logger,
            "floating_mapping_sheet: {:?}",
            self.floating_mapping_sheet()
        );
        info!(
            logger,
            "borrowing_update_type_master: {}",
            self.borrowing_update_type_master()
        );
        info!(
            logger,
            "borrowing_update_type_master_sheet: {:?}",
            self.borrowing_update_type_master_sheet()
        );
        info!(
            logger,
            "benpos_column_count: {}",
            self.benpos_column_count()
        );
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
        let input_cashflow_file = matches
            .value_of("input_cashflow_file")
            .expect("Error getting `input_cashflow_file` value.")
            .to_string();
        let benpos_data_file = matches
            .value_of("benpos_data_file")
            .expect("Error getting `benpos_data_file` value.")
            .to_string();
        let benpos_mapping_file = matches
            .value_of("benpos_mapping_file")
            .expect("Error getting `benpos_mapping_file` value.")
            .to_string();
        let floating_mapping_file = matches
            .value_of("floating_mapping_file")
            .expect("Error getting `floating_mapping_file` value.")
            .to_string();
        let benpos_data_sheet = matches
            .value_of("benpos_data_sheet")
            .expect("Error getting `benpos_data_sheet` value.")
            .to_string();
        let benpos_mapping_sheet = matches
            .value_of("benpos_mapping_sheet")
            .expect("Error getting `benpos_mapping_sheet` value.")
            .to_string();
        let floating_mapping_sheet = matches
            .value_of("floating_mapping_sheet")
            .expect("Error getting `floating_mapping_sheet` value.")
            .to_string();
        let borrowing_update_type_master = matches
            .value_of("borrowing_update_type_master")
            .expect("Error getting `borrowing_update_type_master` value.")
            .to_string();
        let borrowing_update_type_master_sheet = matches
            .value_of("borrowing_update_type_master_sheet")
            .expect("Error getting `borrowing_update_type_master_sheet` value.")
            .to_string();
        let benpos_column_count = matches
            .value_of("benpos_column_count")
            .expect("Error getting `benpos_column_count` value.")
            .to_string()
            .parse::<usize>()
            .expect("Error getting `benpos_column_count` value.");

        ConfigurationParameters {
            input_file,
            input_cashflow_file,
            benpos_data_file,
            benpos_mapping_file,
            floating_mapping_file,
            benpos_data_sheet,
            benpos_mapping_sheet,
            as_on_date,
            floating_mapping_sheet,
            borrowing_update_type_master,
            borrowing_update_type_master_sheet,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            benpos_column_count,
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
    pub fn input_cashflow_file(&self) -> &str {
        &self.input_cashflow_file
    }
    pub fn benpos_data_file(&self) -> &str {
        &self.benpos_data_file
    }
    pub fn benpos_mapping_file(&self) -> &str {
        &self.benpos_mapping_file
    }
    pub fn floating_mapping_file(&self) -> &str {
        &self.floating_mapping_file
    }
    pub fn benpos_data_sheet(&self) -> &str {
        &self.benpos_data_sheet
    }
    pub fn benpos_column_count(&self) -> &usize {
        &self.benpos_column_count
    }
    pub fn benpos_mapping_sheet(&self) -> &str {
        &self.benpos_mapping_sheet
    }
    pub fn floating_mapping_sheet(&self) -> &str {
        &self.floating_mapping_sheet
    }
    pub fn borrowing_update_type_master(&self) -> &str {
        &self.borrowing_update_type_master
    }
    pub fn borrowing_update_type_master_sheet(&self) -> &str {
        &self.borrowing_update_type_master_sheet
    }
    pub fn as_on_date(&self) -> &rbdate::NaiveDate {
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

fn get_eligible_arguments_for_command(command_name: &str) -> clap::ArgMatches {
    Command::new(command_name)
        .about("Pre Processor for Borrowings!")
        .version("1.0.3586")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("input_file")
                .help("Path to Master Input File.")
                .required(true)
        )
        .arg(
            Arg::new("input_cashflow_file")
                .long("input-cf-file")
                .value_name("input_cashflow_file")
                .help("Path to Input Cashflow File.")
                .required(true)
        )
        .arg(
            Arg::new("benpos_data_file")
                .long("benpos-data-file")
                .value_name("benpos_data_file")
                .help("Path to Benpos Data Input File.")
                .required(true)
        )
        .arg(
            Arg::new("benpos_mapping_file")
                .long("benpos-mapping-file")
                .value_name("benpos_mapping_file")
                .help("Path to Benpos Mapping Input File.")
                .required(true)
        )
        .arg(
            Arg::new("floating_mapping_file")
                .long("floating-mapping-file")
                .value_name("floating_mapping_file")
                .help("Path to Floating Mapping Input File.")
                .required(true)
        )
        .arg(
            Arg::new("benpos_data_sheet")
                .long("benpos-data-sheet")
                .value_name("benpos_data_sheet")
                .help("Path to Benpos Data Input Sheet.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("benpos_mapping_sheet")
                .long("benpos-mapping-sheet")
                .value_name("benpos_mapping_sheet")
                .help("Path to Benpos Mapping Input Sheet.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("benpos_column_count")
                .long("benpos-col-count")
                .value_name("benpos_column_count")
                .help("Number of Columns in Benpos Data File.")
                .default_value("74")
                .required(false)
        )
        .arg(
            Arg::new("floating_mapping_sheet")
                .long("floating-mapping-sheet")
                .value_name("floating_mapping_sheet")
                .help("Path to Floating Mapping Input Sheet.")
                .default_value("Sheet1")
                .required(false)
        )
        .arg(
            Arg::new("borrowing_update_type_master")
                .long("borrowing-update-type-master")
                .value_name("borrowing_update_type_master")
                .help("Path to Borrowing Update type Master File.")
                .required(true)
        )
        .arg(
            Arg::new("borrowing_update_type_master_sheet")
                .long("borrowing-update-type-master-sheet")
                .value_name("borrowing_update_type_master_sheet")
                .help("Path to Borrowing Update type Master Sheet.")
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
