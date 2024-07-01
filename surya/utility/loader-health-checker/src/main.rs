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
extern crate health_report;
extern crate sdb_io;

mod configuration_parameters;
mod health_checker;

use health_checker::process;

fn main() {
    let app_name = "loader_hc_report_gen";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    process(config_param);
}
