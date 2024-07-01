use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file: String,
    pub alm_master_file: String,
    pub fin_code_map_file: String,
    pub alm_master_sheet_name: String,
    pub gl_moc_entry_file: String,
    pub gl_moc_sheet_name: String,
    pub fin_code_sheet_name: String,
    pub gl_ex_master: String,
    pub as_on_date: String,
    pub output_file_path: String,
    pub currency: String,
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
        info!(logger, "alm_master_file: {}", self.alm_master_file());
        info!(logger, "fin_code_map_file: {}", self.fin_code_map_file());
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(logger, "gl_moc_entry_file: {}", self.gl_moc_entry_file());
        info!(logger, "gl_moc_sheet_name: {}", self.gl_moc_sheet_name());
        info!(
            logger,
            "fin_code_sheet_name: {}",
            self.fin_code_sheet_name()
        );
        info!(logger, "gl_exclude_master: {}", self.gl_ex_master());
        info!(logger, "currency: {}", self.currency());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let as_on_date = matches
            .value_of("as_on_date")
            .expect("Error getting `as_on_date` value.")
            .to_string();
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
        let alm_master_file = matches
            .value_of("alm_master_file")
            .expect("Error getting `alm_master_file` value.")
            .to_string();
        let fin_code_map_file = matches
            .value_of("fin_code_map_file")
            .expect("Error getting `fin_code_map_file` value.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        let gl_moc_entry_file = matches
            .value_of("gl_moc_entry_file")
            .expect("Error getting `gl_moc_entry_file` value.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency` value.")
            .to_string();
        let gl_moc_sheet_name = matches
            .value_of("gl_moc_sheet_name")
            .expect("Error getting `gl_moc_sheet_name` value.")
            .to_string();
        let fin_code_sheet_name = matches
            .value_of("fin_code_sheet_name")
            .expect("Error getting `fin_code_sheet_name` value.")
            .to_string();
        let gl_ex_master = matches
            .value_of("gl_ex_master")
            .expect("Error getting `gl_ex_master` value.")
            .to_string();
        ConfigurationParameters {
            input_file,
            alm_master_file,
            fin_code_map_file,
            alm_master_sheet_name,
            gl_moc_entry_file,
            gl_moc_sheet_name,
            fin_code_sheet_name,
            gl_ex_master,
            as_on_date,
            output_file_path,
            currency,
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
    pub fn alm_master_file(&self) -> &str {
        &self.alm_master_file
    }
    pub fn fin_code_map_file(&self) -> &str {
        &self.fin_code_map_file
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn gl_moc_entry_file(&self) -> &str {
        &self.gl_moc_entry_file
    }
    pub fn gl_moc_sheet_name(&self) -> &str {
        &self.gl_moc_sheet_name
    }
    pub fn fin_code_sheet_name(&self) -> &str {
        &self.fin_code_sheet_name
    }
    pub fn gl_ex_master(&self) -> &str {
        &self.gl_ex_master
    }
    pub fn as_on_date(&self) -> &str {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn currency(&self) -> &str {
        &self.currency
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
        .about("Pre Processor for Gl Balance!")
        .version("1.0.2464")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("input_file")
                .long("input-file")
                .value_name("input_file")
                .help("Path to Master input file.")
                .required(true)
        )
        .arg(
            Arg::new("alm_master_file")
                .long("alm-master-file")
                .value_name("alm_master_file")
                .help("Path to ALM Master input File.")
                .required(true)
        )
        .arg(
            Arg::new("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("alm_master_sheet_name")
                .default_value("Sheet1")
                .help("Sheet name of ALM Master input File.")
                .required(false)
        )
        .arg(
            Arg::new("fin_code_map_file")
                .long("fin-code-map-file")
                .value_name("fin_code_map_file")
                .help("Path to Finance Code MCommanding input file.")
                .required(true)
        )
        .arg(
            Arg::new("fin_code_sheet_name")
                .long("fin-code-sheet-name")
                .value_name("fin_code_sheet_name")
                .help("Sheet name of Finance Code MCommanding input file.")
                .required(false)
        )
        .arg(
            Arg::new("gl_moc_entry_file")
                .long("gl-moc-file")
                .value_name("gl_moc_entry_file")
                .help("Path to GL MOC Entry input file.")
                .required(true)
        )
        .arg(
            Arg::new("gl_moc_sheet_name")
                .long("gl-moc-sheet-name")
                .value_name("gl_moc_sheet_name")
                .default_value("Sheet1")
                .help("Sheet name of GL MOC Entry input file.")
                .required(false)
        )
        .arg(
            Arg::new("currency")
                .long("currency")
                .value_name("currency")
                .help("Currency.")
                .default_value("INR")
                .required(false)
        )
        .arg(
            Arg::new("gl_ex_master")
                .long("gl-ex-master")
                .value_name("gl_ex_master")
                .help("GL Exclude Master File Path.")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to the output file.")
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
