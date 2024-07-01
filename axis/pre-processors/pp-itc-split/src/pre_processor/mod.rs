use configuration_parameters::ConfigurationParameters;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
mod writers;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    // Initialize a pool of writers.
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let source_map_reader = fs::read_to_string(&config_param.srcmap_file_path())
        .expect("Failed to read source map file!");

    for line in source_map_reader.lines() {
        let new_writer = writers::get_new_writer(line.to_string(), config_param.output_file_path());
        writers_pool.insert(line.to_string(), new_writer);
    }

    let itc_file = match new_buf_rdr(config_param.itc_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.itc_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in itc_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.itc_file_path(),
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();

        let writer = match writers_pool.get_mut(&fields[25].to_string()) {
            Some(writer) => writer,
            None => {
                //If the source id could not be found the output is written to a default file: "NA.txt".
                let new_writer = writers::get_new_writer(
                    "ITC_DEFAULT".to_string(),
                    config_param.output_file_path(),
                );
                writers_pool.insert(fields[25].to_string(), new_writer);
                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool.get_mut(&fields[25].to_string()).unwrap()
            }
        };
        writers::write_data(writer, format!("{}\n", line), log);
    }
}
