use clap;
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
    pub master_file_path: String,
    pub ref1_file_path: String,
    pub ref1_file_sheet_name: String,
    pub ref2_file_path: String,
    pub ref3_file_path: String,
    pub ref4_file_path: String,
    pub ref4_file_sheet_name: String,
    pub ref5_file_path: String,
    pub crm_master_file_path: String,
    pub cust_entity_master_file_path: String,
    pub output_file_path: String,
    pub as_on_date: NaiveDate,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "master_file_path: {}", self.master_file_path());
        info!(logger, "ref1_file_path: {}", self.ref1_file_path());
        info!(
            logger,
            "ref1_file_sheet_name: {}",
            self.ref1_file_sheet_name()
        );
        info!(logger, "ref2_file_path: {}", self.ref2_file_path());
        info!(logger, "ref3_file_path: {}", self.ref3_file_path());
        info!(logger, "ref4_file_path: {}", self.ref4_file_path());
        info!(
            logger,
            "ref4_file_sheet_name: {}",
            self.ref4_file_sheet_name()
        );
        info!(logger, "ref5_file_path: {}", self.ref5_file_path());
        info!(
            logger,
            "crm_master_file_path: {}",
            self.crm_master_file_path()
        );
        info!(
            logger,
            "cust_entity_master_file_path: {}",
            self.cust_entity_master_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let master_file_path = matches
            .value_of("master_file_path")
            .expect("Error getting `master_file_path` value.")
            .to_string();
        let ref1_file_path = matches
            .value_of("ref1_file_path")
            .expect("Error getting `ref1_file_path` value.")
            .to_string();
        let ref1_file_sheet_name = matches
            .value_of("ref1_file_sheet_name")
            .expect("Error getting `ref1_file_sheet_name` value.")
            .to_string();
        let ref2_file_path = matches
            .value_of("ref2_file_path")
            .expect("Error getting `ref2_file_path` value.")
            .to_string();
        let ref3_file_path = matches
            .value_of("ref3_file_path")
            .expect("Error getting `ref3_file_path` value.")
            .to_string();
        let ref4_file_path = matches
            .value_of("ref4_file_path")
            .expect("Error getting `ref4_file_path` value.")
            .to_string();
        let ref4_file_sheet_name = matches
            .value_of("ref4_file_sheet_name")
            .expect("Error getting `ref4_file_sheet_name` value.")
            .to_string();
        let ref5_file_path = matches
            .value_of("ref5_file_path")
            .expect("Error getting `ref5_file_path` value.")
            .to_string();
        let cust_entity_master_file_path = matches
            .value_of("cust_entity_master_file_path")
            .expect("Error getting `cust_entity_master_file_path` value.")
            .to_string();
        let crm_master_file_path = matches
            .value_of("crm_master_file_path")
            .expect("Error getting `crm_master_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
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

        ConfigurationParameters {
            master_file_path,
            ref1_file_path,
            ref1_file_sheet_name,
            ref2_file_path,
            ref3_file_path,
            ref4_file_path,
            ref4_file_sheet_name,
            ref5_file_path,
            cust_entity_master_file_path,
            crm_master_file_path,
            output_file_path,
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
    pub fn master_file_path(&self) -> &str {
        &self.master_file_path
    }
    pub fn ref1_file_path(&self) -> &str {
        &self.ref1_file_path
    }
    pub fn ref1_file_sheet_name(&self) -> &str {
        &self.ref1_file_sheet_name
    }
    pub fn ref2_file_path(&self) -> &str {
        &self.ref2_file_path
    }
    pub fn ref3_file_path(&self) -> &str {
        &self.ref3_file_path
    }
    pub fn ref4_file_path(&self) -> &str {
        &self.ref4_file_path
    }
    pub fn ref4_file_sheet_name(&self) -> &str {
        &self.ref4_file_sheet_name
    }
    pub fn ref5_file_path(&self) -> &str {
        &self.ref5_file_path
    }
    pub fn cust_entity_master_file_path(&self) -> &str {
        &self.cust_entity_master_file_path
    }
    pub fn crm_master_file_path(&self) -> &str {
        &self.crm_master_file_path
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app generates pre-processor output of loans for AUSFB")
        .version("1.0.4271")
        .author("Ravindar Singh <ravindar.sr@surya-soft.com>")
        .arg(
            Arg::with_name("master_file_path")
                .short("m")
                .long("master-file")
                .value_name("master_file_path")
                .help("Path to input file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref1_file_path")
                .short("r1")
                .long("ref1-file")
                .value_name("ref1_file_path")
                .help("Path to Ref1 file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref1_file_sheet_name")
                .short("r1s")
                .long("ref1-sheet-name")
                .value_name("ref1_file_sheet_name")
                .help("Sheet name of Ref1 file")
                .required(false)
        )
        .arg(
            Arg::with_name("ref2_file_path")
                .short("r2")
                .long("ref2-file")
                .value_name("ref2_file_path")
                .help("Path to Ref2 file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref3_file_path")
                .short("r3")
                .long("ref3-file")
                .value_name("ref3_file_path")
                .help("Path to Ref3 file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref4_file_path")
                .short("r4")
                .long("ref4-file")
                .value_name("ref4_file_path")
                .help("Path to Ref4 file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref4_file_sheet_name")
                .short("r4s")
                .long("ref4-sheet-name")
                .value_name("ref4_file_sheet_name")
                .help("Sheet name of Ref4 file")
                .required(false)
        )
        .arg(
            Arg::with_name("ref5_file_path")
                .short("r5")
                .long("ref5-file")
                .value_name("ref5_file_path")
                .help("Path to Ref5 file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("cust_entity_master_file_path")
                .short("ce")
                .long("cust-entity-master-file")
                .value_name("cust_entity_master_file_path")
                .help("Path to Cust Entity Master file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("crm_master_file_path")
                .short("cm")
                .long("crm-master-file")
                .value_name("crm_master_file_path")
                .help("Path to CRM Master file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .short("o")
                .long("output-file")
                .value_name("output_file_path")
                .help("Path to output file that needs to be written.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .short("l")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .short("d")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics log.")
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
                .short("p")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as AS On Date.")
                .required(true)
        )
        .get_matches()
}
