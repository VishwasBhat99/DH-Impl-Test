use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use sdb_day_convention::Conventions;
use slog::Logger;

pub fn get_configuration_parameters() -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app();
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file_path: String,
    repayment_struct_file: String,
    overdue_input_file: String,
    adj_cf_type: String,
    as_on_date: NaiveDate,
    output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    convention: Conventions,
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
            "overdue_input_file: {:?}",
            self.overdue_input_file()
        );
        info!(logger, "adj_cf_type: {:?}", self.adj_cf_type());
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
        let overdue_input_file = matches
            .value_of("overdue_input_file")
            .expect("Error getting `overdue_input_file`.")
            .to_string();
        let adj_cf_type = matches
            .value_of("adj_cf_type")
            .expect("Error getting `adj_cf_type`.")
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
            overdue_input_file,
            adj_cf_type,
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
    pub fn overdue_input_file(&self) -> &str {
        &self.overdue_input_file
    }
    pub fn adj_cf_type(&self) -> &str {
        &self.adj_cf_type
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

fn get_eligible_arguments_for_app() -> clap::ArgMatches<'static> {
    App::new(clap::crate_name!())
        .about("Cashflow Generation for EMI Loans (SIB)")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
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
            Arg::with_name("overdue_input_file")
                .long("overdue-input-file")
                .value_name("OVERDUE")
                .help("Path to the Overdue Cashflows Data.")
                .required(true)
        )
        .arg(
            Arg::with_name("adj_cf_type")
                .long("adj-cf-type")
                .value_name("ADJ_CF_TYPE")
                .help("Decides where to write the adjustment cashflows.")
                .possible_values(&["LAST", "NEXT"])
                .default_value("LAST")
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
