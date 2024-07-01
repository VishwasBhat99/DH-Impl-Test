#[macro_use]
extern crate serde_derive;

mod configuration_parameters;
mod mapper;
mod statics;

use std::time::SystemTime;

fn main() {
    let start_time_main = SystemTime::now();
    let app_name = "bills-pp";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    mapper::process(config_param);

    let end_time_main = SystemTime::now();
    let total_duration = end_time_main
        .duration_since(start_time_main)
        .expect("Could not calculate total duration for main timer.");
    println!(
        "Total Duration taken by FTP old to new format converter: {:?}",
        total_duration
    );
}
