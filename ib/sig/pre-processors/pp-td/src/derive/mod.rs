use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

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

    let skip_rec: i64 = 0;
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

    let mut op_line = String::new();

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
        let concat = format!("{}{}{}{}", fields[54], fields[38], fields[41], fields[39]);
        op_line.push_str(&format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            rbdate::NaiveDate::parse_from_str(fields[0],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            fields[6],
            fields[7],
            fields[8],
            fields[9],
            fields[10],
            fields[11],
            fields[12],
            fields[13],
            fields[14],
            fields[15],
            fields[16],
            fields[17],
            fields[18],
            fields[19],
            fields[20],
            fields[21],
            rbdate::NaiveDate::parse_from_str(fields[22],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[23],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[24],
            fields[25],
            fields[26],
            fields[27],
            fields[28],
            fields[29],
            fields[30],
            fields[31],
            fields[32],
            fields[33],
            fields[34],
            fields[35],
            fields[36],
            fields[37],
            fields[38],
            fields[39],
            fields[40],
            fields[41],
            fields[42],
            fields[43],
            fields[44],
            fields[45],
            fields[46],
            rbdate::NaiveDate::parse_from_str(fields[47],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[48],
            fields[49],
            fields[50],
            rbdate::date_from_timestamp(fields[51].to_string().parse::<i64>().unwrap_or(0)).format("%d-%m-%Y").to_string(),
            rbdate::date_from_timestamp(fields[52].to_string().parse::<i64>().unwrap_or(0)).format("%d-%m-%Y").to_string(),
            fields[53],
            fields[54],
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

        let amt = &fields[42].to_string().parse::<f64>();

        if !amt.is_err() {
            tot_amt_ip += &fields[42].to_string().parse().unwrap_or(0.0);
            tot_amt_op += &fields[42].to_string().parse().unwrap_or(0.0);
        } else {
            error!(log, "Amt could not be parsed in f64");
        }
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
            "Unable to create Output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_writer.write_all(op_line.as_bytes()) {
        Ok(_) => info!(log, "Successfully written outputfile."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output.");
    debug!(diag_log, "Writing TD output file. {:?}.", duration);

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
