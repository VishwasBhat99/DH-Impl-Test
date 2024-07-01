use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_rec_file_path: String,
    pub input_pay_file_path: String,
    pub skip_rows: Vec<String>,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub req_fields_file: String,
    pub bucket_config_file: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "input_file: {}", self.input_rec_file_path());
        info!(logger, "input_file: {}", self.input_pay_file_path());
        info!(logger, "skip_rows: {:?}", self.skip_rows());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "output_file: {}", self.output_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(logger, "req_fields_file: {}", self.req_fields_file());
        info!(logger, "bucket_config_file: {}", self.bucket_config_file());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_rec_file_path = matches
            .value_of("input_rec_file")
            .expect("Error getting `input_rec_file` value.")
            .to_string();
        let req_fields_file = matches
            .value_of("req_fields_file")
            .expect("Error getting `req_fields_file` value.")
            .to_string();
        let bucket_config_file = matches
            .value_of("bucket_config_file")
            .expect("Error getting `bucket_config_file` value.")
            .to_string();
        let input_pay_file_path = matches
            .value_of("input_pay_file")
            .expect("Error getting `input_pay_file` value.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let skip_rows: Vec<String> = matches
            .value_of("skip_rows")
            .expect("Error getting `skip_rows`.")
            .to_string()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file` value.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file` value.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level` value.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag` value.")
            .parse::<bool>()
            .expect("Cannot parse `perf_diag_flag` value as bool.");

        ConfigurationParameters {
            input_rec_file_path,
            input_pay_file_path,
            as_on_date,
            output_file_path,
            log_file_path,
            skip_rows,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
            req_fields_file,
            bucket_config_file,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_rec_file_path(&self) -> &str {
        &self.input_rec_file_path
    }
    pub fn input_pay_file_path(&self) -> &str {
        &self.input_pay_file_path
    }
    pub fn skip_rows(&self) -> &Vec<String> {
        &self.skip_rows
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
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn req_fields_file(&self) -> &str {
        &self.req_fields_file
    }
    pub fn bucket_config_file(&self) -> &str {
        &self.bucket_config_file
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("This app pre-processes the swaps duration moc files.")
        .version("1.1.4738")
        .author("Tanuj Singh Rathore<tanuj.s@surya-soft.com>")
        .arg(
            Arg::with_name("input_rec_file")
                .long("input-rec-file")
                .value_name("input_rec_file")
                .help("Path to input rec file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("bucket_config_file")
                .long("bucket-config-file")
                .value_name("bucket_config_file")
                .help("Path to bucket config file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("req_fields_file")
                .long("config-file")
                .value_name("req_fields_file")
                .help("Path to config file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("input_pay_file")
                .long("input-pay-file")
                .value_name("input_pay_file")
                .help("Path to input pay file that needs to be processed.")
                .required(true)
        )
        .arg(
            Arg::with_name("skip_rows")
                .long("skip-rows")
                .value_name("Skip Rows")
                .help("This value tells about the rows to be skipped from processing")
                .default_value("")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("FILE")
                .help("Path to the output file.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("FILE")
                .help("Path to write general logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("FILE")
                .help("Path to write diagnostics log.")
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
                .help("The date the program assumes as 'today'.")
                .required(true)
        )
        .get_matches()
}
