use self::derive_fields::get_op_line;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

mod derive_fields;

#[derive(Debug, Clone, Default)]
///Fields used for stamping in output file
pub struct MasterData {
    pub vs_param: String,
    pub vg_param: String,
}

impl MasterData {
    pub fn new(master_data: Vec<&str>) -> MasterData {
        MasterData {
            vs_param: master_data[9].to_string(),
            vg_param: master_data[10].to_string(),
        }
    }
    pub fn def() -> MasterData {
        MasterData {
            vs_param: "NONE".to_string(),
            vg_param: "NONE".to_string(),
        }
    }
}

pub fn process(config_param: ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_time = SystemTime::now();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(_error) => panic!("Error while getting input directory path"),
    };
    let master_file = match new_buf_rdr(config_param.master_file_path()) {
        Ok(master_file) => master_file,
        Err(_error) => panic!("Error while getting master file path"),
    };
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(_error) => panic!("Error while getting output directory path"),
    };

    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let master_reader = BufReader::new(master_file);
    let mut tot_acc_encntrd = DEFAULT_INT;
    let tot_amt = DEFAULT_FLOAT;

    //Reading Master Data File
    let mut master_map: HashMap<String, MasterData> = HashMap::new();
    for line in master_reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from master file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        let master_val = MasterData::new(fields.to_owned());
        let concat = format!(
            "{}{}{}{}",
            fields[2].to_string().trim(),
            fields[1].to_string().trim(),
            fields[4].to_string().trim(),
            fields[5].to_string().trim()
        );
        master_map.insert(concat, master_val);
    }

    for line in reader.lines().skip(1) {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        tot_acc_encntrd += 1;
        let output_line = get_op_line(config_param.as_on_date, fields, &mut master_map);
        match writer.write_all(output_line.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }
    let end_process_time = SystemTime::now();
    let duration = end_process_time
        .duration_since(start_process_time)
        .expect("Could not calculate total process duration.");
    info!(diag_log, "Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();

    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd,
        0,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
