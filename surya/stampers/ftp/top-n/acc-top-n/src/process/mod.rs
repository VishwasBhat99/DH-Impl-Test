use crate::configuration_parameters::ConfigurationParameters;
extern crate chrono;
use crate::macros;
use health_report::HealthReport;
use rbdate::DateParser;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::env::current_dir;
use std::fs::File;
use std::hash::Hash;
use std::io::BufWriter;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;
mod config;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut tot_acc_encntrd=0;
    let mut tot_acct_top_n=0;
    let start_time = SystemTime::now();
    let mut input_file_vec: Vec<String> = Vec::new();
    let as_on_month = *config_params.as_on_date();
    let files_config = config::get_files(config_params.config_file_path());
    for config_fields in files_config.files {
        input_file_vec = config_fields.input_file_path;
    }
    let cust_def_file = match new_buf_rdr(config_params.top_n_input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.top_n_input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut cust_def_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in cust_def_file.lines().enumerate().skip(1) {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.top_n_input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();
        let cust_id = input_fields[1].to_string();
        let cust_name = input_fields[2].to_string();
        cust_def_map.insert(cust_id, cust_name);
    }
    let mut op_line: String = String::new();
    for input_file_path in input_file_vec {
        let start_time = SystemTime::now();
        let input_file = match new_buf_rdr(&input_file_path) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found input file: `{}` on location `{}` : {}.",
                config_params.config_file_path(),
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        let mut field_flag=true;
        for (line_num, lines) in input_file.lines().enumerate().skip(1) {
            tot_acc_encntrd +=1;
            let input_line = match lines {
                Ok(input_line) => input_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    input_file_path,
                    line_num + 1,
                    error
                ),
            };
            let input_fields = input_line.split('|').collect::<Vec<&str>>();
            if input_fields.len() < config_params.tot_field_number().try_into().unwrap_or(0){
                info!(
                    logger,
                    "The input file `{}` does not have required field",
                    input_file_path,
                );
                field_flag=false;
                break;
            }
            let cust_id = input_fields[42].to_string();

            if cust_def_map.contains_key(&cust_id) {
                tot_acct_top_n +=1;
                op_line.push_str(&format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                 as_on_month.format("%d-%m-%Y"),
                 input_fields[42].to_string(),
                 input_fields[1].to_string(),
                 input_fields[2].to_string(),
                 input_fields[3].to_string(),
                 input_fields[4].to_string(),
                 input_fields[5].to_string(),
                 input_fields[6].to_string(),
                 input_fields[7].to_string(),
                 input_fields[8].to_string(),
                 input_fields[9].to_string(),
                 input_fields[10].to_string(),
                 input_fields[11].to_string(),
                 input_fields[12].to_string(),
                 input_fields[13].to_string(),
                 input_fields[14].to_string(),
                 input_fields[15].to_string(),
                 input_fields[16].to_string(),
                 input_fields[17].to_string(),
                 input_fields[18].to_string(),
                 input_fields[19].to_string(),
                 input_fields[20].to_string(),
                 input_fields[21].to_string(),
                 input_fields[22].to_string(),
                 input_fields[23].to_string(),
                 input_fields[24].to_string(),
                 input_fields[25].to_string(),
                 input_fields[26].to_string(),
                 input_fields[27].to_string(),
                 input_fields[28].to_string(),
                 input_fields[29].to_string(),
                 input_fields[30].to_string(),
                 input_fields[31].to_string(),
                 input_fields[32].to_string(),
                 input_fields[33].to_string(),
                 input_fields[34].to_string(),
                 input_fields[35].to_string(),
                 input_fields[36].to_string(),
                 input_fields[37].to_string(),
                 input_fields[38].to_string(),
                 input_fields[39].to_string(),
                 input_fields[40].to_string(),
                 input_fields[41].to_string(),
                 input_fields[43].to_string(),
                 input_fields[44].to_string(),
                 input_fields[45].to_string(),
                 input_fields[46].to_string(),
                 input_fields[47].to_string(),
                 input_fields[48].to_string(),
                 input_fields[49].to_string(),
                 input_fields[50].to_string(),
                 input_fields[51].to_string(),
                 input_fields[52].to_string(),
                 input_fields[53].to_string(),
                 input_fields[54].to_string(),
                 input_fields[55].to_string(),
                 input_fields[56].to_string(),
                 input_fields[57].to_string(),
                 input_fields[58].to_string(),
                 input_fields[59].to_string(),

            ));
            }
        }
        if field_flag {
            let end_timer = SystemTime::now();
            let process_duration = end_timer
                .duration_since(start_time)
                .expect("Could not calculate total duration.");
            info!(
                logger,
                "Time for processing the input file {:?}, will be {:?}",
                input_file_path,
                process_duration
            );
        }
    }

    let mut op_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match op_writer.write_all(op_line.as_bytes()) {
        Ok(_) => info!(logger, "Successfully written outputfile."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_params.output_file_path(),
            error
        ),
    }
    let end_timer = SystemTime::now();
        let process_duration = end_timer
            .duration_since(start_time)
            .expect("Could not calculate total duration.");
        info!(
            logger,
            "Time for processing total program will be {:?}",
            process_duration
        );
        let health_report = HealthReport::new(
            tot_acc_encntrd,
            tot_acct_top_n,
            tot_acc_encntrd-tot_acct_top_n,
            0.0,
            0.0,
            0,
        );
        println!("{}", health_report.display());
        health_report.gen_health_rpt(&config_params.output_file_path());
}
