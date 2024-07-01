use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use sdb_day_convention::Conventions;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub exchange_rate_file: String,
    pub blrms_file_path: String,
    pub common_code_file_path: String,
    pub nslr_file_path: String,
    pub base_currency: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub bond_master_file_path: String,
    pub country_code: String,
    day_convention: Conventions,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "exchange_rate_file: {}", self.exchange_rate_file());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "blrms_file_path: {}", self.blrms_file_path());
        info!(logger, "base_currency: {}", self.base_currency());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "day_convention: {:?}", self.day_convention());
        info!(logger, "nslr_file_path: {}", self.nslr_file_path());
        info!(logger, "country_code: {}", self.country_code());
        info!(
            logger,
            "common_code_file_path: {}",
            self.common_code_file_path()
        );
        info!(
            logger,
            "bond_master_file_path: {}",
            self.bond_master_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let bond_master_file_path = matches
            .value_of("bond_master_file_path")
            .expect("Error getting `bond_master_file_path` value.")
            .to_string();

        let common_code_file_path = matches
            .value_of("common_code_file_path")
            .expect("Error getting `common_code_file_path` value.")
            .to_string();
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file` value.")
            .to_string();
        let blrms_file_path = matches
            .value_of("blrms_file_path")
            .expect("Error getting `blrms_file_path` value.")
            .to_string();
        let nslr_file_path = matches
            .value_of("nslr_file_path")
            .expect("Error getting `nslr_file_path` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency` value.")
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
        let country_code = matches
            .value_of("country_code")
            .expect("Error getting `country_code` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let day_convention = {
            let conv = matches
                .value_of("day_convention")
                .expect("Error getting `day_convention` value.");
            match conv {
                "ACT/ACT" => Conventions::ACTbyACT,
                "ACT/365" => Conventions::ACTby365,
                "ACT/360" => Conventions::ACTby360,
                "30/360" => Conventions::Thirtyby360,
                "ACT30/360" => Conventions::AccruedThirtyby360,
                _ => {
                    panic!("Incorrect day convention parameter passed. Must be one of { ACT/ACT, ACT/365, ACT/360, 30/360, ACT30/360 }")
                }
            }
        };

        ConfigurationParameters {
            exchange_rate_file,
            common_code_file_path,
            blrms_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            base_currency,
            diagnostics_file_path,
            log_level,
            country_code,
            is_perf_diagnostics_enabled,
            nslr_file_path,
            bond_master_file_path,
            day_convention,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
    }
    pub fn blrms_file_path(&self) -> &str {
        &self.blrms_file_path
    }
    pub fn nslr_file_path(&self) -> &str {
        &self.nslr_file_path
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
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn country_code(&self) -> &str {
        &self.country_code
    }
    pub fn common_code_file_path(&self) -> &str {
        &self.common_code_file_path
    }
    pub fn bond_master_file_path(&self) -> &str {
        &self.bond_master_file_path
    }
    pub fn day_convention(&self) -> &Conventions {
        &self.day_convention
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app pre-processes the tri party corporate bond.")
        .version("1.1.4975")
        .author("Tanuj Singh Rathore<tanuj.s@surya-soft.com>")
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("exchange_rate_file")
                .help("Path to exchange rate file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("bond_master_file_path")
                .long("bond-master-file-path")
                .value_name("bond_master_file_path")
                .help("Path to common code file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("common_code_file_path")
                .long("common-code-file-path")
                .value_name("common_code_file_path")
                .help("Path to common code file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("blrms_file_path")
                .long("blrms-file-path")
                .value_name("blrms_file_path")
                .help("Path to blrms file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("base_currency")
                .help("Base Currency is required.")
                .required(true)
        )
        .arg(
            Arg::with_name("nslr_file_path")
                .long("nslr-file-path")
                .value_name("nslr_file_path")
                .help("Path nslr file that needs to be processed.")
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
            Arg::with_name("country_code")
                .long("country-code")
                .value_name("country_code")
                .help("Country code.")
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
            Arg::with_name("day_convention")
                .short("C")
                .long("day-convention")
                .value_name("CONVENTION")
                .help("The convention to be used for interest calculation.")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
