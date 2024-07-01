use configuration_parameters::ConfigurationParameters;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::io::prelude::*;
use std::io::Write;

mod config;

pub fn process_name(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    _diag_logger: &Logger,
) {
    // output file writer
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`.",
                config_params.output_file_path(),
                error
            );
        }
    };
    // read existing src map file path
    let old_src_file = match new_buf_rdr(config_params.src_map_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}`.",
            config_params.src_map_file_path(),
            error
        ),
    };
    let mut old_src_map: Vec<String> = Vec::new();
    let mut index = 1;
    for (line_num, lines) in old_src_file.lines().enumerate() {
        match lines {
            Ok(line) => {
                let line_info: Vec<&str> = line.split('|').collect();
                old_src_map.push(line_info[1].to_string());
                write!(writer, "{}", line)
                    .expect("Unable to write existing data to new src map file.");
                index += 1;
            }
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.src_map_file_path(),
                line_num + 1,
                error
            ),
        };
    }
    // read all input files
    let files_config = config::get_files(config_params.input_config_file_path());
    let mut new_src_map: Vec<String> = Vec::new();
    for file in files_config.files {
        let new_src_file = match new_buf_rdr(&file.input_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}`.",
                &file.input_file_path, error
            ),
        };
        for (line_num, lines) in new_src_file.lines().enumerate() {
            match lines {
                Ok(line) => {
                    let line_info: Vec<&str> = line.split('|').collect();
                    new_src_map.push(line_info[1].to_string());
                }
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    &file.input_file_path,
                    line_num + 1,
                    error
                ),
            };
        }
    }
    // create new src map file
    for value in new_src_map {
        if !old_src_map.contains(&value) {
            write!(writer, "{}|{}", index, value)
                .expect("Unable to write existing data to new src map file.");
            index += 1;
        }
    }
}
