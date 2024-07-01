use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::convert::TryInto;
use std::env::current_dir;
use std::io::prelude::*;
use xlsxwriter::*;
pub mod config;

pub fn maskdata(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc: i64 = 0;
    let mut acc_succ: i64 = 0;
    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.input_files {
        let input_file = match new_buf_rdr(&file.file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not find file `{}` at location `{}` : {}.",
                file.file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        //Splitting input file path to get filename.
        let input_path_split: Vec<&str> = file.file_path.split('/').collect();
        let filename = input_path_split
            .last()
            .expect("Could not determine name of input file.");

        //Append input file name to output file path.
        //Output path is of format: /folder1/folder2/
        let full_output_path = format!("{}{}", file.output_path, filename);

        match file.file_type.as_str() {
            "txt" | "csv" => {
                let position_considered = file.position_considered[0] - 1;
                let positions_to_change = &file.position_changed[0];
                let skip_lines = file.no_lines_skipped[0];
                let separator = file.separator;
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

                for (line_num, lines) in input_file.lines().enumerate() {
                    acc_enc += 1;
                    let line = match lines {
                        Ok(line) => line,
                        Err(error) => panic!(
                            "Unable to read file `{}` at line number: `{}` : {}",
                            file.file_path,
                            line_num + 1,
                            error
                        ),
                    };
                    let fields: Vec<&str> = line.split(&separator).collect();
                    let key = fields[position_considered];
                    for (index, mut val) in fields.iter().enumerate() {
                        //Check if line is not to be skipped.
                        if line_num >= skip_lines && positions_to_change.contains(&(index + 1)) {
                            val = &key;
                        }
                        output_line.push_str(val);
                        output_line.push_str(&separator);
                    }
                    //Remove separator at the end.
                    for ind in 0..separator.len() {
                        output_line.pop();
                    }
                    output_line.push('\n');
                    acc_succ += 1;
                }
                match writer.write_all(output_line.as_bytes()) {
                    Ok(_) => log_info!(
                        logger,
                        "Successfully written to output file:{}.",
                        full_output_path
                    ),
                    Err(error) => panic!(
                        "Unable to write processed lines to file `{}`: {}.",
                        full_output_path, error,
                    ),
                }
            }
            "xlsx" => {
                let mut input_file = open_workbook_auto(&file.file_path)
                    .expect("Unable to open the input xlsx file.");
                let workbook = Workbook::new(&full_output_path);
                let positions_considered = file.position_considered;
                let positions_to_change_vec = file.position_changed;
                for (index, sheet_name) in file.sheet_names.iter().enumerate() {
                    let mut sheet = workbook
                        .add_worksheet(Some(sheet_name))
                        .expect("Could not add sheet to excel.");

                    if let Some(Ok(reader)) = input_file.worksheet_range(sheet_name) {
                        //Case: No fields are to be masked in a sheet: 0 in input implies no position to be considered.
                        if positions_considered.get(index) <= Some(&0) {
                            //Considering x as row number and y as column number.
                            for (x, row) in reader.rows().enumerate() {
                                acc_enc += 1;
                                let mut y = 0;
                                while y < row.len() {
                                    sheet
                                        .write_string(
                                            x.try_into().unwrap(),
                                            y.try_into().unwrap(),
                                            &row[y].to_string(),
                                            None,
                                        )
                                        .expect("Could not write to sheet.");
                                    y += 1;
                                }
                                acc_succ += 1;
                            }
                            continue;
                        }
                        let position_considered = positions_considered[index] - 1;
                        let positions_to_change = &positions_to_change_vec[index];
                        let skip_lines = file.no_lines_skipped.get(index).unwrap_or(&0);
                        //Considering x as row number and y as column number.
                        for (x, row) in reader.rows().enumerate() {
                            acc_enc += 1;
                            let mut y = 0;
                            while y < row.len() {
                                let val_considered = &row[position_considered].to_string();
                                //Check if row is not to be skipped.
                                if &x >= skip_lines && positions_to_change.contains(&(y + 1)) {
                                    sheet
                                        .write_string(
                                            x.try_into().unwrap(),
                                            y.try_into().unwrap(),
                                            val_considered,
                                            None,
                                        )
                                        .expect("Could not write masked value to sheet.");
                                } else {
                                    sheet
                                        .write_string(
                                            x.try_into().unwrap(),
                                            y.try_into().unwrap(),
                                            &row[y].to_string(),
                                            None,
                                        )
                                        .expect("Could not write to sheet.");
                                }

                                y += 1;
                            }
                            acc_succ += 1;
                        }
                    }
                }
                workbook.close().expect("Failed to close workbook.");
            }
            _ => {
                log_error!(
                    logger,
                    "Type {} could not be determined for file:{}",
                    file.file_type,
                    file.file_path
                );
            }
        }
        let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
        health_report.gen_health_rpt(&full_output_path);
    }
}
