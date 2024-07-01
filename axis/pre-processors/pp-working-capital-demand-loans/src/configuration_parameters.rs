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
    pub exchange_rate_file: String,
    pub balm_gam_file_path: String,
    pub wcdl_file_path: String,
    pub npa_data_file_path: String,
    pub npa_live_file_path: String,
    pub currency: String,
    pub npa_config_file_path: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub code_type: String,
    pub cm_code: String,
    pub balm_rct_file_path: String,
    pub balm_gac_file_path: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "balm_gam_file: {}", self.balm_gam_file_path());
        info!(logger, "npa_data_file: {}", self.npa_data_file_path());
        info!(logger, "npa_live_file: {}", self.npa_live_file_path());
        info!(logger, "npa_config_file: {}", self.npa_config_file_path());
        info!(logger, "currency: {}", self.currency());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "wcdl_file_path: {}", self.wcdl_file_path());
        info!(logger, "cm_code: {}", self.cm_code());
        info!(logger, "code_type: {}", self.code_type());
        info!(logger, "balm_gac_file_path: {}", self.balm_gac_file_path());
        info!(logger, "balm_rct_file_path: {}", self.balm_rct_file_path());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file` value.")
            .to_string();
        let balm_gam_file_path = matches
            .value_of("balm_gam_file")
            .expect("Error getting `balm_gam_file` value.")
            .to_string();
        let balm_gac_file_path = matches
            .value_of("balm_gac_file")
            .expect("Error getting `balm_gac_file` value.")
            .to_string();
        let balm_rct_file_path = matches
            .value_of("balm_rct_file")
            .expect("Error getting `balm_rct_file` value.")
            .to_string();
        let wcdl_file_path = matches
            .value_of("wcdl_file_path")
            .expect("Error getting `wcdl_file_path` value.")
            .to_string();
        let npa_data_file_path = matches
            .value_of("npa_data_file")
            .expect("Error getting `npa_data_file` value.")
            .to_string();
        let npa_live_file_path = matches
            .value_of("npa_live_file")
            .expect("Error getting `npa_live_file` value.")
            .to_string();
        let npa_config_file_path = matches
            .value_of("npa_config_file")
            .expect("Error getting `npa_config_file` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency` value.")
            .to_string();
        let code_type = matches
            .value_of("code_type")
            .expect("Error getting `code_type` value.")
            .to_string();
        let cm_code = matches
            .value_of("cm_code")
            .expect("Error getting `cm_code` value.")
            .to_string();
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

        ConfigurationParameters {
            exchange_rate_file,
            balm_gam_file_path,
            npa_data_file_path,
            npa_live_file_path,
            npa_config_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            currency,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            wcdl_file_path,
            code_type,
            cm_code,
            balm_rct_file_path,
            balm_gac_file_path,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn balm_gam_file_path(&self) -> &str {
        &self.balm_gam_file_path
    }
    pub fn balm_gac_file_path(&self) -> &str {
        &self.balm_gac_file_path
    }
    pub fn balm_rct_file_path(&self) -> &str {
        &self.balm_rct_file_path
    }
    pub fn wcdl_file_path(&self) -> &str {
        &self.wcdl_file_path
    }
    pub fn npa_data_file_path(&self) -> &str {
        &self.npa_data_file_path
    }
    pub fn npa_live_file_path(&self) -> &str {
        &self.npa_live_file_path
    }
    pub fn npa_config_file_path(&self) -> &str {
        &self.npa_config_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn currency(&self) -> &str {
        &self.currency
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
    pub fn code_type(&self) -> &str {
        &self.code_type
    }
    pub fn cm_code(&self) -> &str {
        &self.cm_code
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app pre-processes the working capital demand loans.")
        .version("1.0.5005")
        .author("Tanuj Singh Rathore<tanuj.s@surya-soft.com>")
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("exchange_rate_file")
                .help("Path to exchange rate file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("balm_gam_file")
                .long("balm-gam-file")
                .value_name("balm_gam_file")
                .help("Path to balm gam file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("balm_gac_file")
                .long("balm-gac-file")
                .value_name("balm_gac_file")
                .help("Path to balm gac file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("balm_rct_file")
                .long("balm-rct-file")
                .value_name("balm_rct_file")
                .help("Path to balm rct file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("wcdl_file_path")
                .long("wcdl-file-path")
                .value_name("wcdl_file_path")
                .help("Path wcdl file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_data_file")
                .long("npa-data-file")
                .value_name("NPA_DATA_FILE")
                .help("Path to npa data file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_live_file")
                .long("npa-live-file")
                .value_name("NPA_LIVE_FILE")
                .help("Path to npa live file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("currency")
                .value_name("currency")
                .help("Currency to be used for processing.")
                .required(true)
        )
        .arg(
            Arg::with_name("npa_config_file")
                .long("npa-config-file")
                .value_name("NPA_CONFIG_FILE")
                .help("Path to npa config file that needs to be processed.")
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
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
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
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .arg(
            Arg::with_name("code_type")
                .long("code-type")
                .value_name("code_type")
                .help("Code type'.")
                .required(true)
        )
        .arg(
            Arg::with_name("cm_code")
                .long("cm-code")
                .value_name("cm_code")
                .help("cm_code'.")
                .required(true)
        )
        .get_matches()
}
