use crate::configuration_parameters::ConfigurationParameters;
use std::fs;

pub struct InputData {
    pub input_data: String,
    pub curr_data: String,
}

pub fn file_read(input: &ConfigurationParameters) -> InputData {
    let data_read = InputData {
        input_data: fs::read_to_string(input.input_file_path())
            .expect("Unable to read the contents of input file."),
        curr_data: fs::read_to_string(input.rule_file_path())
            .expect("Unable to read the contents of currency rule file."),
    };
    data_read
}
