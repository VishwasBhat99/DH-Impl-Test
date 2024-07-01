use configuration_parameters::ConfigurationParameters;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use std::env::current_dir;
use std::io::prelude::*;

pub fn process(config_param: ConfigurationParameters) {
    let static_adder_file = match new_buf_rdr(config_param.static_adder()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.static_adder(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut static_adder = Vec::new();
    for (line_num, lines) in static_adder_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.static_adder(),
                line_num + 1,
                error
            ),
        };
        static_adder.push(line.trim().to_string());
    }

    let static_remover_file = match new_buf_rdr(config_param.static_remover()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.static_remover(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut static_remover = Vec::new();
    for (line_num, lines) in static_remover_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.static_remover(),
                line_num + 1,
                error
            ),
        };
        static_remover.push(line.trim().to_string());
    }

    let dynamic_master_file = match new_buf_rdr(config_param.dynamic_master()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.dynamic_master(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut dynamic_master = Vec::new();
    for (line_num, lines) in dynamic_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.dynamic_master(),
                line_num + 1,
                error
            ),
        };
        dynamic_master.push(line.trim().to_string());
    }

    dynamic_master.append(&mut static_adder);

    for gl_rem in static_remover.iter() {
        dynamic_master.retain(|gl| gl != gl_rem);
    }

    dynamic_master.sort();
    dynamic_master.dedup();

    let mut output_line: String = String::new();
    for gl in dynamic_master.iter() {
        output_line.push_str(gl);
        output_line.push('\n');
    }

    let mut writer = match buf_file_wrtr(config_param.excld_master(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.excld_master(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match writer.write_all(output_line.as_bytes()) {
        Ok(_) => println!("Successfully merged master files to GLExcludeMaster.txt."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.excld_master(),
            error,
        ),
    }
}
