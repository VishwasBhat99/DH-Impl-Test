use chrono::Local;
use clap::{App, Arg};
use rbdate::{DateParser, NaiveDate};
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    input_file_path: String,
    sheet_name: String,
    ref_file_path_1: String,
    ref_file_path_2: String,
    ref_file_path_3: String,
    ref_file_path_4: String,
    alm_master_sheet_name: String,
    ex_rt_file_path: String,
    lcy: String,
    is_consolidated: bool,
    as_on_date: NaiveDate,
    output_file_path_borrowings: String,
    output_file_path_lendings: String,
    concat_file: String,
    rec_output_file_path: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    entity: String,
    is_perf_diagnostics_enabled: bool,
    funding_source_file_path: String,
    funding_source_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_file_path());
        info!(logger, "ref_file_path_1: {}", self.ref_file_path_1());
        info!(logger, "ref_file_path_2: {}", self.ref_file_path_2());
        info!(logger, "ref_file_path_3: {}", self.ref_file_path_3());
        info!(logger, "ref_file_path_4: {}", self.ref_file_path_4());
        info!(
            logger,
            "alm_master_sheet_name: {}",
            self.alm_master_sheet_name()
        );
        info!(logger, "ex_rt_file_path: {}", self.ex_rt_file_path());
        info!(logger, "local_ccy: {}", self.lcy());
        info!(logger, "is_consolidated: {}", self.is_consolidated());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(
            logger,
            "output_file_borrowings: {}",
            self.output_file_path_borrowings()
        );
        info!(logger, "sheet_name: {}", self.sheet_name());
        info!(
            logger,
            "output_file_lendings: {}",
            self.output_file_path_lendings()
        );
        info!(logger, "rec_output_file: {}", self.rec_output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "entity: {}", self.entity());
        info!(logger, "concat_file: {}", self.concat_file());
        info!(
            logger,
            "funding_source_file_path: {}",
            self.funding_source_file_path()
        );
        info!(
            logger,
            "funding_source_sheet_name: {}",
            self.funding_source_sheet_name()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file")
            .expect("Error getting `input_file` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let sheet_name = matches
            .value_of("sheet_name")
            .expect("Error getting `sheet_name` value.")
            .to_string();
        let funding_source_file_path = matches
            .value_of("funding_source_file_path")
            .expect("Error getting `funding_source_file_path` value.")
            .to_string();
        let funding_source_sheet_name = matches
            .value_of("funding_source_sheet_name")
            .expect("Error getting `funding_source_sheet_name` value.")
            .to_string();
        let output_file_path_borrowings = matches
            .value_of("output_file_borrowings")
            .expect("Error getting `output_file_borrowings` value.")
            .to_string();
        let output_file_path_lendings = matches
            .value_of("output_file_lendings")
            .expect("Error getting `output_file_lendings` value.")
            .to_string();
        let rec_output_file_path = matches
            .value_of("rec_output_file")
            .expect("Error getting `rec_output_file_path` value.")
            .to_string();
        let concat_file = matches
            .value_of("concat_file")
            .expect("Error getting `concat_file` value.")
            .to_string();
        let timestamp = Local::now()
            .naive_local()
            .format("%d%m%Y_%H%M%S")
            .to_string();
        let mut log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        log_file_path = log_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let mut diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        diagnostics_file_path = diagnostics_file_path.replace(".txt", "_") + &timestamp + ".txt";
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let entity = matches
            .value_of("entity")
            .expect("Error getting `entity` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");
        let ref_file_path_1 = matches
            .value_of("ref_file_1")
            .expect("Error getting `ref_file_1` value.")
            .to_string();
        let ref_file_path_2 = matches
            .value_of("ref_file_2")
            .expect("Error getting `ref_file_2` value.")
            .to_string();
        let ref_file_path_3 = matches
            .value_of("ref_file_3")
            .expect("Error getting `ref_file_3` value.")
            .to_string();
        let ref_file_path_4 = matches
            .value_of("ref_file_4")
            .expect("Error getting `ref_file_4` value.")
            .to_string();
        let alm_master_sheet_name = matches
            .value_of("alm_master_sheet_name")
            .expect("Error getting `alm_master_sheet_name` value.")
            .to_string();
        let ex_rt_file_path = matches
            .value_of("ex_rt_file")
            .expect("Error getting `File level exchange rate file path`.")
            .to_string();
        let lcy = matches
            .value_of("lcy")
            .expect("Error getting `local currency`.")
            .to_string();
        let is_consolidated = matches
            .value_of("is_consolidated")
            .expect("Error getting `is_consolidated` flag.")
            .parse::<bool>()
            .expect("Error while parsing `is_consolidated` flag.");
        ConfigurationParameters {
            input_file_path,
            sheet_name,
            ref_file_path_1,
            ref_file_path_2,
            ref_file_path_3,
            ref_file_path_4,
            alm_master_sheet_name,
            ex_rt_file_path,
            lcy,
            is_consolidated,
            as_on_date,
            output_file_path_borrowings,
            output_file_path_lendings,
            concat_file,
            rec_output_file_path,
            log_file_path,
            diagnostics_file_path,
            log_level,
            entity,
            is_perf_diagnostics_enabled,
            funding_source_file_path,
            funding_source_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn ref_file_path_1(&self) -> &str {
        &self.ref_file_path_1
    }
    pub fn sheet_name(&self) -> &str {
        &self.sheet_name
    }
    pub fn ref_file_path_2(&self) -> &str {
        &self.ref_file_path_2
    }
    pub fn ref_file_path_3(&self) -> &str {
        &self.ref_file_path_3
    }
    pub fn ref_file_path_4(&self) -> &str {
        &self.ref_file_path_4
    }
    pub fn funding_source_file_path(&self) -> &str {
        &self.funding_source_file_path
    }
    pub fn funding_source_sheet_name(&self) -> &str {
        &self.funding_source_sheet_name
    }
    pub fn alm_master_sheet_name(&self) -> &str {
        &self.alm_master_sheet_name
    }
    pub fn ex_rt_file_path(&self) -> &str {
        &self.ex_rt_file_path
    }
    pub fn lcy(&self) -> &str {
        &self.lcy
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn output_file_path_borrowings(&self) -> &str {
        &self.output_file_path_borrowings
    }
    pub fn output_file_path_lendings(&self) -> &str {
        &self.output_file_path_lendings
    }
    pub fn concat_file(&self) -> &str {
        &self.concat_file
    }
    pub fn rec_output_file_path(&self) -> &str {
        &self.rec_output_file_path
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
    pub fn entity(&self) -> &str {
        &self.entity
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.3.5458")
        .author("Sougata Bhattacharjee <sougata.b@surya-soft.com>")
        .about("Pre-processor for Murex Borrowings and Lendings")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("INPUT_FILE")
                .help("Path to the input file.")
                .required(true)
        )
        .arg(
            Arg::with_name("funding_source_file_path")
                .long("funding-source-file-path")
                .value_name("Funding source file path")
                .help("Path to Funding source file.")
                .required(true)
        )
        .arg(
            Arg::with_name("funding_source_sheet_name")
                .long("funding-source-sheet-name")
                .value_name("funding source sheet name")
                .help("funding source sheet name.")
                .required(true)
        )
        .arg(
            Arg::with_name("sheet_name")
                .long("sheet-name")
                .value_name("SHEET_NAME")
                .help("Sheet name in input excel file.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_1")
                .long("ref-file-1")
                .value_name("REF_FILE_1")
                .help("Path to the reference files: R1.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_2")
                .long("ref-file-2")
                .value_name("REF_FILE_2")
                .help("Path to the reference files: R2.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_3")
                .long("ref-file-3")
                .value_name("REF_FILE_3")
                .help("Path to the reference files: R3.")
                .required(true)
        )
        .arg(
            Arg::with_name("ref_file_4")
                .long("ref-file-4")
                .value_name("REF_FILE_4")
                .help("Path to the reference files: R4.")
                .required(true)
        )
        .arg(
            Arg::with_name("alm_master_sheet_name")
                .long("alm-master-sheet-name")
                .value_name("alm_master_sheet_name")
                .help("Alm Master File Sheet Name.")
                .required(true)
        )
        .arg(
            Arg::with_name("ex_rt_file")
                .long("ex-rt-file")
                .value_name("Exchange Rate File Path")
                .help("The path to the exchange rate file.")
                .required(true)
        )
        .arg(
            Arg::with_name("lcy")
                .long("lcy")
                .value_name("Local Currency")
                .help("Local Currency.")
                .required(true)
        )
        .arg(
            Arg::with_name("is_consolidated")
                .long("is-consolidated")
                .value_name("is_consolidated")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether the currency is consolidated or not.")
                .default_value("false")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file_borrowings")
                .long("output-file-borrowings")
                .value_name("Borrowings Output File")
                .help("Path to the borrowings output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_lendings")
                .long("output-file-lendings")
                .value_name("Lendings Output File")
                .help("Path to the lendings output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("concat_file")
                .long("concat-file")
                .value_name("Concat File")
                .help("Path to the Concat file.")
                .required(true)
        )
        .arg(
            Arg::with_name("rec_output_file")
                .long("rec-output-file")
                .value_name("Reconcilation Output File")
                .help("Path to the reconcilation output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
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
            Arg::with_name("entity")
                .long("entity")
                .value_name("ENTITY")
                .help("The entity for which the program has to run.")
                .required(true)
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
