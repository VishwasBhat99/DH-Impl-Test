use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    cost_data_pp1_file: String,
    soldim_file: String,
    divdim_file: String,
    prddim_file: String,
    cost_alloc_pp2_file_path: String,
    as_on_date: NaiveDate,
    unique_id_columns: String,
    delimeter: String,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "cost_data_pp1_file: {}", self.cost_data_pp1_file());
        info!(logger, "soldim_file: {}", self.soldim_file());
        info!(logger, "divdim_file: {}", self.divdim_file());
        info!(logger, "prddim_file: {}", self.prddim_file());
        info!(
            logger,
            "cost_alloc_pp2_file_path: {}",
            self.cost_alloc_pp2_file_path()
        );
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "unique_id_columns: {}", self.unique_id_columns());
        info!(logger, "delimeter: {}", self.delimeter());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let cost_data_pp1_file = matches
            .value_of("cost_data_pp1_file")
            .expect("Error getting `cost_data_pp1_file`.")
            .to_string();
        let soldim_file = matches
            .value_of("soldim_file")
            .expect("Error getting `soldim_file`.")
            .to_string();
        let divdim_file = matches
            .value_of("divdim_file")
            .expect("Error getting `divdim_file`.")
            .to_string();
        let prddim_file = matches
            .value_of("prddim_file")
            .expect("Error getting `prddim_file`.")
            .to_string();
        let cost_alloc_pp2_file_path = matches
            .value_of("cost_alloc_pp2_file_path")
            .expect("Error getting `cost_alloc_pp2_file_path`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let unique_id_columns = matches
            .value_of("unique_id_columns")
            .expect("Error getting `unique_id_columns`.")
            .to_string();
        let delimeter = matches
            .value_of("delimeter")
            .expect("Error getting `delimeter`.")
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
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            cost_data_pp1_file,
            soldim_file,
            divdim_file,
            prddim_file,
            cost_alloc_pp2_file_path,
            as_on_date,
            unique_id_columns,
            delimeter,
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
    pub fn cost_data_pp1_file(&self) -> &str {
        &self.cost_data_pp1_file
    }
    pub fn soldim_file(&self) -> &str {
        &self.soldim_file
    }
    pub fn divdim_file(&self) -> &str {
        &self.divdim_file
    }
    pub fn prddim_file(&self) -> &str {
        &self.prddim_file
    }
    pub fn cost_alloc_pp2_file_path(&self) -> &str {
        &self.cost_alloc_pp2_file_path
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn unique_id_columns(&self) -> &str {
        &self.unique_id_columns
    }
    pub fn delimeter(&self) -> &str {
        &self.delimeter
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
        .about("Cost Allocation PP Merger Program!!")
        .version("1.0.3965")
        .author("harsh8501 <harsh.sk@surya-soft.com>")
        .arg(
            Arg::with_name("cost_data_pp1_file")
                .long("cost-data-pp1-file")
                .value_name("Cost Data PP1 File")
                .help("Cost Data PP1 file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("soldim_file")
                .long("soldim-file")
                .value_name("SolDim File")
                .help("SolDim file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("divdim_file")
                .long("divdim-file")
                .value_name("DivDim File")
                .help("DivDim file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("prddim_file")
                .long("prddim-file")
                .value_name("PrdDim File")
                .help("PrdDim file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("cost_alloc_pp2_file_path")
                .long("cost-alloc-pp2-file")
                .value_name("CostAlloc PP2 File Path")
                .help("Path to CostAlloc PP2 file.")
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
            Arg::with_name("unique_id_columns")
                .long("unique-id-columns")
                .value_name("Unique ID Columns")
                .help("Unique ID Columns.")
                .required(true)
        )
        .arg(
            Arg::with_name("delimeter")
                .long("delimeter")
                .value_name("Delimeter")
                .help("Delimeter.")
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
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
