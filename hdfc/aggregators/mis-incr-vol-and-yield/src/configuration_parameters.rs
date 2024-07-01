use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);

    let parameters = ConfigurationParameters::new_from_matches(matches);
    return parameters;
}

pub struct ConfigurationParameters {
    output_file_path: String,
    as_on_date: NaiveDate,
    config_file_path: String,
    consol_ccy: String,
    org_tenor_file_path: String,
    incr_acc_file_path: String,
    psl_map_file_path: String,
    alco_map_file_path: String,
    org_tenor_sheet_name: String,
    psl_map_sheet_name: String,
    alco_map_sheet_name: String,
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
        info!(
            logger,
            "org_tenor_file_path: {}",
            self.org_tenor_file_path()
        );
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());

        info!(logger, "psl_map_file_path: {}", self.psl_map_file_path());
        info!(logger, "alco_map_file_path: {}", self.alco_map_file_path());
        info!(
            logger,
            "org_tenor_sheet_name: {}",
            self.org_tenor_sheet_name()
        );
        info!(logger, "psl_map_sheet_name: {}", self.psl_map_sheet_name());
        info!(
            logger,
            "alco_map_sheet_name: {}",
            self.alco_map_sheet_name()
        );
        info!(logger, "incr_acc_file_path: {}", self.incr_acc_file_path());
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
        let org_tenor_file_path = matches.value_of("org_tenor_file_path").unwrap().to_string();

        let psl_map_file_path = matches.value_of("psl_map_file_path").unwrap().to_string();
        let alco_map_file_path = matches.value_of("alco_map_file_path").unwrap().to_string();
        let org_tenor_sheet_name = matches
            .value_of("org_tenor_sheet_name")
            .unwrap()
            .to_string();
        let psl_map_sheet_name = matches.value_of("psl_map_sheet_name").unwrap().to_string();
        let alco_map_sheet_name = matches.value_of("alco_map_sheet_name").unwrap().to_string();
        let incr_acc_file_path = matches.value_of("incr_acc_file_path").unwrap().to_string();
        ConfigurationParameters {
            output_file_path,
            as_on_date,
            config_file_path,
            consol_ccy,
            org_tenor_file_path,
            incr_acc_file_path,
            psl_map_file_path,
            alco_map_file_path,
            org_tenor_sheet_name,
            psl_map_sheet_name,
            alco_map_sheet_name,
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
        return &self.config_file_path;
    }
    pub fn output_file_path(&self) -> &str {
        return &self.output_file_path;
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        return &self.as_on_date;
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
        return &self.consol_ccy;
    }
    pub fn psl_map_file_path(&self) -> &str {
        return &self.psl_map_file_path;
    }
    pub fn alco_map_file_path(&self) -> &str {
        return &self.alco_map_file_path;
    }
    pub fn org_tenor_sheet_name(&self) -> &str {
        return &self.org_tenor_sheet_name;
    }
    pub fn psl_map_sheet_name(&self) -> &str {
        return &self.psl_map_sheet_name;
    }
    pub fn alco_map_sheet_name(&self) -> &str {
        return &self.alco_map_sheet_name;
    }
    pub fn org_tenor_file_path(&self) -> &str {
        return &self.org_tenor_file_path;
    }
    pub fn incr_acc_file_path(&self) -> &str {
        return &self.incr_acc_file_path;
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This program generates output for MIS Benchmark!")
        .version("1.0.3990")
        .author("Sougata Bhattacharjee <sougata.b@surya-soft.com>")
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
        .arg(
            Arg::with_name("org_tenor_file_path")
                .short("ot")
                .long("org-tenor-file-path")
                .value_name("FILE")
                .help("Path to the original tenor file")
                .required(true),
        )
        .arg(
            Arg::with_name("psl_map_sheet_name")
                .short("pms")
                .long("psl-map-sheet-name")
                .value_name("SHEET")
                .help("psl_map_sheet_name")
                .required(true),
        )
        .arg(
            Arg::with_name("alco_map_sheet_name")
                .short("ams")
                .long("alco-map-sheet-name")
                .value_name("SHEET")
                .help("alco_map_sheet_name")
                .required(true),
        )
        .arg(
            Arg::with_name("org_tenor_sheet_name")
                .short("ots")
                .long("org-tenor-sheet-name")
                .value_name("SHEET")
                .help("org_tenor_sheet_name")
                .required(true),
        )
        .arg(
            Arg::with_name("alco_map_file_path")
                .short("am")
                .long("alco-map-file-path")
                .value_name("FILE")
                .help("Path to the ALCO Mapping file")
                .required(true),
        )
        .arg(
            Arg::with_name("psl_map_file_path")
                .short("pm")
                .long("psl-map-file-path")
                .value_name("FILE")
                .help("Path to the PSL Mapping File")
                .required(true),
        )
        .arg(
            Arg::with_name("incr_acc_file_path")
                .short("ia")
                .long("incr-acc-file-path")
                .value_name("FILE")
                .help("Path to the Account Skip File")
                .required(true),
        )
        .get_matches()
}
