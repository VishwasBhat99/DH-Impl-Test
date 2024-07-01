use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use chrono::format;
use health_report::HealthReport;
use rbdate::DateParser;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

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

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let alm_borrowing_reader = fs::read_to_string(&config_params.alm_borrowing_file())
        .expect("Could Not Read ALM Borrowing File");
    let mut op_writer = get_writer(&config_params.borrowing_pp_file_path());

    let master_file = match new_buf_rdr(config_params.master_file_path()) {
        Ok(master_file) => master_file,
        Err(_error) => panic!("Error while getting master file path"),
    };

    let mut tot_acc_encntrd = 0;
    let mut skp_acc = 0;
    let mut tot_amt = 0.0;
    let date_parser = DateParser::new("%Y%m%d".to_string(), false);

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

    for line in alm_borrowing_reader.lines().skip(1) {
        let derived_fields = line.split("|").collect::<Vec<&str>>();
        tot_acc_encntrd += 1;
        let concat = format!(
            "{}{}{}{}",
            derived_fields[23], derived_fields[10], derived_fields[13], derived_fields[11]
        );
        write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            date_parser.parse_opt(&derived_fields[0]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            derived_fields[1],
            derived_fields[2],
            derived_fields[3],
            derived_fields[4],
            derived_fields[5],
            date_parser.parse_opt(&derived_fields[6]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            derived_fields[7],
            derived_fields[8],
            derived_fields[9],
            derived_fields[10],
            derived_fields[11],
            derived_fields[12],
            derived_fields[13],
            derived_fields[14],
            derived_fields[15],
            derived_fields[16],
            derived_fields[17],
            date_parser.parse_opt(&derived_fields[18]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&derived_fields[19]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            derived_fields[20],
            derived_fields[21],
            derived_fields[22],
            derived_fields[23],
            config_params.as_on_date().format("%d-%m-%Y"),
            format!("{:.1}", 0.0),
            format!("{:.1}", 0.0),
            format!("{:.1}", 0.0),
            config_params.as_on_date().format("%d-%m-%Y"),
            config_params.as_on_date().format("%d-%m-%Y"),
            concat,
            master_map.get(&concat).unwrap_or(&MasterData::def()).vs_param,
            master_map.get(&concat).unwrap_or(&MasterData::def()).vg_param,
            "",
            ""
        );
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.borrowing_pp_file_path());
}
