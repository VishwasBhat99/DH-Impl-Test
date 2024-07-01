use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    fin_map_ref_path: String,
    fin_map_sheet_name: String,
    interelimination_ref_path: String,
    interelimination_sheet_name: String,
    ora_gl_map_ref_path: String,
    ora_gl_map_sheet_name: String,
    input_currency: String,
    as_on_date: NaiveDate,
    base_currency: String,
    currency_conversion_file_path: String,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "fin_map_sheet_name: {}", self.fin_map_sheet_name());
        info!(logger, "fin_map_ref_path: {}", self.fin_map_ref_path());
        info!(
            logger,
            "interelimination_sheet_name: {}",
            self.interelimination_sheet_name()
        );
        info!(
            logger,
            "interelimination_ref_path: {}",
            self.interelimination_ref_path()
        );
        info!(
            logger,
            "ora_gl_map_sheet_name: {}",
            self.ora_gl_map_sheet_name()
        );
        info!(
            logger,
            "ora_gl_map_ref_path: {}",
            self.ora_gl_map_ref_path()
        );
        info!(logger, "base_currency: {}", self.base_currency());
        info!(
            logger,
            "currency_conversion_file_path: {}",
            self.currency_conversion_file_path()
        );
        info!(logger, "input_currency: {}", self.input_currency());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_value`.")
            .to_string();
        let fin_map_sheet_name = matches
            .value_of("fin_map_sheet_name")
            .expect("Error getting `fin_map_sheet_name` value.")
            .to_string();
        let fin_map_ref_path = matches
            .value_of("fin_map_ref_path")
            .expect("Error getting `fin_map_ref_path` value.")
            .to_string();
        let base_currency = matches
            .value_of("base_currency")
            .expect("Error getting `base_currency`.")
            .to_string();
        let currency_conversion_file_path = matches
            .value_of("exchange_rate_file")
            .expect("Error getting `exchange_rate_file_path`.")
            .to_string();
        let interelimination_sheet_name = matches
            .value_of("interelimination_sheet_name")
            .expect("Error getting `interelimination_sheet_name` value.")
            .to_string();
        let interelimination_ref_path = matches
            .value_of("interelimination_ref_path")
            .expect("Error getting `interelimination_ref_path` value.")
            .to_string();
        let ora_gl_map_sheet_name = matches
            .value_of("ora_gl_map_sheet_name")
            .expect("Error getting `ora_gl_map_sheet_name` value.")
            .to_string();
        let ora_gl_map_ref_path = matches
            .value_of("ora_gl_map_ref_path")
            .expect("Error getting `ora_gl_map_ref_path` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error while getting `as_on_date`."),
        );
        let input_currency = matches
            .value_of("input_currency")
            .expect("Error getting `input_currency value.`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `diagnostics flag as enabled/disabled`.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` as bool.");

        ConfigurationParameters {
            input_file_path,
            fin_map_ref_path,
            fin_map_sheet_name,
            interelimination_ref_path,
            interelimination_sheet_name,
            ora_gl_map_ref_path,
            ora_gl_map_sheet_name,
            input_currency,
            as_on_date,
            base_currency,
            currency_conversion_file_path,
            output_file_path,
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
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn fin_map_ref_path(&self) -> &str {
        &self.fin_map_ref_path
    }
    pub fn fin_map_sheet_name(&self) -> &str {
        &self.fin_map_sheet_name
    }
    pub fn interelimination_ref_path(&self) -> &str {
        &self.interelimination_ref_path
    }
    pub fn interelimination_sheet_name(&self) -> &str {
        &self.interelimination_sheet_name
    }
    pub fn ora_gl_map_ref_path(&self) -> &str {
        &self.ora_gl_map_ref_path
    }
    pub fn ora_gl_map_sheet_name(&self) -> &str {
        &self.ora_gl_map_sheet_name
    }
    pub fn input_currency(&self) -> &str {
        &self.input_currency
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn base_currency(&self) -> &str {
        &self.base_currency
    }
    pub fn currency_conversion_file_path(&self) -> &str {
        &self.currency_conversion_file_path
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
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about(".cf file generator for LC")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("fin_map_ref_path")
                .long("fin-map-ref-path")
                .value_name("FIN MAP REF PATH")
                .help("Path to the financial mapping file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("fin_map_sheet_name")
                .long("fin-map-sheet-name")
                .value_name("FIN MAP Sheet Name")
                .help("Path to the financial mapping sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("exchange_rate_file")
                .long("exchange-rate-file")
                .value_name("EXCHANGE RATE FILE")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_currency")
                .long("base-currency")
                .value_name("BASE CURRENCY")
                .help("The BASE currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("interelimination_ref_path")
                .long("interelimination-ref-path")
                .value_name("INTERELIMINATION REF PATH")
                .help("Path to the interelimination file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("interelimination_sheet_name")
                .long("interelimination-sheet-name")
                .value_name("INTERELIMINATION Sheet Name")
                .help("Path to the interelimination sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("ora_gl_map_ref_path")
                .long("ora-gl-map-ref-path")
                .value_name("ORA GL MAP REF PATH")
                .help("Path to the ORA GL mapping file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("ora_gl_map_sheet_name")
                .long("ora-gl-map-sheet-name")
                .value_name("ORA GL MAP Sheet Name")
                .help("Path to the ORA GL mapping sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_currency")
                .long("input-currency")
                .value_name("Input Currency")
                .help("Consolidated currency in input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file.")
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
                .help("The date for which the program has to run.")
                .required(true)
        )
        .get_matches()
}
