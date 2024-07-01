#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]

extern crate clap;
extern crate sdb_io;
#[macro_use]
mod configuration_parameters;
mod merger;

fn main() {
    let app_name = "merge-gl-master";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    merger::process(config_param);
}
