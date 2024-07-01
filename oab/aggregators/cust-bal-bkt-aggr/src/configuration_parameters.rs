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
    final_input_file_path: String,
    output_file_path: String,
    llg_mapper_file_path: String,
    country_code: String,
    allocation_order: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file_path: {}", self.input_file_path());
        info!(logger, "output_file_path: {}", self.output_file_path());
        info!(
            logger,
            "final_input_file_path: {}",
            self.final_input_file_path()
        );
        info!(
            logger,
            "llg_mapper_file_path: {}",
            self.llg_mapper_file_path()
        );
        info!(logger, "country_code: {}", self.country_code());
        info!(logger, "allocation_order: {}", self.allocation_order());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path`.")
            .to_string();
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path`.")
            .to_string();
        let final_input_file_path = matches
            .value_of("final_input_file_path")
            .expect("Error getting `final_input_file_path`.")
            .to_string();
        let allocation_order = matches
            .value_of("allocation_order")
            .expect("Error getting `allocation_order`.")
            .to_string();
        let llg_mapper_file_path = matches
            .value_of("llg_mapper_file_path")
            .expect("Error getting `llg_mapper_file_path`.")
            .to_string();
        let country_code = matches
            .value_of("country_code")
            .expect("Error getting `country_code`.")
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
            final_input_file_path,
            output_file_path,
            allocation_order,
            llg_mapper_file_path,
            country_code,
            as_on_date,
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
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn final_input_file_path(&self) -> &str {
        &self.final_input_file_path
    }
    pub fn allocation_order(&self) -> &str {
        &self.allocation_order
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn llg_mapper_file_path(&self) -> &str {
        &self.llg_mapper_file_path
    }
    pub fn country_code(&self) -> &str {
        &self.country_code
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
        .about("Customer Balance BucketWise Aggregator!!")
        .version("1.0.4194")
        .author("VishwasBhat99 <vishwas.b@surya-soft.com>")
        .arg(
            Arg::with_name("input_file_path")
                .long("input-file")
                .value_name("Input File Path")
                .help("Path to Retail or Non-Retail Cust-Bal-Aggr File.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_path")
                .long("output-file")
                .value_name("Output File Path")
                .help("Path to Output File.")
                .required(true)
        )
        .arg(
            Arg::with_name("final_input_file_path")
                .long("final-input-file")
                .value_name("Final Input File Path")
                .help("Path to Retail or Non-Retail Final File.")
                .required(true)
        )
        .arg(
            Arg::with_name("allocation_order")
                .long("allocation-order")
                .value_name("Allocation Order")
                .help("Value which tells on whether to allocate amounts from (first->last) buckets or (last->first) buckets.")
                .possible_values(&["ASC","DESC"])
                .default_value("ASC")
                .required(false)
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
            Arg::with_name("llg_mapper_file_path")
                .long("llg-mapper-file")
                .value_name("LLG Mapper File Path")
                .help("Path to LLG Mapper File.")
                .required(true)
        )
        .arg(
            Arg::with_name("country_code")
                .long("country")
                .value_name("Country Code")
                .help("Country Code to be Stamped in Output.")
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
