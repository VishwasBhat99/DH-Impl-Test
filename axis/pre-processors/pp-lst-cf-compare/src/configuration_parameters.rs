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
    input_cf_file_path: String,
    input_lst_file_path: String,
    cf_metadata_file_path: String,
    lst_metadata_file_path: String,
    base_output_file_path: String,
    config_file_path: String,
    output_file1_name: String,
    output_file2_name: String,
    output_file3_name: String,
    output_file4_name: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_cf_file_path: {}", self.input_cf_file_path());
        info!(
            logger,
            "input_lst_file_path: {}",
            self.input_lst_file_path()
        );
        info!(
            logger,
            "cf_metadata_file_path: {}",
            self.cf_metadata_file_path()
        );
        info!(
            logger,
            "lst_metadata_file_path: {}",
            self.lst_metadata_file_path()
        );
        info!(logger, "config_file_path: {}", self.config_file_path());
        info!(logger, "base_output_file: {}", self.base_output_file_path());
        info!(logger, "output_file1_name: {}", self.output_file1_name());
        info!(logger, "output_file2_name: {}", self.output_file2_name());
        info!(logger, "output_file3_name: {}", self.output_file3_name());
        info!(logger, "output_file4_name: {}", self.output_file4_name());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_cf_file_path = matches
            .value_of("input_cf_file_path")
            .expect("Error getting `input_cf_path_file`.")
            .to_string();
        let input_lst_file_path = matches
            .value_of("input_lst_file_path")
            .expect("Error getting `input_lst_path_file`.")
            .to_string();
        let cf_metadata_file_path = matches
            .value_of("cf_metadata_file_path")
            .expect("Error getting `cf_metadata_file_path`.")
            .to_string();
        let lst_metadata_file_path = matches
            .value_of("lst_metadata_file_path")
            .expect("Error getting `lst_metadata_file_path`.")
            .to_string();
        let config_file_path = matches
            .value_of("config_file_path")
            .expect("Error while getting `config file path`.")
            .to_string();
        let base_output_file_path = matches
            .value_of("base_output_file")
            .expect("Error while getting `base output report file path`.")
            .to_string();
        let output_file1_name = matches
            .value_of("output_file1_name")
            .expect("Error while getting `base output file1 name`.")
            .to_string();
        let output_file2_name = matches
            .value_of("output_file2_name")
            .expect("Error while getting `base output file2 name`.")
            .to_string();
        let output_file3_name = matches
            .value_of("output_file3_name")
            .expect("Error while getting `base output file3 name`.")
            .to_string();
        let output_file4_name = matches
            .value_of("output_file4_name")
            .expect("Error while getting `base output file4 name`.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting as on date as DD-MM-YYYY."),
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
            input_cf_file_path,
            input_lst_file_path,
            cf_metadata_file_path,
            lst_metadata_file_path,
            base_output_file_path,
            config_file_path,
            output_file1_name,
            output_file2_name,
            output_file3_name,
            output_file4_name,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properateies (they're private).
// Also, because users of these properateies usually borrow.
impl ConfigurationParameters {
    pub fn input_cf_file_path(&self) -> &str {
        &self.input_cf_file_path
    }
    pub fn input_lst_file_path(&self) -> &str {
        &self.input_lst_file_path
    }
    pub fn cf_metadata_file_path(&self) -> &str {
        &self.cf_metadata_file_path
    }
    pub fn lst_metadata_file_path(&self) -> &str {
        &self.lst_metadata_file_path
    }
    pub fn base_output_file_path(&self) -> &str {
        &self.base_output_file_path
    }
    pub fn config_file_path(&self) -> &str {
        &self.config_file_path
    }
    pub fn output_file1_name(&self) -> &str {
        &self.output_file1_name
    }
    pub fn output_file2_name(&self) -> &str {
        &self.output_file2_name
    }
    pub fn output_file3_name(&self) -> &str {
        &self.output_file3_name
    }
    pub fn output_file4_name(&self) -> &str {
        &self.output_file4_name
    }
    pub fn as_on_date(&self) -> String {
        self.as_on_date.format("%Y%m%d").to_string()
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
        .about("IR-Control Report for cfs with similar TD value month and matchfieldss.")
        .arg(
            Arg::with_name("input_cf_file_path")
                .long("input-cf-file-path")
                .value_name("Input cf file path")
                .help("Path to cf input file. ")
                .required(true)
        )
        .arg(
            Arg::with_name("input_lst_file_path")
                .long("input-lst-file-path")
                .value_name("Input lst file path")
                .help("Path to lst file. ")
                .required(true)
        )
       .arg(
            Arg::with_name("cf_metadata_file_path")
                .long("cf-metadata-file-path")
                .value_name("cf Metadata File")
                .help("Path to json file that has cf metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("lst_metadata_file_path")
                .long("lst-metadata-file-path")
                .value_name("lst Metadata File")
                .help("Path to json file that has lst metadata.")
                .required(true)
        )
        .arg(
            Arg::with_name("base_output_file")
                .long("base-output-file")
                .value_name("Base Output File")
                .help("Base path of the output files")
                .required(true)
        )
        .arg(
            Arg::with_name("config_file_path")
                .long("config-file-path")
                .value_name("Config File Path")
                .help("Config file path")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file1_name")
                .long("output-file1-name")
                .value_name("Output File 1 name")
                .help("Output file 1 name")
                .required(true)
        ).arg(
            Arg::with_name("output_file2_name")
                .long("output-file2-name")
                .value_name("Output File 2 name")
                .help("Output file 2 name")
                .required(true)
        ).arg(
            Arg::with_name("output_file3_name")
                .long("output-file3-name")
                .value_name("Output File 3 name")
                .help("Output file 3 name")
                .required(true)
        ).arg(
            Arg::with_name("output_file4_name")
                .long("output-file4-name")
                .value_name("Output File 4 name")
                .help("Output file 4 name")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("As On Date")
                .help("The date for which the program has to be processed.")
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
