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
    alco_master: String,
    alco_sheet: String,
    tenor_master: String,
    tenor_sheet: String,
    bucket_master: String,
    bucket_sheet: String,
    cat_master: String,
    cat_sheet: String,
    lcr_master: String,
    wd_nwd_master: String,
    wd_nwd_sheet: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    config_file_path: String,
    incremental_file_path: String,
    consol_ccy: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) -> () {
        info!(logger, "config_file: {}", self.config_file_path());
        info!(logger, "incremental_file: {}", self.incremental_file_path());
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
        info!(logger, "bucket_master: {}", self.bucket_master());
        info!(logger, "bucket_sheet: {}", self.bucket_sheet());
        info!(logger, "cat_master: {}", self.cat_master());
        info!(logger, "cat_sheet: {}", self.cat_sheet());
        info!(logger, "lcr_master: {}", self.lcr_master());
        info!(logger, "wd_nwd_master: {}", self.wd_nwd_master());
        info!(logger, "wd_nwd_sheet: {}", self.wd_nwd_sheet());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let config_file_path = matches.value_of("config_file").unwrap().to_string();
        let incremental_file_path = matches.value_of("incremental_file").unwrap().to_string();
        let output_file_path = matches.value_of("output_file").unwrap().to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .unwrap_or("this string will convert to today"),
        );
        let alco_master = matches
            .value_of("alco_master")
            .expect("Error getting `alco_master`.")
            .to_string();
        let alco_sheet = matches
            .value_of("alco_sheet")
            .expect("Error getting `alco_sheet`.")
            .to_string();
        let tenor_master = matches
            .value_of("tenor_master")
            .expect("Error getting `tenor_master`.")
            .to_string();
        let tenor_sheet = matches
            .value_of("tenor_sheet")
            .expect("Error getting `tenor_sheet`.")
            .to_string();
        let bucket_master = matches
            .value_of("bucket_master")
            .expect("Error getting `bucket_master`.")
            .to_string();
        let bucket_sheet = matches
            .value_of("bucket_sheet")
            .expect("Error getting `bucket_sheet`.")
            .to_string();
        let cat_master = matches
            .value_of("cat_master")
            .expect("Error getting `cat_master`.")
            .to_string();
        let cat_sheet = matches
            .value_of("cat_sheet")
            .expect("Error getting `cat_sheet`.")
            .to_string();
        let lcr_master = matches
            .value_of("lcr_master")
            .expect("Error getting `lcr_master`.")
            .to_string();
        let wd_nwd_master = matches
            .value_of("wd_nwd_master")
            .expect("Error getting `wd_nwd_master`.")
            .to_string();
        let wd_nwd_sheet = matches
            .value_of("wd_nwd_sheet")
            .expect("Error getting `wd_nwd_sheet`.")
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
            alco_master,
            alco_sheet,
            tenor_master,
            tenor_sheet,
            bucket_master,
            bucket_sheet,
            cat_master,
            cat_sheet,
            lcr_master,
            wd_nwd_master,
            wd_nwd_sheet,
            output_file_path,
            as_on_date,
            config_file_path,
            incremental_file_path,
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
    pub fn incremental_file_path(&self) -> &str {
        &self.incremental_file_path
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
    pub fn alco_master(&self) -> &str {
        &self.alco_master
    }
    pub fn alco_sheet(&self) -> &str {
        &self.alco_sheet
    }
    pub fn tenor_master(&self) -> &str {
        &self.tenor_master
    }
    pub fn tenor_sheet(&self) -> &str {
        &self.tenor_sheet
    }
    pub fn bucket_master(&self) -> &str {
        &self.bucket_master
    }
    pub fn bucket_sheet(&self) -> &str {
        &self.bucket_sheet
    }
    pub fn cat_master(&self) -> &str {
        &self.cat_master
    }
    pub fn cat_sheet(&self) -> &str {
        &self.cat_sheet
    }
    pub fn lcr_master(&self) -> &str {
        &self.lcr_master
    }
    pub fn wd_nwd_master(&self) -> &str {
        &self.wd_nwd_master
    }
    pub fn wd_nwd_sheet(&self) -> &str {
        &self.wd_nwd_sheet
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Generates reports for TD-Movement-Incremental-Rollover.")
        .version("1.0.2862")
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
            Arg::with_name("bucket_master")
                .long("bucket-master")
                .value_name("Amount Bucket Master File")
                .help("Path to Amount Bucket Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("bucket_sheet")
                .long("bucket-sheet")
                .value_name("Amount Bucket Master Sheet Name")
                .help("Name of Amount Bucket Master File sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("cat_master")
                .long("cat-master")
                .value_name("Category Master File")
                .help("Path to Category Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("cat_sheet")
                .long("cat-sheet")
                .value_name("Category Master Sheet Name")
                .help("Name of Category Master File sheet")
                .default_value("Sheet1")
                .required(false),
        )
        .arg(
            Arg::with_name("lcr_master")
                .long("lcr-master")
                .value_name("LCR Master File")
                .help("Path to LCR Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("wd_nwd_master")
                .long("wd-nwd-master")
                .value_name("WD/NWD Master File")
                .help("Path to WD/NWD Master File")
                .required(true),
        )
        .arg(
            Arg::with_name("wd_nwd_sheet")
                .long("wd-nwd-sheet")
                .value_name("WD/NWD Master Sheet Name")
                .help("Name of WD/NWD Master File sheet")
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
                .short("c")
                .long("config-file")
                .value_name("FILE")
                .help("Path to config file that needs to be processed")
                .required(true),
        )
        .arg(
            Arg::with_name("incremental_file")
                .short("i")
                .long("incremental-file")
                .value_name("FILE")
                .help("Path to increment file that needs to be used for lookup")
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
