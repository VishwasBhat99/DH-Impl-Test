use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    npa_prev_master: String,
    npa_curr_master: String,
    npa_prev_sheet: String,
    npa_curr_sheet: String,
    tenor_master: String,
    tenor_sheet: String,
    alco_master: String,
    alco_sheet: String,
    psl_master: String,
    psl_sheet: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    config_file_path: String,
    consol_ccy: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "consol_ccy: {}", self.consol_ccy());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "alco_master: {}", self.alco_master());
        info!(logger, "alco_sheet: {}", self.alco_sheet());
        info!(logger, "tenor_master: {}", self.tenor_master());
        info!(logger, "tenor_sheet: {}", self.tenor_sheet());
        info!(logger, "psl_master: {}", self.psl_master());
        info!(logger, "psl_sheet: {}", self.psl_sheet());
        info!(logger, "npa_prev_master: {}", self.npa_prev_master());
        info!(logger, "npa_prev_sheet: {}", self.npa_prev_sheet());
        info!(logger, "npa_curr_master: {}", self.npa_curr_master());
        info!(logger, "npa_curr_sheet: {}", self.npa_curr_sheet());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches.value_of("config_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let npa_prev_master = matches
            .value_of("npa_prev_master")
            .expect("Error getting `npa_prev_master`.")
            .to_string();
        let npa_prev_sheet = matches
            .value_of("npa_prev_sheet")
            .expect("Error getting `npa_prev_sheet`.")
            .to_string();
        let npa_curr_master = matches
            .value_of("npa_curr_master")
            .expect("Error getting `npa_curr_master`.")
            .to_string();
        let npa_curr_sheet = matches
            .value_of("npa_curr_sheet")
            .expect("Error getting `npa_curr_sheet`.")
            .to_string();
        let tenor_master = matches
            .value_of("tenor_master")
            .expect("Error getting `tenor_master`.")
            .to_string();
        let tenor_sheet = matches
            .value_of("tenor_sheet")
            .expect("Error getting `tenor_sheet`.")
            .to_string();
        let alco_master = matches
            .value_of("alco_master")
            .expect("Error getting `alco_master`.")
            .to_string();
        let alco_sheet = matches
            .value_of("alco_sheet")
            .expect("Error getting `alco_sheet`.")
            .to_string();
        let psl_master = matches
            .value_of("psl_master")
            .expect("Error getting `psl_master`.")
            .to_string();
        let psl_sheet = matches
            .value_of("psl_sheet")
            .expect("Error getting `psl_sheet`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let consol_ccy = matches.value_of("consol_ccy").unwrap().to_string();

        ConfigurationParameters {
            npa_prev_master,
            npa_prev_sheet,
            npa_curr_master,
            npa_curr_sheet,
            tenor_master,
            tenor_sheet,
            alco_master,
            alco_sheet,
            psl_master,
            psl_sheet,
            output_file_path,
            as_on_date,
            config_file_path,
            consol_ccy,
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
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
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
    pub fn consol_ccy(&self) -> &str {
        &self.consol_ccy
    }
    pub fn tenor_master(&self) -> &str {
        &self.tenor_master
    }
    pub fn tenor_sheet(&self) -> &str {
        &self.tenor_sheet
    }
    pub fn alco_master(&self) -> &str {
        &self.alco_master
    }
    pub fn alco_sheet(&self) -> &str {
        &self.alco_sheet
    }
    pub fn psl_master(&self) -> &str {
        &self.psl_master
    }
    pub fn psl_sheet(&self) -> &str {
        &self.psl_sheet
    }
    pub fn npa_prev_master(&self) -> &str {
        &self.npa_prev_master
    }
    pub fn npa_prev_sheet(&self) -> &str {
        &self.npa_prev_sheet
    }
    pub fn npa_curr_master(&self) -> &str {
        &self.npa_curr_master
    }
    pub fn npa_curr_sheet(&self) -> &str {
        &self.npa_curr_sheet
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .version("1.0.0")
        .about("This program generates output for NPA Multi-Dimensional Account Level Report")
        .arg(
            Arg::with_name("npa_prev_master")
                .long("npa-prev-master")
                .value_name("NPA Previous Month Master File")
                .help("Path to NPA Previous Month Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("npa_prev_sheet")
                .long("npa-prev-sheet")
                .value_name("NPA Previous Month Master Sheet Name")
                .help("Name of NPA Previous Month Master Sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("npa_curr_master")
                .long("npa-curr-master")
                .value_name("NPA Current Month Master File")
                .help("Path to NPA Current Month Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("npa_curr_sheet")
                .long("npa-curr-sheet")
                .value_name("NPA Current Month Master Sheet Name")
                .help("Name of NPA Current Month Master Sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("tenor_master")
                .long("tenor-master")
                .value_name("Tenor Master File")
                .help("Path to Tenor Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("tenor_sheet")
                .long("tenor-sheet")
                .value_name("Tenor Master Sheet Name")
                .help("Name of Tenor Master File sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("alco_master")
                .long("alco-master")
                .value_name("ALCO Master File")
                .help("Path to ALCO Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("alco_sheet")
                .long("alco-sheet")
                .value_name("ALCO Master Sheet Name")
                .help("Name of ALCO Master File sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("psl_master")
                .long("psl-master")
                .value_name("PSL/Non PSL Master File")
                .help("Path to PSL/Non PSL Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("psl_sheet")
                .long("psl-sheet")
                .value_name("PSL/Non PSL Master Sheet Name")
                .help("Name of PSL/Non PSL Master File sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
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
            Arg::with_name("log_file")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
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
            Arg::with_name("config_file")
                .short("i")
                .long("config-file")
                .value_name("FILE")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file")
                .short("o")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(false),
        )
        .arg(
            Arg::with_name("consol_ccy")
                .long("consol-ccy")
                .value_name("CONSOLIDATED CURRENCY")
                .help("Consolidated Currncy")
                .required(true),
        )
        .get_matches()
}
