pub fn get_file_reference<'a>(
    matches: &'a clap::ArgMatches<'a>,
) -> (&'a str, &'a str, &'a str, &'a str, &'a str) {
    let cf_file_path: &str = match matches.value_of("cf_file_path") {
        Some(file_path) => file_path,
        None => {
            panic!("Cashflows file path not found.");
        }
    };

    let cf_fields_file_path: &str = match matches.value_of("cf_fields_file_path") {
        Some(file_path) => file_path,
        None => {
            panic!("Cashflows fields file path not found.");
        }
    };

    let requiredfields_file_path: &str = match matches.value_of("req_field_file_path") {
        Some(file_path) => file_path,
        None => {
            panic!("Required feilds file path not found.");
        }
    };

    let output_file_path: &str = match matches.value_of("output_file_path") {
        Some(file_path) => file_path,
        None => {
            panic!("Output file path not found.");
        }
    };

    let field_delimiter: &str = match matches.value_of("field_delimiter") {
        Some(del) => del,
        None => "|",
    };

    (
        cf_file_path,
        cf_fields_file_path,
        requiredfields_file_path,
        output_file_path,
        field_delimiter,
    )
}
