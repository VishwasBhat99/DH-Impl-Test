use clap;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    repayment_schedule_file_path: String,
    call_date_file: String,
    output_file_path: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    write_int_cashflows: bool,
    is_perf_diagnostics_enabled: bool,
    od_additional_day: i64,
    adjust_to_prev_cf: bool,
    rep_master_file_path: String,
    rep_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(
            logger,
            "repayment_schedule_file_path: {}",
            self.repayment_schedule_file_path()
        );
        info!(logger, "call_date_file: {}", self.call_date_file());
        info!(logger, "rep_sheet_name: {}", self.rep_sheet_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(
            logger,
            "write_int_cashflows: {}",
            self.write_int_cashflows()
        );
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "od_additional_day: {}", self.od_additional_day());
        info!(logger, "adjust_to_prev_cf: {}", self.adjust_to_prev_cf());
        info!(
            logger,
            "rep_master_file_path: {}",
            self.rep_master_file_path()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let rep_master_file_path = matches
            .value_of("rep_master_file_path")
            .expect("Error getting `rep_master_file_path`.")
            .to_string();
        let rep_sheet_name = matches
            .value_of("rep_sheet_name")
            .expect("Error getting `rep_sheet_name`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("`as-on-date` not well-formatted."),
        );
        let repayment_schedule_file_path = matches
            .value_of("repayment_schedule_file_path")
            .expect("Error getting `repayment_schedule_file_path`.")
            .to_string();
        let call_date_file = matches
            .value_of("call_date_file")
            .expect("Error getting `call_date_file`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
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
        let write_int_cashflows = matches
            .value_of("write_int_cashflows")
            .expect("Error getting `write_int_cashflows`.")
            .parse::<bool>()
            .expect("Cannot parse `write_int_cashflows` as bool.");
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");
        let od_additional_day = matches
            .value_of("od_additional_day")
            .expect("Error getting `od_additional_day`.")
            .parse::<i64>()
            .expect("Cannot parse `od_additional_day` as Int32.");
        let adjust_to_prev_cf = matches
            .value_of("adjust_to_prev_cf")
            .expect("Error getting `adjust_to_prev_cf`.")
            .parse::<bool>()
            .expect("Cannot parse `adjust_to_prev_cf` as bool.");

        ConfigurationParameters {
            input_file_path,
            rep_sheet_name,
            repayment_schedule_file_path,
            call_date_file,
            output_file_path,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            rep_master_file_path,
            write_int_cashflows,
            is_perf_diagnostics_enabled,
            od_additional_day,
            adjust_to_prev_cf,
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
    pub fn repayment_schedule_file_path(&self) -> &str {
        &self.repayment_schedule_file_path
    }
    pub fn call_date_file(&self) -> &str {
        &self.call_date_file
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
    pub fn rep_master_file_path(&self) -> &str {
        &self.rep_master_file_path
    }
    pub fn rep_sheet_name(&self) -> &str {
        &self.rep_sheet_name
    }
    pub fn write_int_cashflows(&self) -> bool {
        self.write_int_cashflows
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn od_additional_day(&self) -> i64 {
        self.od_additional_day
    }
    pub fn adjust_to_prev_cf(&self) -> bool {
        self.adjust_to_prev_cf
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This program generates cashflow for loans!!")
        .author("Janani <janani.p@surya-soft.com>")
        .version("1.0.5024")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file-path")
                .value_name("Input File Path")
                .help("Path to input file.")
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
            Arg::with_name("repayment_schedule_file_path")
                .long("repayment-schedule-file-path")
                .value_name("Repayment Schedule File Path")
                .help("Path to Repayment Schedule file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rep_master_file_path")
                .long("next-rep-date-file")
                .value_name("Next Repricing Date file File Path")
                .help("Path to Next Repricing Date file.")
                .required(true)
        )
        .arg(
            Arg::with_name("call_date_file")
                .long("call-date-file")
                .value_name("Call Date File Path")
                .help("Path to Call Date file.")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::with_name("rep_sheet_name")
                .long("rep-sheet-name")
                .value_name("REP SHEET NAME")
                .help("Sheet name for repricing master file")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file-path")
                .value_name("Output File Path")
                .help("Path to output file.")
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
            Arg::with_name("write_int_cashflows")
                .long("write-int-cashflows")
                .value_name("INTEREST CASHFLOW FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether interest amount will be calculated for the output.")
                .default_value("true")
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
            Arg::with_name("od_additional_day")
                .long("od-additional-day")
                .value_name("OD ADDITIONAL DAY")
                .help("This fields decides number of days to be added for Overdue cashflow")
                .default_value(&"0")
                .required(false)
        )
        .arg(
            Arg::with_name("adjust_to_prev_cf")
                .long("adjust-to-prev-cf")
                .value_name("ADJUST TO PREV CF")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether cf to be adjusted to previous cf or not.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
