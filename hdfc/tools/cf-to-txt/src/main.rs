#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp))]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod lib;

fn main() {
    let arguments: clap::ArgMatches = lib::cmd_parser::cmd_arg_parser("Cashflows Reader");

    let (cf_file_path, cf_fields_file_path, requiredfields_file_path, output_file_path, delimiter) =
        lib::validate_cmd_arguments::get_file_reference(&arguments);

    lib::output_generator::generate_result(
        &cf_file_path,
        &cf_fields_file_path,
        &requiredfields_file_path,
        &output_file_path,
        &delimiter,
    );

    // Output file location is represented wrt the current location.
    println!("Successfully generated output file at {}", output_file_path);
}
