use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_master_file: String,
    excess_output_file_path: String,
    overdue_output_file_path: String,
    single_eistructure_output_file_path: String,
    excess_emimultiple_output_file_path: String,
    overdue_emimultiple_output_file_path: String,
    multipleemiaccounts_output_file_path: String,
    multiplepmiaccounts_output_file_path: String,
    prod_gl_master_file: String,
    prod_gl_master_sheet: String,
    default_gls: Vec<String>,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
    pub sma_file_path: String,
    pub data_src_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_master_file: {}", self.input_master_file());
        info!(
            logger,
            "excess_output_file: {}",
            self.excess_output_file_path()
        );
        info!(
            logger,
            "overdue_output_file: {}",
            self.overdue_output_file_path()
        );
        info!(
            logger,
            "single_eistructure_output_file_path: {}",
            self.single_eistructure_output_file_path()
        );
        info!(
            logger,
            "excess_emimultiple_output_file_path: {}",
            self.excess_emimultiple_output_file_path()
        );
        info!(
            logger,
            "overdue_emimultiple_output_file_path: {}",
            self.overdue_emimultiple_output_file_path()
        );
        info!(
            logger,
            "multipleemiaccounts_output_file_path: {}",
            self.multipleemiaccounts_output_file_path()
        );
        info!(
            logger,
            "multiplepmiaccounts_output_file_path: {}",
            self.multiplepmiaccounts_output_file_path()
        );
        info!(
            logger,
            "prod_gl_master_file: {}",
            self.prod_gl_master_file()
        );
        info!(
            logger,
            "prod_gl_master_sheet: {}",
            self.prod_gl_master_sheet()
        );
        info!(logger, "default_gls: {:?}", self.default_gls());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "sma_file_path: {}", self.sma_file_path());
        info!(logger, "data_src_name: {}", self.data_src_name());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_master_file = matches
            .value_of("input_master_file")
            .expect("Error getting `input_master_file`.")
            .to_string();
        let excess_output_file_path = matches
            .value_of("excess_output_file")
            .expect("Error getting `excess_output_file_path`.")
            .to_string();
        let overdue_output_file_path = matches
            .value_of("overdue_output_file")
            .expect("Error getting `overdue_output_file_path`.")
            .to_string();
        let single_eistructure_output_file_path = matches
            .value_of("single_eistructure_output_file_path")
            .expect("Error getting `single_eistructure_output_file_path`.")
            .to_string();
        let excess_emimultiple_output_file_path = matches
            .value_of("excess_emimultiple_output_file_path")
            .expect("Error getting `excess_emimultiple_output_file_path`.")
            .to_string();
        let overdue_emimultiple_output_file_path = matches
            .value_of("overdue_emimultiple_output_file_path")
            .expect("Error getting `overdue_emimultiple_output_file_path`.")
            .to_string();
        let multipleemiaccounts_output_file_path = matches
            .value_of("multipleemiaccounts_output_file_path")
            .expect("Error getting `multipleemiaccounts_output_file_path`.")
            .to_string();
        let multiplepmiaccounts_output_file_path = matches
            .value_of("multiplepmiaccounts_output_file_path")
            .expect("Error getting `multiplepmiaccounts_output_file_path`.")
            .to_string();
        let prod_gl_master_file = matches
            .value_of("prod_gl_master_file")
            .expect("Error getting `prod_gl_master_file`.")
            .to_string();
        let prod_gl_master_sheet = matches
            .value_of("prod_gl_master_sheet")
            .expect("Error getting `prod_gl_master_sheet`.")
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
        let default_gls: Vec<String> = matches
            .value_of("default_gls")
            .expect("Error getting `default_gls`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let sma_file_path = matches
            .value_of("sma_file_path")
            .expect("Error getting `sma_file_path` value.")
            .to_string();
        let data_src_name = matches
            .value_of("data_src_name")
            .expect("Error getting `data_src_name` value.")
            .to_string();
        ConfigurationParameters {
            input_master_file,
            excess_output_file_path,
            overdue_output_file_path,
            single_eistructure_output_file_path,
            excess_emimultiple_output_file_path,
            overdue_emimultiple_output_file_path,
            multipleemiaccounts_output_file_path,
            multiplepmiaccounts_output_file_path,
            prod_gl_master_file,
            prod_gl_master_sheet,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            default_gls,
            sma_file_path,
            data_src_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_master_file(&self) -> &str {
        &self.input_master_file
    }
    pub fn excess_output_file_path(&self) -> &str {
        &self.excess_output_file_path
    }
    pub fn overdue_output_file_path(&self) -> &str {
        &self.overdue_output_file_path
    }
    pub fn single_eistructure_output_file_path(&self) -> &str {
        &self.single_eistructure_output_file_path
    }
    pub fn excess_emimultiple_output_file_path(&self) -> &str {
        &self.excess_emimultiple_output_file_path
    }
    pub fn overdue_emimultiple_output_file_path(&self) -> &str {
        &self.overdue_emimultiple_output_file_path
    }
    pub fn multipleemiaccounts_output_file_path(&self) -> &str {
        &self.multipleemiaccounts_output_file_path
    }
    pub fn multiplepmiaccounts_output_file_path(&self) -> &str {
        &self.multiplepmiaccounts_output_file_path
    }
    pub fn prod_gl_master_file(&self) -> &str {
        &self.prod_gl_master_file
    }
    pub fn prod_gl_master_sheet(&self) -> &str {
        &self.prod_gl_master_sheet
    }
    pub fn default_gls(&self) -> &Vec<String> {
        &self.default_gls
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
    pub fn sma_file_path(&self) -> &str {
        &self.sma_file_path
    }
    pub fn data_src_name(&self) -> &str {
        &self.data_src_name
    }
}

fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("Program For Pre-Processor Individual Loans!!")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("input_master_file")
                .long("input-master-file")
                .value_name("Input Master File")
                .help("Input Master file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("excess_output_file")
                .long("excess-output")
                .value_name("Excess Output File Path")
                .help("Path to Excess Output File.")
                .required(true)
        )
        .arg(
            Arg::with_name("overdue_output_file")
                .long("overdue-output")
                .value_name("Overdue Output File Path")
                .help("Path to Overdue output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("single_eistructure_output_file_path")
                .long("singleEIStructure-output")
                .value_name("singleEIStructure output File Path")
                .help("Path to singleEIStructure output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("excess_emimultiple_output_file_path")
                .long("excesseEMIMultiple-output")
                .value_name("excesseEMIMultiple output File Path")
                .help("Path to excesseEMIMultiple output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("overdue_emimultiple_output_file_path")
                .long("overdueEMIMultiple-output")
                .value_name("overdueEMIMultiple output File Path")
                .help("Path to overdueEMIMultiple output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("multipleemiaccounts_output_file_path")
                .long("multipleEMIAccounts-output")
                .value_name("multipleEMIAccounts output File Path")
                .help("Path to multipleEMIAccounts output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("multiplepmiaccounts_output_file_path")
                .long("multiplePMIAccounts-output")
                .value_name("multiplePMIAccounts output File Path")
                .help("Path to multiplePMIAccounts output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("prod_gl_master_file")
                .long("prod-gl-master-file")
                .value_name("Prod GL File Path")
                .help("Path to Product GLs Mapping Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("prod_gl_master_sheet")
                .long("prod-gl-master-sheet")
                .value_name("Prod GLs Master Sheet Name")
                .help("Sheet Name to be read from Product GLs Master File.")
                .default_value("Sheet1")
                .required(false)
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
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
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
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("default_gls")
                .long("default-gls")
                .value_name("Default GLs")
                .help("Default GL Codes: GLCode,EMI-Overdue-GL,PreEMI-Overdue-GL,EMI-Excess-GL,PreEMI-Excess-GL")
                .required(true)
        )
        .arg(
            Arg::with_name("sma_file_path")
                .long("sma-file-path")
                .value_name("SMA_FILE")
                .help("Path to the SMA File")
                .required(true)
        )
        .arg(
            Arg::with_name("data_src_name")
                .long("data-src-name")
                .value_name("DATA_SRC_NAME")
                .help("Data Source Name")
                .required(true)
        )
        .get_matches()
}
