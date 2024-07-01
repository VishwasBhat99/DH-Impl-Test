use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::split_pos::get_split_pos;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
pub mod config;
mod split_pos;

pub fn maskdata(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.input_files {
        let input_file = match new_buf_rdr(&file.input_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not find file `{}` at location `{}` : {}.",
                file.input_file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        //Splitting input file path to get filename.
        let input_path_split: Vec<&str> = file.input_file_path.split('/').collect();
        let filename = input_path_split
            .last()
            .expect("Could not determine name of input file.");

        //Append input file name to output file path.
        //Output path is of format: /folder1/folder2/
        let full_output_path = format!("{}{}", file.output_path, filename);

        let skip_lines = file.no_lines_skipped;
        let mut output_line = String::new();
        let mut writer = match buf_file_wrtr(&full_output_path, None) {
            Ok(file) => file,
            Err(error) => panic!(
                "Unable to create output file: `{}` at location `{}` : {}",
                full_output_path,
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error,
            ),
        };

        //Get the positions at which data is to be masked.
        let mut split_val = get_split_pos(&file.position_changed);
        //Sort the array according to starting position.
        split_val.sort_by(|d1, d2| d1.st_pos.cmp(&d2.st_pos));

        let replace_char = file.replace_char.to_string();
        for (index, line) in input_file.lines().enumerate() {
            acc_enc += 1;
            let acc_info: String = match line {
                Ok(acc_info) => acc_info,
                Err(error) => {
                    log_error!(logger, "Failed to read line from input file: {:?}", error);
                    continue;
                }
            };

            //Skip headers.
            if index < skip_lines {
                output_line.push_str(&acc_info);
                output_line.push('\n');
                acc_succ += 1;
                continue;
            }

            let mut st_pos = 0_usize;
            for mask_range in &split_val {
                if st_pos != mask_range.st_pos {
                    output_line.push_str(
                        acc_info
                            .get(st_pos..mask_range.st_pos as usize)
                            .expect("Could not get index for a value."),
                    );
                }
                let replace_string = replace_char.repeat(mask_range.length);

                output_line.push_str(&replace_string);
                st_pos = mask_range.end_pos;
            }
            let trailing_fields = format!(
                "{}\n",
                &acc_info
                    .get(st_pos..acc_info.len())
                    .expect("Could not get trailing values from file.")
            );
            output_line.push_str(&trailing_fields);
            acc_succ += 1;
        }
        write_data(&mut writer, output_line, logger);

        let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
        health_report.gen_health_rpt(&full_output_path);
    }
    fn write_data(writer: &mut BufWriter<File>, op: String, logger: &Logger) {
        let output_as_bytes = op.as_bytes();
        match writer.write(output_as_bytes) {
            Ok(_val) => {}
            Err(err) => {
                log_info!(logger, "Error writing to output file. Error: {}", err);
            }
        }
    }
}
