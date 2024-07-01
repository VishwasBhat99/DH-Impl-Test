use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    tcfsl_npa_file: String,
    finnone_fsl_file_path: String,
    finnone_cashflow_file_path: String,
    writeoff_merged_file_path: String,
    output_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    tcfsl_npa_sheet_name: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "tcfsl_npa_file: {}", self.tcfsl_npa_file());
        info!(
            logger,
            "finnone_fsl_file_path: {}",
            self.finnone_fsl_file_path()
        );
        info!(
            logger,
            "writeoff_merged_file: {}",
            self.writeoff_merged_file_path()
        );
        info!(
            logger,
            "finnone_cashflow_file_path: {}",
            self.finnone_cashflow_file_path()
        );
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "tcfsl_npa_sheet_name: {}",
            self.tcfsl_npa_sheet_name()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let tcfsl_npa_file = matches
            .value_of("tcfsl_npa_file")
            .expect("Error getting `tcfsl_npa_file`.")
            .to_string();
        let finnone_fsl_file_path = matches
            .value_of("finnone_fsl_file_path")
            .expect("Error getting `finnone_fsl_file_path`.")
            .to_string();
        let writeoff_merged_file_path = matches
            .value_of("writeoff_merged_file_path")
            .expect("Error getting `writeoff_merged_file_path`.")
            .to_string();
        let finnone_cashflow_file_path = matches
            .value_of("finnone_cashflow_file_path")
            .expect("Error getting `finnone_cashflow_file_path`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
            .to_string();
        let tcfsl_npa_sheet_name = matches
            .value_of("tcfsl_npa_sheet_name")
            .expect("Error getting `tcfsl_npa_sheet_name`.")
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
            tcfsl_npa_file,
            finnone_fsl_file_path,
            finnone_cashflow_file_path,
            writeoff_merged_file_path,
            output_file,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            tcfsl_npa_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn tcfsl_npa_file(&self) -> &str {
        &self.tcfsl_npa_file
    }
    pub fn tcfsl_npa_sheet_name(&self) -> &str {
        &self.tcfsl_npa_sheet_name
    }
    pub fn finnone_fsl_file_path(&self) -> &str {
        &self.finnone_fsl_file_path
    }
    pub fn writeoff_merged_file_path(&self) -> &str {
        &self.writeoff_merged_file_path
    }
    pub fn finnone_cashflow_file_path(&self) -> &str {
        &self.finnone_cashflow_file_path
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
        .about("Program for ubs loans!!")
        .version("1.2.4958")
        .author("Sonali<sonali.s@surya-soft.com>")
        .arg(
            Arg::with_name("tcfsl_npa_file")
                .long("tcfsl-npa-file")
                .value_name("MAPPING FILE")
                .help("Path torepricing mapping master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("tcfsl_npa_sheet_name")
                .long("tcfsl-sheet-name")
                .value_name("TCFSL SHEET NAME")
                .help("Sheet name for Tcfsl file")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_fsl_file_path")
                .long("finnone-fsl-file")
                .value_name("FINNONE FSL FILE")
                .help("Path to Finnone fsl File.")
                .required(true)
        )
        .arg(
            Arg::with_name("writeoff_merged_file_path")
                .long("writeoff-merged-file")
                .value_name("WRITEOFF MERGED FILE PATH")
                .help("Path to Writeoff merged File.")
                .required(true)
        )
        .arg(
            Arg::with_name("finnone_cashflow_file_path")
                .long("finnnone-cashflow-file")
                .value_name("FINNONE CASHFLOW FILE PATH")
                .help("Path to Finnone Cashflow file.")
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
