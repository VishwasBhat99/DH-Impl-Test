use clap::{App, Arg};
use slog::Logger;
pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub casatd_master_file_path: String,
    pub adv_master_file_path: String,
    pub ops_acc_data_file_path: String,
    pub mult_depo_file_path: String,
    pub const_desc_file_path: String,
    pub prod_code_file_path: String,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub diagnostics_flag: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(
            logger,
            "casatd_master_file_path: {}",
            self.casatd_master_file_path()
        );
        info!(
            logger,
            "adv_master_file_path: {}",
            self.adv_master_file_path()
        );
        info!(
            logger,
            "ops_acc_data_file_path: {:?}",
            self.ops_acc_data_file_path()
        );
        info!(
            logger,
            "mult_depo_file_path: {:?}",
            self.mult_depo_file_path()
        );
        info!(
            logger,
            "const_desc_file_path: {:?}",
            self.const_desc_file_path()
        );
        info!(
            logger,
            "prod_code_file_path: {:?}",
            self.prod_code_file_path()
        );
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file_path: {}", self.log_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "diagnostics_flag: {}", self.diagnostics_flag());
        info!(
            logger,
            "diagnostics_file_path: {}",
            self.diagnostics_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let casatd_master_file_path = matches
            .value_of("casatd_master_file_path")
            .expect("Error getting `casatd_master_file_path` value.")
            .to_string();
        let adv_master_file_path = matches
            .value_of("adv_master_file_path")
            .expect("Error getting `adv_master_file_path` value.")
            .to_string();
        let ops_acc_data_file_path = matches
            .value_of("ops_acc_data_file_path")
            .expect("Error getting `ops_acc_data_file_path` value.")
            .to_string();
        let mult_depo_file_path = matches
            .value_of("mult_depo_file_path")
            .expect("Error getting `mult_depo_file_path` value.")
            .to_string();
        let const_desc_file_path = matches
            .value_of("const_desc_file_path")
            .expect("Error getting `const_desc_file_path` value.")
            .to_string();
        let prod_code_file_path = matches
            .value_of("prod_code_file_path")
            .expect("Error getting `prod_code_file_path` value.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file_path")
            .expect("Error getting `log_file_path` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_file_path")
            .expect("Error getting `diagnostics_file_path` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let diagnostics_flag = matches
            .value_of("diagnostics_flag")
            .expect("Error getting `diagnostics_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `diagnostics_flag` value as bool.");

        ConfigurationParameters {
            casatd_master_file_path,
            adv_master_file_path,
            ops_acc_data_file_path,
            mult_depo_file_path,
            const_desc_file_path,
            prod_code_file_path,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            diagnostics_flag,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn casatd_master_file_path(&self) -> &str {
        &self.casatd_master_file_path
    }
    pub fn adv_master_file_path(&self) -> &str {
        &self.adv_master_file_path
    }
    pub fn ops_acc_data_file_path(&self) -> &str {
        &self.ops_acc_data_file_path
    }
    pub fn mult_depo_file_path(&self) -> &str {
        &self.mult_depo_file_path
    }
    pub fn const_desc_file_path(&self) -> &str {
        &self.const_desc_file_path
    }
    pub fn prod_code_file_path(&self) -> &str {
        &self.prod_code_file_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }

    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn diagnostics_flag(&self) -> bool {
        self.diagnostics_flag
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
}
fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("BIU Preprocessor!")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("casatd_master_file_path")
                .long("casatd-master")
                .value_name("CA SA TD Master File")
                .help("Path to the CA SA TD Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("adv_master_file_path")
                .long("adv-master")
                .value_name("Advance Master File")
                .help("Path to the Advance Master file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ops_acc_data_file_path")
                .long("ops-acc-data")
                .value_name("Optional Account Data file")
                .help("Optional Account Data file.")
                .required(true)
        )
        .arg(
            Arg::with_name("mult_depo_file_path")
                .long("mult-depo-file")
                .value_name("Multiple deposites file path")
                .help("Multiple deposites file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("const_desc_file_path")
                .long("const-desc-file-path")
                .value_name("Const description File Path")
                .help("Const description file path")
                .required(true)
        )
        .arg(
            Arg::with_name("prod_code_file_path")
                .long("prod-code-file-path")
                .value_name("Product Code file path")
                .help("Product Code File Path.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file_path")
                .long("log-file-path")
                .value_name("FILE")
                .help("Log file path")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_file_path")
                .long("diagnostics-file-path")
                .value_name("FILE")
                .help("Diagnostics log file path")
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
            Arg::with_name("diagnostics_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
