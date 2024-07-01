use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
use std::collections::HashMap;

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
    let start_read_timer = SystemTime::now();

    let mut skip_rec: i64 = 0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;

    let inp_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(inp_file) => inp_file,
        Err(error) => panic!(
            "Could not found inp file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

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

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let mut op_line=String::new();

    let start_derive_timer = SystemTime::now();
    for (line_num, lines) in inp_file.lines().enumerate().skip(1) {
        let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                ),
            };

        let mut fields: Vec<&str> = line.split('|').collect();

        tot_acc_encntrd += 1;
        let concat = format!("{}{}{}{}", fields[13], fields[1], fields[14], fields[2]);
        op_line.push_str(&format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            rbdate::NaiveDate::parse_from_str(fields[0],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            fields[6],
            fields[7],
            rbdate::NaiveDate::parse_from_str(fields[8],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[9],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[10],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[11],
            fields[12],
            fields[13],
            fields[14],
            config_param.as_on_date().format("%d-%m-%Y").to_string(),
            "0.0".to_string(),
            "0.0".to_string(),
            "0.0".to_string(),
            config_param.as_on_date().format("%d-%m-%Y").to_string(),
            config_param.as_on_date().format("%d-%m-%Y").to_string(),
            concat,
            master_map.get(&concat).unwrap_or(&MasterData::def()).vs_param,
            master_map.get(&concat).unwrap_or(&MasterData::def()).vg_param,
            "NA",
            "NA"
        ));

        tot_amt_ip += &fields[7].to_string().parse().unwrap_or(0.0);
        tot_amt_op += &fields[7].to_string().parse().unwrap_or(0.0);
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();

    let mut op_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_writer.write_all(op_line.as_bytes()) {
        Ok(_) => info!(log,"Successfully written output file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output");
    debug!(
        diag_log,
        "Writing output file, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec,
        skip_rec,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
