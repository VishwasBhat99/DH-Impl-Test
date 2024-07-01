use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub income_master_file: String,
    pub aggr_input_file: String,
    pub as_on_date: rbdate::NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub common_cap_file: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "income_master_file: {}", self.income_master_file());
        info!(logger, "aggr_input_file: {}", self.aggr_input_file());
        info!(logger, "common_cap_file: {}", self.common_cap_file());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
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
        let aggr_input_file = matches
            .value_of("aggr_input_file")
            .expect("Error getting `aggr_input_file` value.")
            .to_string();
        let income_master_file = matches
            .value_of("income_master_file")
            .expect("Error getting `income_master_file` value.")
            .to_string();
        let common_cap_file = matches
            .value_of("common_cap_file")
            .expect("Error getting `common_cap_file` value.")
            .to_string();

        ConfigurationParameters {
            income_master_file,
            aggr_input_file,
            common_cap_file,
            as_on_date,
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
    pub fn income_master_file(&self) -> &str {
        &self.income_master_file
    }
    pub fn aggr_input_file(&self) -> &str {
        &self.aggr_input_file
    }
    pub fn as_on_date(&self) -> &rbdate::NaiveDate {
        &self.as_on_date
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn common_cap_file(&self) -> &str {
        &self.common_cap_file
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
        .about("Program Income Expense Adjuster Program!")
        .version("1.0.3371")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::new("income_master_file")
                .long("income-master-file")
                .value_name("income_master_file")
                .help("Path to Income Master File.")
                .required(true)
        )
        .arg(
            Arg::new("aggr_input_file")
                .long("aggr-input-file")
                .value_name("aggr_input_file")
                .help("Path to Input Aggr File.")
                .required(true)
        )
        .arg(
            Arg::new("common_cap_file")
                .long("common-cap-file")
                .value_name("common_cap_file")
                .help("Path to Common Capitalization File.")
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
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file")
                .default_value("none")
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
