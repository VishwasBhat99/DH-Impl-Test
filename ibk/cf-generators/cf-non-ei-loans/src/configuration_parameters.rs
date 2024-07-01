use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use sdb_day_convention::Conventions;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    repayment_struct_file: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    convention: Conventions,
    holiday_yearrccy_file: String,
    currency: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "convention: {:?}", self.convention());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(
            logger,
            "repayment_struct_file: {}",
            self.repayment_struct_file()
        );
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(
            logger,
            "holiday_yearrccy_file: {:?}",
            self.holiday_yearrccy_file()
        );
        info!(logger, "currency: {:?}", self.currency());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file_value`.")
            .to_string();
        let repayment_struct_file = matches
            .value_of("repayment_struct_file")
            .expect("Error getting `repayment_struct_file`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );

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
        let holiday_yearrccy_file = matches
            .value_of("holiday_yearrccy_file")
            .expect("Error getting `holiday_yearrccy_file`.")
            .to_string();
        let currency = matches
            .value_of("currency")
            .expect("Error getting `currency`.")
            .to_string();
        let conv = matches
            .value_of("convention")
            .expect("Error getting `convention`.")
            .to_string();
        let convention = match conv.as_str() {
            "ACTbyACT" => Conventions::ACTbyACT,
            "ACTby360" => Conventions::ACTby360,
            "Thirtyby360" => Conventions::Thirtyby360,
            _ => Conventions::ACTby365,
        };

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
            repayment_struct_file,
            as_on_date,
            output_file_path,
            log_file_path,
            diagnostics_file_path,
            convention,
            holiday_yearrccy_file,
            currency,
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
    pub fn repayment_struct_file(&self) -> &str {
        &self.repayment_struct_file
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
    pub fn holiday_yearrccy_file(&self) -> &str {
        &self.holiday_yearrccy_file
    }
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn convention(&self) -> &Conventions {
        &self.convention
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
        .about("Cashflow generation for Non Ei Loans")
        .version("1.0.4293")
        .author("srinivas644 <srinivas.r@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("repayment_struct_file")
                .long("repayment-struct-file")
                .value_name("Repayment Structure File")
                .help("Path to the repayment structure file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("Output File")
                .help("Path to the output file")
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
            Arg::with_name("convention")
                .long("convention")
                .value_name("CONVENTION")
                .possible_values(&["ACTbyACT", "ACTby360", "Thirtyby360", "ACTby365"])
                .help("Conventions")
                .default_value("ACTby365")
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
            Arg::with_name("holiday_yearrccy_file")
                .long("holiday-yearccy-file")
                .value_name("Holiday for Year and Currency")
                .help("Path to the Holiday/Working Day Data for all Years and Currencies file.")
                .required(true)
        )
        .arg(
            Arg::with_name("currency")
                .long("ccy")
                .value_name("Currency")
                .help("Path to the repayment structure file.")
                .default_value("KWD")
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
