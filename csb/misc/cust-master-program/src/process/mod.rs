use configuration_parameters::ConfigurationParameters;
use slog::Logger;
use std::io::BufRead;
use std::time::{Duration, SystemTime};

mod io;

pub struct Fields {
    pub T1: String,
    pub T2: String,
    pub T3: String,
    pub T4: String,
}
pub fn process_name(config_params: ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let input_file_path = io::read_file(&config_params.input_file_path);
    let mut output_file_path = io::create_file(&config_params.output_file_path);
    let open_acc_file_data = io::read(&config_params.open_acc_file_path);
    let mut total_duration = Duration::new(0, 0);
    for line_iter in input_file_path.lines() {
        let line = line_iter.expect("Error while reading the line from the input file.");
        let cust_master_fields: Vec<&str> = line.split("|").collect();
        let search_start_time = SystemTime::now();
        if search_client_code(&open_acc_file_data, cust_master_fields[1]) {
            let search_end_time = SystemTime::now();
            let duration = search_end_time.duration_since(search_start_time).expect(
                "Could not calculate total duration for search_client_code function timer.",
            );
            total_duration += duration;
            let fields = Fields {
                T1: String::from("N"),
                T2: String::from("N"),
                T3: String::from("Y"),
                T4: String::from("Y"),
            };
            io::write_file(fields, &mut output_file_path, line.to_string());
        } else {
            let search_end_time = SystemTime::now();
            let duration = search_end_time.duration_since(search_start_time).expect(
                "Could not calculate total duration for search_client_code function timer.",
            );
            total_duration += duration;
            let fields = Fields {
                T1: String::from("N"),
                T2: String::from("N"),
                T3: String::from("Y"),
                T4: String::from("N"),
            };
            io::write_file(fields, &mut output_file_path, line.to_string());
        }
    }
    debug!(
        logger,
        "Total time taken for execution of the function search_client_code: {:?}.", total_duration
    );
}

pub fn search_client_code(open_acc_file_data: &Vec<String>, client_code: &str) -> bool {
    let result = open_acc_file_data.iter().find(|&val| val == client_code);
    match result {
        Some(client_code) => true,
        None => false,
    }
}
