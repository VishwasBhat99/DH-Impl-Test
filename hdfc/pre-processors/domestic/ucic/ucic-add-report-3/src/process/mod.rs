use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_structs::*;
use health_report::HealthReport;
use slog::Logger;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::time::SystemTime;
use sdb_io::new_buf_rdr;

mod config;
mod input_structs;


pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut skip_rec_count = 0;

    //Read master file and store:
    let text_desc_master_file = match new_buf_rdr(config_params.master_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}`,{}",
            config_params.master_file(),
            error
        ),
    };
    let mut text_desc_vec: Vec<DescMaster> = Vec::new();
    for (line_num, lines) in text_desc_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.master_file(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_params.master_file_delimiter()).collect();
        let text_desc_master_line = DescMaster {
            flag_value: fields[0].to_string(),
            condition: fields[1].to_string(),
            txt_desc_flag: fields[2].to_string(),
        };
        text_desc_vec.push(text_desc_master_line);
    }
   
    //Read config.json and get files:
    let files_config = config::get_files(config_params.config_file());
    for file in files_config.files {
        let mut acc_enc = 0;
        let mut acc_skp = 0;
        let input_file_path = file.input_file_path;
        let output_file_path = file.output_file_path;
        let file_delimiter = file.delimiter;

        let output_file =
        File::create(output_file_path.clone()).expect("Unable to Create Output File Path");
        let mut output_writer = BufWriter::new(output_file);
        let out_error = format!(
            "Could not write output in sme output file {}",
            &output_file_path
        );

        //input_file:
        let input_file = match new_buf_rdr(&input_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}`,{}",
                input_file_path,
                error
            ),
        };
        for (line_num, lines) in input_file.lines().enumerate() {
            tot_acc_encntrd += 1;
            acc_enc +=1;
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    input_file_path,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split(&file_delimiter).collect();
            let nam_cust_full = fields[60].to_string();
            let test_desc_flag = get_text_desc_flag(nam_cust_full, &text_desc_vec);

            let op_str = format!{
                "{}|{}",
                    line,
                    test_desc_flag,            
            };
            writeln!(output_writer, "{}", op_str).expect(&out_error);
        }

        // generate health check
        let health_report = HealthReport::new(
            acc_enc,
            acc_enc - acc_skp,
            acc_skp,
            0.0,
            0.0,
            0,
        );
        health_report.gen_health_rpt(&output_file_path);
        let end_process_timer = SystemTime::now();
        let duration = end_process_timer
            .duration_since(start_process_timer)
            .expect("Could not calculate total duration for the process.");
        debug!(
            diag_log,
            "Total Duration for Reading and Writing UCIC Report 3: {:?}.", duration
        );

        // generate health check
        let health_report = HealthReport::new(
            acc_enc,
            acc_enc - acc_skp,
            acc_skp,
            0.0,
            0.0,
            0,
        );
        health_report.gen_health_rpt(&output_file_path);

    }

    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing UCIC Report 3: {:?}.", duration
    );

}

fn get_text_desc_flag(nam_cust_full: String, text_desc_data_vec: &Vec<DescMaster>) -> String {
    let mut text_desc_flag = "OTHERS".to_string();
    for text_desc_data in text_desc_data_vec {
        text_desc_flag = match text_desc_data.condition.to_uppercase().as_str() {
            "START" => {
                if  nam_cust_full.to_uppercase().starts_with(&text_desc_data.flag_value.to_uppercase()) 
                {
                    text_desc_data.txt_desc_flag.clone()
                } else {
                    "OTHERS".to_string()
                }
            }
            "END" => {
                if nam_cust_full.to_uppercase().ends_with(&text_desc_data.flag_value.to_uppercase()) 
                {
                    text_desc_data.txt_desc_flag.clone()
                } else {
                    "OTHERS".to_string()
                }
            }
            "BETWEEN" => {
                if nam_cust_full.to_uppercase().contains(&text_desc_data.flag_value.to_uppercase()) {
                    text_desc_data.txt_desc_flag.clone()
                } else {
                    "OTHERS".to_string()
                }
            }
            "MATCHCASE" => {
                if nam_cust_full.to_uppercase().contains(&text_desc_data.flag_value.to_uppercase()) && nam_cust_full.len() == text_desc_data.flag_value.len() {
                    text_desc_data.txt_desc_flag.clone()
                } else {
                    "OTHERS".to_string()
                }
            }
            _ => "OTHERS".to_string(),
        };
        if text_desc_flag != "OTHERS" {
            break;
        }
    }
    text_desc_flag
}
