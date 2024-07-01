use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub gam_file_path: String,
    pub itc_file_path: String,
    pub icv_file_path: String,
    pub lavs_file_path: String,
    pub ivs_file_path: String,
    pub pca_file_path: String,
    pub as_on_date: NaiveDate,
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
        info!(logger, "gam_file_path: {}", self.gam_file_path());
        info!(logger, "itc_file_path: {}", self.itc_file_path());
        info!(logger, "icv_file_path: {}", self.icv_file_path());
        info!(logger, "lavs_file_path: {}", self.lavs_file_path());
        info!(logger, "ivs_file_path: {}", self.ivs_file_path());
        info!(logger, "pca_file_path: {}", self.pca_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let gam_file_path = matches
            .value_of("gam_file_path")
            .expect("Error getting `gam_file_path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
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
        let itc_file_path = matches
            .value_of("itc_file_path")
            .expect("Error getting `itc_file_path` value.")
            .to_string();
        let icv_file_path = matches
            .value_of("icv_file_path")
            .expect("Error getting `icv_file_path` value.")
            .to_string();
        let lavs_file_path = matches
            .value_of("lavs_file_path")
            .expect("Error getting `lavs_file_path` value.")
            .to_string();
        let ivs_file_path = matches
            .value_of("ivs_file_path")
            .expect("Error getting `ivs_file_path` value.")
            .to_string();
        let pca_file_path = matches
            .value_of("pca_file_path")
            .expect("Error getting `pca_file_path` value.")
            .to_string();

        ConfigurationParameters {
            gam_file_path,
            itc_file_path,
            icv_file_path,
            lavs_file_path,
            ivs_file_path,
            pca_file_path,
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
    pub fn gam_file_path(&self) -> &str {
        &self.gam_file_path
    }
    pub fn itc_file_path(&self) -> &str {
        &self.itc_file_path
    }
    pub fn icv_file_path(&self) -> &str {
        &self.icv_file_path
    }
    pub fn lavs_file_path(&self) -> &str {
        &self.lavs_file_path
    }
    pub fn ivs_file_path(&self) -> &str {
        &self.ivs_file_path
    }
    pub fn pca_file_path(&self) -> &str {
        &self.pca_file_path
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
        .about("Program to generate int rate for Loans/OD/SA accounts.")
        .version("1.1.4297")
        .author("srinivasr <srinivas.r@surya-soft.com>")
        .arg(
            Arg::with_name("gam_file_path")
                .long("gam-file-path")
                .value_name("GAM File Path")
                .help("Path to the GAM file.")
                .required(true)
        )
        .arg(
            Arg::with_name("itc_file_path")
                .long("itc-file-path")
                .value_name("ITC File Path")
                .help("Path to the ITC File Path")
                .required(true)
        )
        .arg(
            Arg::with_name("icv_file_path")
                .long("icv-file-path")
                .value_name("ICV File Path")
                .help("Path to the ICV File.")
                .required(true)
        )
        .arg(
            Arg::with_name("lavs_file_path")
                .long("lavs-file-path")
                .value_name("lavs File Path")
                .help("Path to the lavs File.")
                .required(true)
        )
        .arg(
            Arg::with_name("ivs_file_path")
                .long("ivs-file-path")
                .value_name("IVS File Path")
                .help("Path to the IVS File.")
                .required(true)
        )
        .arg(
            Arg::with_name("pca_file_path")
                .long("pca-file-path")
                .value_name("PCA File Path")
                .help("Path to the PCA file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output file path.")
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
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
