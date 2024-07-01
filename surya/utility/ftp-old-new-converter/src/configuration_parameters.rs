use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub input_file_path: String,
    pub as_on_date: NaiveDate,
    pub output_file_path: String,
    pub a_or_l: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file_path = matches
            .value_of("input_file_path")
            .expect("Error getting `input_file_path` value.")
            .to_string();

        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date` value."),
        );
        let output_file_path = matches
            .value_of("output_file_path")
            .expect("Error getting `output_file_path` value.")
            .to_string();
        let a_or_l = matches
            .value_of("a_or_l")
            .expect("Error getting `a_or_l` value.")
            .to_string();

        ConfigurationParameters {
            input_file_path,
            as_on_date,
            output_file_path,
            a_or_l,
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
    pub fn a_or_l(&self) -> &str {
        &self.a_or_l
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .version("1.2.0")
        .author("Tarun Jaiswal <tarun.j@surya-soft.com>")
        .about("This app converts old ftp summary output to new one!")
        .arg(
            Arg::with_name("input_file_path")
                .short("i")
                .long("input-file-path")
                .value_name("Input File")
                .help("Path to input file that needs to be processed.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_file_path")
                .short("o")
                .long("output-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("a_or_l")
                .short("r")
                .long("a-or-l")
                .value_name("Asset or Liability")
                .help("Assign Asset or Liability field.")
                .possible_values(&["A", "L"])
                .default_value("A")
                .required(false),
        )
        .arg(
            Arg::with_name("as_on_date")
                .short("a")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which program has to run.")
                .required(true),
        )
        .get_matches()
}
