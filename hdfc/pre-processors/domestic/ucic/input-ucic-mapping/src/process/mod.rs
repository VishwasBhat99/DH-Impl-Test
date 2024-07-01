use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::config::Files;
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::time::SystemTime;

mod config;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();

    //Reading UCIC Master Data File
    let mut ucic_master_map: HashMap<String, String> = HashMap::new();
    let mapping_master = match File::open(config_params.ucic_master_file()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open UCIC master file {}",
                config_params.ucic_master_file()
            );
        }
    };
    let master_reader = BufReader::new(mapping_master);
    let master_row = fs::read_to_string(config_params.ucic_master_file()).expect("Unable to read Ucic Master File");
    let last_master_row_num = master_row.lines().count()-1;
    for (line_no, line) in master_reader.lines().enumerate() {
        let ucic_info: String = match line {
            Ok(ucic_info) => ucic_info,
            Err(error) => {
                log_error!(
                    _log,
                    "Cannot read line {} from Ucic Master file: {:?}",
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        if line_no == last_master_row_num {
            break;
        }
        let ucic_fields: Vec<&str> = ucic_info
            .split(&config_params.ucic_field_delimiter)
            .collect();
        let cust_id = ucic_fields[0].to_string();
        let ucic = ucic_fields[6].to_string();
        ucic_master_map.insert(cust_id, ucic);
    }

    //Read Config File
    let mut config_file = match File::open(config_params.io_config_file_path()) {
        Ok(io_config_file) => io_config_file,
        Err(_err) => {
            panic!(
                "Could not open Input Output Config file {}",
                config_params.io_config_file_path()
            );
        }
    };
    let mut config_string = String::new();
    config_file
        .read_to_string(&mut config_string)
        .expect("Failed to read Config File");

    // Parse JSON into vector of FileData
    let file_data: Files =
        serde_json::from_str(&config_string).expect("Failed to parse Config JSON");

    // Access individual file data
    for file in &file_data.files {
        let mut tot_acc_encntrd = 0;
        let mut skip_rec_count = 0;
        let mut op_writer = get_writer(&file.output_file_path);

        let input_file = match File::open(&file.input_file_path) {
            Ok(inp_file) => inp_file,
            Err(_err) => {
                panic!("Could not read input file {}", file.input_file_path);
            }
        };
        let input_reader = BufReader::new(input_file);
        let ip_row = fs::read_to_string(&file.input_file_path).expect("Failed to read input file");
        let last_row_num = ip_row.lines().count()-1;
        //Read individual file data
        for (line_no, line) in input_reader.lines().enumerate() {
            tot_acc_encntrd += 1;
            let acc_info: String = match line {
                Ok(acc_info) => acc_info,
                Err(error) => {
                    skip_rec_count += 1;
                    log_error!(
                        _log,
                        "Cannot read line {} from Input file: {:?}",
                        line_no + 1,
                        error
                    );
                    continue;
                }
            };
            let out_error = format!("Could not write output in file {}", file.output_file_path);
            if line_no == last_row_num {
               writeln!(op_writer, "{}", acc_info).expect(&out_error);
               break;
            };
            let mut input_fields: Vec<&str> = acc_info.split(&file.input_field_delimiter).collect();
            let cust_id = input_fields[file.cust_id_position - 1].to_string();
            let ucic = ucic_master_map
                .get(&cust_id)
                .unwrap_or(&cust_id)
                .to_string();
            if !ucic.is_empty() && ucic.to_uppercase() != "NULL" && ucic != "0".to_string() && ucic.to_uppercase() != "BLANK"{
                input_fields[file.cust_id_position - 1] = ucic.as_str();
            }
            let op_str = input_fields.join(&file.input_field_delimiter);
            writeln!(op_writer, "{}", op_str).expect(&out_error);
        }
        // generate health check for each file in config file
        let health_report = HealthReport::new(
            tot_acc_encntrd,
            tot_acc_encntrd - skip_rec_count,
            skip_rec_count,
            0.0,
            0.0,
            0,
        );
        health_report.gen_health_rpt(&file.output_file_path);
    }
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    }
}
