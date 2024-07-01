use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub param_file_path: String,
    pub values_file_path: String,
    pub percent_file_path: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub msf_llg: String,
    pub afacility_llg: String,
    pub afacility_percent: String,
    pub ccy: String,
    pub entity: String,
    pub as_on_date: NaiveDate,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "param_file_path: {}", self.param_file_path());
        info!(logger, "percent_file_path: {}", self.percent_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "values_file_path: {}", self.values_file_path());
        info!(logger, "msf_llg: {}", self.msf_llg());
        info!(logger, "afacility_llg: {}", self.afacility_llg());
        info!(logger, "afacility_percent: {}", self.afacility_percent());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "ccy: {}", self.ccy());
        info!(logger, "entity: {}", self.entity());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let param_file_path = matches
            .value_of("param_file_path")
            .expect("Error getting `param_file_path` value.")
            .to_string();
        let percent_file_path = matches
            .value_of("percent_file_path")
            .expect("Error getting `percent_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let values_file_path = matches
            .value_of("values_file_path")
            .expect("Error getting `values_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let msf_llg = matches
            .value_of("msf_llg")
            .expect("Error getting `msf_llg` value.")
            .to_string();
        let afacility_llg = matches
            .value_of("afacility_llg")
            .expect("Error getting `afacility_llg` value.")
            .to_string();
        let afacility_percent = matches
            .value_of("afacility_percent")
            .expect("Error getting `afacility_percent` value.")
            .to_string();
        let ccy = matches
            .value_of("ccy")
            .expect("Error getting `ccy` value.")
            .to_string();
        let entity = matches
            .value_of("entity")
            .expect("Error getting `entity` value.")
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
        ConfigurationParameters {
            param_file_path,
            values_file_path,
            percent_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            msf_llg,
            afacility_llg,
            afacility_percent,
            ccy,
            entity,
            as_on_date,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn param_file_path(&self) -> &str {
        &self.param_file_path
    }
    pub fn percent_file_path(&self) -> &str {
        &self.percent_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn values_file_path(&self) -> &str {
        &self.values_file_path
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn ccy(&self) -> &str {
        &self.ccy
    }
    pub fn entity(&self) -> &str {
        &self.entity
    }
    pub fn msf_llg(&self) -> &str {
        &self.msf_llg
    }
    pub fn afacility_llg(&self) -> &str {
        &self.afacility_llg
    }
    pub fn afacility_percent(&self) -> &str {
        &self.afacility_percent
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app modifies data to conform with the input requirements of Basel MSF PP!")
        .arg(
            Arg::with_name("param_file_path")
                .long("param-file-path")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("percent_file_path")
                .long("percent-file-path")
                .value_name("Percentage File")
                .help("Path to percentage file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("values_file_path")
                .long("values-file-path")
                .value_name("Values File")
                .help("Path to the Values Download file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Diagnostics log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("msf_llg")
                .long("msf-llg")
                .value_name("MSF LLG")
                .help("MSF LLG.")
                .required(true)
        )
        .arg(
            Arg::with_name("ccy")
                .long("ccy")
                .value_name("Currency")
                .help("Currency Value.")
                .required(true)
        )
        .arg(
            Arg::with_name("entity")
                .long("entity")
                .value_name("Entity")
                .help("Entity Value.")
                .required(true)
        )
        .arg(
            Arg::with_name("afacility_llg")
                .long("afacility-llg")
                .value_name("Afacility llg")
                .help(" Value for Afacility llg.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("afacility_percent")
                .long("afacility-percent")
                .value_name("Afacility percentage")
                .help("Value for afacility percentage.")
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
