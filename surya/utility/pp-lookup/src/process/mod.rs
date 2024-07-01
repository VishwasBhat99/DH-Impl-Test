use self::io::get_writer;
use self::read_metadata::*;
use crate::configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;
use rbdate::DateParser;
mod config;
mod io;
mod read_metadata;

pub fn get_lookup_output(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let files_config = config::get_files(config_params.config_file());
    let date_parser = DateParser::new(config_params.input_date_format().to_string(), false);
    for file in files_config.files {
        let start_time = SystemTime::now();
        let mut acc_encountered_1 = 0;
        let mut acc_skipped_1 = 0;
        let mut acc_encountered_2 = 0;
        let mut acc_skipped_2 = 0;
        let mut input_map: HashMap<String, String> = HashMap::new();
        let mut header = String::new();
        let mut op_writer = get_writer(&file.output_file_path);
        let del = file.delimiter;

        if file.is_header_req {
            for field in file.req_fields_1.iter() {
                header.push_str(field);
                header.push_str(&del);
            }
            for field in file.req_fields_2.iter() {
                header.push_str(&field);
                header.push_str(&del);
            }
            header.pop();
            header.push_str("\n");
            op_writer
                .write_all(header.as_bytes())
                .expect("Unable to write the Header to output");
        }

        let input_file =
            File::open(&file.input_file_path_2).expect("Could Not Read The Input File 2");
        let reader = BufReader::new(input_file);
        let mut ip_map: HashMap<String, MetaDataFields> = HashMap::new();
        let mut column_count = 0;
        let mut date_fields_vec: Vec<String> = Vec::new();
        let mut date_pos_vec: Vec<u8> = Vec::new();
        // read the lookup file and store the data (input file 2 acts as lookup file)
        for (index, line) in reader.lines().enumerate() {
            acc_encountered_2 += 1;
            let acc_data = match line {
                Ok(acc_data) => acc_data,
                Err(error) => {
                    log_info!(log, "Error in Input file 1 line {} : {}", index + 1, error);
                    acc_skipped_2 += 1;
                    continue;
                }
            };
            let inp_fields: Vec<&str> = acc_data.split(&del).collect();
            let mut op = String::new();

            read_input_metadata(
                file.metadata_file_path_2.clone(),
                log,
                diag_log,
                &mut ip_map,
                &mut column_count,
                &mut date_fields_vec,
            );
            for field in date_fields_vec.iter() {
                date_pos_vec.push(ip_map.get(&field.clone()).unwrap().position);
            }
            let lookup_pos = &ip_map.get(&file.inp2_lookup_key).unwrap().position;

            for field in file.req_fields_2.iter() {
                let pos = match ip_map.get(field) {
                    Some(val) => val.position as u8,
                    _ => 0 as u8,
                };
                if pos > 0 {
                    if date_pos_vec.contains(&pos) {
                        let date = date_parser.parse(inp_fields[(pos - 1) as usize]);
                        op.push_str(&date.format("%d-%m-%Y").to_string());
                    } else {
                        op.push_str(inp_fields[(pos - 1) as usize]);
                    }
                }
                op.push_str(&del);
            }
            let lookup = &inp_fields[(lookup_pos - 1) as usize].to_string();
            input_map.insert(lookup.to_string(), op);
        }
        let mut ip_map: HashMap<String, MetaDataFields> = HashMap::new();
        let mut column_count = 0;
        let mut date_fields_vec: Vec<String> = Vec::new();
        let mut date_pos_vec: Vec<u8> = Vec::new();

        // read the input file 1 and append to output based on lookup key field
        let input_file =
            File::open(&file.input_file_path_1).expect("Could Not Read The Input File 1");
        let reader = BufReader::new(input_file);

        read_input_metadata(
            file.metadata_file_path_1,
            log,
            diag_log,
            &mut ip_map,
            &mut column_count,
            &mut date_fields_vec,
        );
        for field in date_fields_vec {
            date_pos_vec.push(ip_map.get(&field).unwrap().position);
        }
        let lookup_pos = &ip_map.get(&file.inp1_lookup_key).unwrap().position;
        for (index, line) in reader.lines().enumerate() {
            acc_encountered_1 += 1;
            let acc_data = match line {
                Ok(acc_data) => acc_data,
                Err(error) => {
                    log_info!(log, "Error in Input file 1 line {} : {}", index + 1, error);
                    acc_skipped_1 += 1;
                    continue;
                }
            };
            let inp_fields: Vec<&str> = acc_data.split(&del).collect();
            let mut op = String::new();
            let mut append_req = false;
            let lookup = inp_fields[(lookup_pos - 1) as usize].to_string();
            if input_map.get(&lookup).is_some() {
                append_req = true;
            }
            for field in file.req_fields_1.iter() {
                let pos = match ip_map.get(field) {
                    Some(val) => val.position as u8,
                    _ => 0 as u8,
                };
                if pos > 0 {
                    if date_pos_vec.contains(&pos) {
                        let date = date_parser.parse(inp_fields[(pos - 1) as usize]);
                        op.push_str(&date.format("%d-%m-%Y").to_string());
                    } else {
                        op.push_str(inp_fields[(pos - 1) as usize]);
                    }
                }
                op.push_str(&del);
            }
            let mut final_output = if append_req {
                op + input_map.get(&lookup).unwrap()
            } else {
                op
            };
            final_output.pop();
            final_output.push_str("\n");
            op_writer
                .write_all(final_output.as_bytes())
                .expect("Unable to write to Output file.");
        }
        op_writer
            .flush()
            .expect("Unable to write complete output to file.");
        let end_time = SystemTime::now();
        let total_duration = end_time
            .duration_since(start_time)
            .expect("Could not calculate total duration.");
        println!("Time for Processing: {:?}", total_duration);
        println!(
            "Total accounts encountered in Input file 1: {}\nTotal accounts encountered in Input file 2: {}\nTotal accounts skipped in Input file 1: {}\nTotal accounts skipped in Input file 2: {}",
            acc_encountered_1, acc_encountered_2, acc_skipped_1, acc_skipped_2
        );

        let health_report = HealthReport::new(
            acc_encountered_1 as i64,
            acc_encountered_1 - acc_skipped_1 as i64,
            acc_skipped_1,
            0.0,
            0.0,
            0,
        );
        health_report.gen_health_rpt(&file.output_file_path);
    }
}
