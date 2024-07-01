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
    input_file_path: String,
    as_on_date: NaiveDate,
    key_field: String,
    delimiter: String,
    round_off_ex_rt: i64,
    output_file_path: String,
    log_file_path: String,
    calc_ir_from_ason: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "delimiter: {}", self.delimiter());
        info!(logger, "key_field: {}", self.key_field());
        info!(logger, "round_off_ex_rt: {}", self.round_off_ex_rt());
        info!(logger, "as_on_date: {:?}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "calc_ir_from_ason: {:?}", self.calc_ir_from_ason());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input-file-path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser
            .parse_opt(
                matches
                    .value_of("as_on_date")
                    .expect("Error getting `as_on_date` value."),
            )
            .expect("Cannot parse `as_on_date` value as `DD-MM-YYYY` format");

        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file` value.")
            .to_string();
        let key_field = matches
            .value_of("key_field")
            .expect("Error getting `key_field` value.")
            .to_string();
        let delimiter = matches
            .value_of("delimiter")
            .expect("Error getting `delimiter` value.")
            .to_string();
        let round_off_ex_rt = matches
            .value_of("round_off_ex_rt")
            .expect("Error getting `round_off_ex_rt` value.")
            .parse::<i64>()
            .expect("Cannot parse round_off_ex_rt provided.");
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let calc_ir_from_ason = matches
            .value_of("calc_ir_from_ason")
            .expect("Error getting `calc_ir_from_ason` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool type");
        ConfigurationParameters {
            input_file_path,
            as_on_date,
            output_file_path,
            delimiter,
            round_off_ex_rt,
            key_field,
            calc_ir_from_ason,
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
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }
    pub fn round_off_ex_rt(&self) -> &i64 {
        &self.round_off_ex_rt
    }
    pub fn key_field(&self) -> &str {
        &self.key_field
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn calc_ir_from_ason(&self) -> &str {
        &self.calc_ir_from_ason
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
        .about("This app generates Cashflows for TD overseas.")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
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
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("delimiter")
                .long("delimiter")
                .value_name("delimiter")
                .help("Field separator used in input file.")
                .required(false)
                .default_value("|")
        )
        .arg(
            Arg::with_name("key_field")
                .long("key-field")
                .value_name("key-field")
                .help("Key field to be used to group accounts")
                .required(true)
                .default_value("foracid")
        )
        .arg(
            Arg::with_name("round_off_ex_rt")
                .long("round-off-ex-rt")
                .value_name("round_off_ex_rt")
                .help("No of decimals to round off ex rt.")
                .required(false)
                .default_value("7")
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics log file.")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("calc_ir_from_ason")
                .long("calculate-int-rt-from-ason")
                .value_name("Calculate IR from AsOn")
                .help("Flag to calculate Interest rate from As on.")
                .possible_values(&["Y", "N"])
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
                .value_name("As On Date")
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
