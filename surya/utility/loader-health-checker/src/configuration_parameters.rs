use clap::{App, Arg};

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub output_file_path: String,
    pub loader_log_file_path: String,
    pub loader_flag: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let output_file_path = matches
            .value_of("output_file")
            .expect("Error getting `output_file` value.")
            .to_string();
        let loader_log_file_path = matches
            .value_of("loader_log_file_path")
            .expect("Error getting `loader_log_file_path` value.")
            .to_string();
        let loader_flag = matches
            .value_of("loader_flag")
            .expect("Error getting `loader_flag` value.")
            .to_string();
        ConfigurationParameters {
            output_file_path,
            loader_log_file_path,
            loader_flag,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn output_file_path(&self) -> &str {
        &self.output_file_path
    }
    pub fn loader_log_file_path(&self) -> &str {
        &self.loader_log_file_path
    }
    pub fn loader_flag(&self) -> &str {
        &self.loader_flag
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Loader Health Check Report Writer!")
        .version("1.0.3094")
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .short("o")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("loader_log_file_path")
                .long("loader-log-file")
                .short("l")
                .value_name("Log File")
                .help("Path to the loader log file.")
                .required(true),
        )
        .arg(
            Arg::with_name("loader_flag")
                .long("loader-flag")
                .short("f")
                .value_name("Loader Flag")
                .help("Loader flag for Oracle will be oracle and for MS SQL mssql")
                .possible_values(&["oracle", "mssql", "MSSQL", "ORACLE"])
                .required(true),
        )
        .get_matches()
}
