use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    mapping_master_file_path: String,
    recon_file_path: String,
    gstt_extraction_file_path: String,
    output_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "mapping_maaster_path: {}",
            self.mapping_master_file_path()
        );
        info!(logger, "recon_file_path: {}", self.recon_file_path());
        info!(
            logger,
            "gstt_file_path: {}",
            self.gstt_extraction_file_path()
        );
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let mapping_master_file_path = matches
            .value_of("mapping_master_file_path")
            .expect("Error getting `mapping_master_file_path`.")
            .to_string();
        let recon_file_path = matches
            .value_of("recon_file_path")
            .expect("Error getting `recon_file_path`.")
            .to_string();
        let gstt_extraction_file_path = matches
            .value_of("gstt_extraction_file_path")
            .expect("Error getting `gstt_extraction_file_path`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
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
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            mapping_master_file_path,
            recon_file_path,
            gstt_extraction_file_path,
            output_file,
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
    pub fn mapping_master_file_path(&self) -> &str {
        &self.mapping_master_file_path
    }
    pub fn recon_file_path(&self) -> &str {
        &self.recon_file_path
    }
    pub fn gstt_extraction_file_path(&self) -> &str {
        &self.gstt_extraction_file_path
    }
    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
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
        .about("PP Main diff Program for SIB!!")
        .version("1.0.4888")
        .author("Sonali<sonali.s@surya-soft.com>")
        .arg(
            Arg::with_name("mapping_master_file_path")
                .long("mapping-file-path")
                .value_name("MAPPING FILE")
                .help("Path to Mapping file.")
                .required(true)
        )
        .arg(
            Arg::with_name("recon_file_path")
                .long("recon-file")
                .value_name("RECON FILE")
                .help("Path to Recon File.")
                .required(true)
        )
        .arg(
            Arg::with_name("gstt_extraction_file_path")
                .long("gstt-extraction-file")
                .value_name("gstt EXTRACTION")
                .help("Path to gstt extraction File.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("OUTPUT")
                .help("Path to Output File.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("LOG_FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("DIAGLOG_FILE")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG_LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS_FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}