use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    exchange_rate_file: String,
    coupon_master_file_path: String,
    coupon_sheet_name: String,
    output_file: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    local_ccy: String,
    consolidated_ccy: String,
    is_perf_diagnostics_enabled: bool,
    exchange_rate_output_file: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "coupon_master_file_path: {}",
            self.coupon_master_file_path()
        );
        info!(
            logger,
            "exchange_rate_output_file: {}",
            self.exchange_rate_output_file()
        );
        info!(
            logger,
            "exchange_rate_file_path: {}",
            self.exchange_rate_file()
        );
        info!(logger, "output_file: {}", self.output_file());
        info!(logger, "local_ccy: {}", self.local_ccy());
        info!(logger, "consolidated_ccy: {}", self.consolidated_ccy());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "coupon_sheet_name: {}", self.coupon_sheet_name());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let exchange_rate_output_file = matches
            .value_of("exchange_rate_output_file")
            .expect("Error getting `exchange_rate_output_file`.")
            .to_string();
        let coupon_master_file_path = matches
            .value_of("coupon_master_file_path")
            .expect("Error getting `coupon_master_file_path`.")
            .to_string();
        let coupon_sheet_name = matches
            .value_of("coupon_sheet_name")
            .expect("Error getting `coupon_sheet_name`.")
            .to_string();
        let local_ccy = matches
            .value_of("local_ccy")
            .expect("Error getting `local_ccy`.")
            .to_string();
        let consolidated_ccy = matches
            .value_of("consolidated_ccy")
            .expect("Error getting `consolidated_ccy`.")
            .to_string();
        let exchange_rate_file = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file`.")
            .to_string();
        let output_file = matches
            .value_of("output_file")
            .expect("Error getting `output_file`.")
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
            input_file_path,
            coupon_master_file_path,
            exchange_rate_file,
            output_file,
            local_ccy,
            consolidated_ccy,
            as_on_date,
            exchange_rate_output_file,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            coupon_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn coupon_master_file_path(&self) -> &str {
        &self.coupon_master_file_path
    }
    pub fn exchange_rate_file(&self) -> &str {
        &self.exchange_rate_file
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
    pub fn exchange_rate_output_file(&self) -> &str {
        &self.exchange_rate_output_file
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn coupon_sheet_name(&self) -> &str {
        &self.coupon_sheet_name
    }
    pub fn consolidated_ccy(&self) -> &str {
        &self.consolidated_ccy
    }
    pub fn local_ccy(&self) -> &str {
        &self.local_ccy
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Coupon stamper for HDFC")
        .version("1.2.4648")
        .author("Sonali<sonali.s@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("INPUT FILE")
                .help("Path to get input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("coupon_master_file_path")
                .long("coupon-master-file")
                .value_name("COUPON FILE")
                .help("Path to COUPON File.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("Exchange rate file")
                .help("Path to exchange rate File.")
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
            Arg::with_name("exchange_rate_output_file")
                .long("exchange-rate-out")
                .value_name("INTERMEDIATE OUTPUT")
                .help("Path to Exchenge rate output file.")
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
                .possible_values(["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("none")
                .required(false)
        ).
        arg(
            Arg::with_name("coupon_sheet_name")
                .long("coupon-sheet-name")
                .value_name("SHEET NAME")
                .help("This field will give the coupon sheet name")
                .default_value("Sheet1")
                .required(false)
        ).
        arg(
            Arg::with_name("consolidated_ccy")
                .long("consolidated-ccy")
                .value_name("CONSOLIDATED CCY")
                .help("This field will give the consolidated_ccy")
                .required(true)
        ).
        arg(
            Arg::with_name("local_ccy")
                .long("local-ccy")
                .value_name("LOCAL CCY")
                .help("This field will give the local ccy")
                .required(true)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS_FLAG")
                .possible_values(["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
