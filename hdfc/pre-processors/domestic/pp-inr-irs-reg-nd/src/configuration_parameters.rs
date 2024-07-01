use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub inr_irs_infile_path: String,
    pub ref_file_path: String,
    pub inr_irs_nd_file_path: String,
    pub inr_irs_outfile_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub sheet_name: String,
    pub as_on_date: NaiveDate,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "inr_ins_infile: {}", self.inr_irs_infile_path());
        info!(logger, "ref_file: {}", self.ref_file_path());
        info!(logger, "inr_irs_nd_file: {}", self.inr_irs_nd_file_path());
        info!(logger, "inr_irs_outfile: {}", self.inr_irs_outfile_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "sheet_name: {}", self.sheet_name());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let inr_irs_infile_path = matches
            .value_of("inr-irs-infile")
            .expect("Error getting `inr-irs-infile`.")
            .to_string();
        let ref_file_path = matches
            .value_of("ref-file")
            .expect("Error getting `ref-file`.")
            .to_string();
        let inr_irs_nd_file_path = matches
            .value_of("inr-irs-nd-file")
            .expect("Error getting `inr_irs_nd_file_path`.")
            .to_string();
        let inr_irs_outfile_path = matches
            .value_of("inr-irs-outfile")
            .expect("Error getting `inr_irs_outfile_path`.")
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

        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet name`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );

        ConfigurationParameters {
            inr_irs_infile_path,
            ref_file_path,
            inr_irs_nd_file_path,
            inr_irs_outfile_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            sheet_name,
            as_on_date,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn inr_irs_infile_path(&self) -> &str {
        &self.inr_irs_infile_path
    }
    pub fn ref_file_path(&self) -> &str {
        &self.ref_file_path
    }
    pub fn inr_irs_nd_file_path(&self) -> &str {
        &self.inr_irs_nd_file_path
    }
    pub fn inr_irs_outfile_path(&self) -> &str {
        &self.inr_irs_outfile_path
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
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("pp generator for inr irs red nd")
        .arg(
            Arg::new("inr-irs-infile")
                .long("inr-irs-infile")
                .value_name("INR INS INPATH")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::new("ref-file")
                .long("ref-file")
                .value_name("INR INS INPATH")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::new("inr-irs-nd-file")
                .long("inr-irs-nd-file")
                .value_name("inr irs outFile")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("inr-irs-outfile")
                .long("inr-irs-outfile")
                .value_name("inr irs outFile")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::new("log_file")
                .long("log-file")
                .value_name("Log File.")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::new("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics Log File")
                .help("Path to write diagnostics logs.")
                .required(true)
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
            Arg::new("sheet_name")
               .long("sheet-name")
                .value_name("sheet name")
                .help("sheet name is missing")
                .required(true)
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
