use clap::{Arg, Command};
use slog::Logger;

pub fn get_configuration_parameters(command_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_command(command_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub input_metadata_file: String,
    pub as_on_date: rbdate::NaiveDate,
    pub output_file_path: String,
    pub required_fields_file: String,
    pub date_prefix: String,
    pub date_suffix: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "input_metadata_file: {}",
            self.input_metadata_file()
        );
        info!(
            logger,
            "required_fields_file: {}",
            self.required_fields_file()
        );
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "date_prefix: {}", self.date_prefix());
        info!(logger, "date_suffix: {}", self.date_suffix());
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
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();
        let input_metadata_file = matches
            .value_of("input_metadata_file")
            .expect("Error getting `input_metadata_file` value.")
            .to_string();
        let required_fields_file = matches
            .value_of("required_fields_file")
            .expect("Error getting `required_fields_file` value.")
            .to_string();
        let date_prefix = matches
            .value_of("date_prefix")
            .expect("Error getting `date_prefix` value.")
            .to_string();
        let date_suffix = matches
            .value_of("date_suffix")
            .expect("Error getting `date_suffix` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            input_metadata_file,
            required_fields_file,
            date_prefix,
            date_suffix,
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
    pub fn input_metadata_file(&self) -> &str {
        &self.input_metadata_file
    }
    pub fn required_fields_file(&self) -> &str {
        &self.required_fields_file
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
    pub fn date_prefix(&self) -> &str {
        &self.date_prefix
    }
    pub fn date_suffix(&self) -> &str {
        &self.date_suffix
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
        .about("Treasury CF program for closed account extraction")
        .version("1.1.4339")
        .author("Sachin Mulgir <sachin.m@surya-soft.com>")
        .arg(
            Arg::new("input_file_path")
                .long("input-file")
                .value_name("input_file")
                .help("Path to Input Cashflow File.")
                .required(true)
        )
        .arg(
            Arg::new("input_metadata_file")
                .long("input-metadata-file")
                .value_name("input_metadata_file")
                .help("Path to Input Metadata File.")
                .required(true)
        )
        .arg(
            Arg::new("required_fields_file")
                .long("required-fields-file")
                .value_name("required_fields_file")
                .help("Path to Required Fields File.")
                .required(true)
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
            Arg::new("date_prefix")
                .long("date-prefix")
                .value_name("DATE PREFIX")
                .help("Path before the date")
                .required(true)
        )
        .arg(
            Arg::new("date_suffix")
                .long("date-suffix")
                .value_name("DATE SUFFIX")
                .help("Path after the date")
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
