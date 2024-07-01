use clap::App;
use clap::Arg;

pub fn cmd_arg_parser(app_name: &str) -> clap::ArgMatches {
    let arguments: clap::ArgMatches = App::new(app_name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(
            "This program generates an output file with specified fields from the .cf file having cashflows."
        )
        .arg(
            Arg::with_name("cf_file_path")
                .help("Sets the cashflows file path to use.")
                .required(true)
                .takes_value(true)
                .long("cf-file")
                .short("c")
        )
        .arg(
            Arg::with_name("cf_fields_file_path")
                .help("Sets the cashflows fields file path to use.")
                .required(true)
                .takes_value(true)
                .long("cf-field-file")
                .short("f")
        )
        .arg(
            Arg::with_name("req_field_file_path")
                .help("Sets the required field file path to use.")
                .required(true)
                .takes_value(true)
                .long("required-field-file")
                .short("r")
        )
        .arg(
            Arg::with_name("as_on_date")
                .help("Date for which the program has to run.")
                .required(true)
                .takes_value(true)
                .long("as-on-date")
                .short("a")
        )
        .arg(
            Arg::with_name("output_file_path")
                .help("Sets the output file path to use.")
                .required(true)
                .takes_value(true)
                .long("output-file")
                .short("o")
        )
        .arg(
            Arg::with_name("field_delimiter")
                .help("Sets the field delimiter character to use. Default is '|'.")
                .required(false)
                .takes_value(true)
                .long("delimiter")
                .short("d")
        )
        .get_matches();

    arguments
}
