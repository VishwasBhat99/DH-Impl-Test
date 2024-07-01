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
    pub gac_file_path: String,
    pub cmg_file_path: String,
    pub eab_file_path: String,
    pub gsp_file_path: String,
    pub mapping_master_file_path: String,
    pub itc_file_path: String,
    pub ex_rt_file_path: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub crm_user_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "gam_file_path: {}", self.gam_file_path());
        info!(logger, "gac_file_path: {}", self.gac_file_path());
        info!(logger, "cmg_file_path: {}", self.cmg_file_path());
        info!(logger, "eab_file_path: {}", self.eab_file_path());
        info!(logger, "eab_file_path: {}", self.eab_file_path());
        info!(logger, "itc_file_path: {}", self.itc_file_path());
        info!(logger, "crm_user_file_path: {}", self.crm_user_file_path());
        info!(
            logger,
            "mapping_master_file_path: {}",
            self.mapping_master_file_path()
        );
        info!(logger, "ex_rt_file_path: {}", self.ex_rt_file_path());
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
        let itc_file_path = matches
            .value_of("itc_file_path")
            .expect("Error getting `itc_file_path` value.")
            .to_string();
        let mapping_master_file_path = matches
            .value_of("mapping_master_file_path")
            .expect("Error getting `mapping_master_file_path` value.")
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
        let gac_file_path = matches
            .value_of("gac_file_path")
            .expect("Error getting `gac_file_path` value.")
            .to_string();
        let cmg_file_path = matches
            .value_of("cmg_file_path")
            .expect("Error getting `cmg_file_path` value.")
            .to_string();
        let crm_user_file_path = matches
            .value_of("crm_user_file_path")
            .expect("Error getting `crm_user_file_path` value.")
            .to_string();
        let eab_file_path = matches
            .value_of("eab_file_path")
            .expect("Error getting `eab_file_path` value.")
            .to_string();
        let gsp_file_path = matches
            .value_of("gsp_file_path")
            .expect("Error getting `gsp_file_path` value.")
            .to_string();
        let ex_rt_file_path = matches
            .value_of("ex_rt_file_path")
            .expect("Error getting `ex_rt_file_path` value.")
            .to_string();

        ConfigurationParameters {
            gam_file_path,
            gac_file_path,
            itc_file_path,
            cmg_file_path,
            eab_file_path,
            gsp_file_path,
            mapping_master_file_path,
            ex_rt_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            crm_user_file_path,
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
    pub fn gac_file_path(&self) -> &str {
        &self.gac_file_path
    }
    pub fn itc_file_path(&self) -> &str {
        &self.itc_file_path
    }
    pub fn cmg_file_path(&self) -> &str {
        &self.cmg_file_path
    }
    pub fn eab_file_path(&self) -> &str {
        &self.eab_file_path
    }
    pub fn gsp_file_path(&self) -> &str {
        &self.gsp_file_path
    }
    pub fn mapping_master_file_path(&self) -> &str {
        &self.mapping_master_file_path
    }
    pub fn ex_rt_file_path(&self) -> &str {
        &self.ex_rt_file_path
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
    pub fn crm_user_file_path(&self) -> &str {
        &self.crm_user_file_path
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program to split GAM data file.")
        .version("1.0.5072")
        .author("NPunyashree <punyashree.n@surya-soft.com>")
        .arg(
            Arg::with_name("gam_file_path")
                .long("gam-file-path")
                .value_name("GAM File Path")
                .help("Path to the GAM file.")
                .required(true)
        )
        .arg(
            Arg::with_name("crm_user_file_path")
                .long("crm-user-file-path")
                .value_name("CRM USER File Path")
                .help("Path to the CRM USER file.")
                .required(true)
        )
        .arg(
            Arg::with_name("gac_file_path")
                .long("gac-file-path")
                .value_name("GAC File Path")
                .help("Path to the GAC File Path")
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
            Arg::with_name("cmg_file_path")
                .long("cmg-file-path")
                .value_name("CMG File Path")
                .help("Path to the CMG File.")
                .required(true)
        )
       .arg(
            Arg::with_name("ex_rt_file_path")
                .long("ex-rt-file-path")
                .value_name("EX_RT File Path")
                .help("Path to the EXCHANGE RATE File.")
                .required(true)
        )
        .arg(
            Arg::with_name("eab_file_path")
                .long("eab-file-path")
                .value_name("eab File Path")
                .help("Path to the eab File.")
                .required(true)
        )
        .arg(
            Arg::with_name("gsp_file_path")
                .long("gsp-file-path")
                .value_name("gsp File Path")
                .help("Path to the gsp File.")
                .required(true)
        )
      .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("mapping_master_file_path")
                .long("mapping-master-file")
                .value_name("Mapping Master File Path")
                .help("Path to the Mapping Master File.")
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
