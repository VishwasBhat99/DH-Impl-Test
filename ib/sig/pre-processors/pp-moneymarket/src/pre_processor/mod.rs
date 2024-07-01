extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::InputAccount;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

mod derive_fields;
mod input_account;

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

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut writer = BufWriter::new(output_file);

    let master_file = match new_buf_rdr(config_param.master_file_path()) {
        Ok(master_file) => master_file,
        Err(_error) => panic!("Error while getting master file path"),
    };

    //Reading Master Data File
    let mut master_map: HashMap<String, MasterData> = HashMap::new();
    for line in master_file.lines() {
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

    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in reader.deserialize().enumerate().skip(1) {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_acc_encntrd += 1;
        let temp_string = get_op_line(&input_account, config_param.as_on_date, &mut master_map);
        op_line.push_str(temp_string.as_str());
        op_line.push('\n');
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
