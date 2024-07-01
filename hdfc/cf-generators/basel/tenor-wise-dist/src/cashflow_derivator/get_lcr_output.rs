use crate::cashflow_derivator::convert_datatype::*;
use crate::cashflow_derivator::get_txt_writer;
use crate::configuration_parameters::ConfigurationParameters;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn get_lcr_map(config_params: &ConfigurationParameters) -> HashMap<i64, i64> {
    let lcr_output_path = format!("{}_lcr.txt", &config_params.output_file_path());
    let mut lcr_output_file = get_txt_writer(&lcr_output_path);

    let ret_input_file = match File::open(&config_params.ret_input_file_path()) {
        Ok(ret_input_file) => ret_input_file,
        Err(error) => panic!("{}", error),
    };
    let ret_reader = BufReader::new(ret_input_file);

    let non_ret_input_file = match File::open(&config_params.non_ret_input_file_path()) {
        Ok(non_ret_input_file) => non_ret_input_file,
        Err(error) => panic!("{}", error),
    };
    let non_ret_reader = BufReader::new(non_ret_input_file);

    let mut lcr_map: HashMap<i64, i64> = HashMap::new();
    for line in ret_reader.lines() {
        match line {
            Ok(input_line) => {
                let input_fields = input_line.split('|').collect::<Vec<&str>>();
                lcr_map.insert(str_to_int(input_fields[1]), str_to_int(input_fields[2]));
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }

    for line in non_ret_reader.lines() {
        match line {
            Ok(input_line) => {
                let input_fields = input_line.split('|').collect::<Vec<&str>>();
                lcr_map.insert(str_to_int(input_fields[1]), str_to_int(input_fields[2]));
            }
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }

    for (key, data) in &lcr_map {
        write!(lcr_output_file, "{}|{}\n", &key, &data).expect("Unable to write txt file.");
    }

    lcr_map
}
