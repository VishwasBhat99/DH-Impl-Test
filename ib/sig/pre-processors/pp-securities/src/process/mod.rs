use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
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
    let alm_opics_securities_reader =
        fs::read_to_string(&config_params.alm_opics_securities_file())
            .expect("Could Not Read ALM Opics Securities File");
    let mut op_writer = get_writer(&config_params.securities_pp_file_path());

    let mut tot_acc_encntrd = 0;
    let mut skp_acc = 0;
    let mut tot_amt = 0.0;
    let date_parser = DateParser::new("%Y-%m-%d".to_string(), false);

    let master_file = match new_buf_rdr(config_params.master_file_path()) {
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

    for line in alm_opics_securities_reader.lines().skip(1) {
        let derived_fields = line.split("|").collect::<Vec<&str>>();
        tot_acc_encntrd += 1;
        let concat = format!(
            "{}{}{}{}",
            derived_fields[37], derived_fields[3], derived_fields[35], derived_fields[34]
        );
        write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            date_parser.parse_opt(&derived_fields[0]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            derived_fields[1],
            derived_fields[2],
            derived_fields[3],
            derived_fields[4],
            date_parser.parse_opt(&derived_fields[5]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&derived_fields[6]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&derived_fields[7]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
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
            derived_fields[18],
            derived_fields[19],
            derived_fields[20],
            derived_fields[21],
            derived_fields[22],
            derived_fields[23],
            derived_fields[24],
            derived_fields[25],
            derived_fields[26],
            derived_fields[27],
            derived_fields[28],
            derived_fields[29],
            date_parser.parse_opt(&derived_fields[30]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            date_parser.parse_opt(&derived_fields[31]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            derived_fields[32],
            derived_fields[33],
            derived_fields[34],
            derived_fields[35],
            derived_fields[36],
            derived_fields[37],
            config_params.as_on_date().format("%d-%m-%Y"),
            //Curr-Bal-LCY
            if derived_fields[21].to_string().trim().to_uppercase() == "HTM"{
                derived_fields[19].to_owned()
            }else{
                (derived_fields[19].parse::<f64>().unwrap_or(0.0) -
                derived_fields[17].parse::<f64>().unwrap_or(0.0) -
                derived_fields[18].parse::<f64>().unwrap_or(0.0)).to_string()
            },
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
    health_report.gen_health_rpt(&config_params.securities_pp_file_path());
}
