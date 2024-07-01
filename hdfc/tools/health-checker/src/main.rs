#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(clippy::panicking_unwrap, clippy::unwrap_used)
)]

extern crate clap;
extern crate health_report;

mod configuration_parameters;
mod health_checker;
mod statics;

use health_checker::process;

fn main() {
    let app_name = "health_checker";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    process(config_param);
}
