use calamine::{open_workbook_auto, DataType, Reader};
use chrono::Datelike;
use chrono::NaiveDate;
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
pub struct RepData {
    vsa_codes: String,
    repricing_day: String,
    repricing_freq: String,
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
impl RepData {
    pub fn new_from_xlsx(rep_data: &[DataType]) -> RepData {
        RepData {
            vsa_codes: get_str_from_xlsx(rep_data, 0),
            repricing_day: get_str_from_xlsx(rep_data, 1),
            repricing_freq: get_str_from_xlsx(rep_data, 2),
        }
    }
}
pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();

    let mut skip_rec: i64 = 0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;

    let master_file = match new_buf_rdr(config_param.master_file_path()) {
        Ok(master_file) => master_file,
        Err(_error) => panic!("Error while getting master file path"),
    };

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
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );
    let mut repricing_map: HashMap<String, RepData> = HashMap::new();
    let mut repricing_master_file = open_workbook_auto(config_param.repricing_master_file_path())
        .expect("Unable to open the repricing master xlsx file.");
    if let Some(Ok(repricing_file_reader)) =
        repricing_master_file.worksheet_range(&config_param.repricing_file_sheet_name())
    {
        for (row_no, row) in repricing_file_reader.rows().enumerate().skip(1) {
            let rep_data = RepData::new_from_xlsx(row);
            repricing_map.insert(rep_data.vsa_codes.to_string(), rep_data);
        }
    }

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
        tot_acc_encntrd += 1;
        let fields: Vec<&str> = line.split('|').collect();
        let concat = format!("{}{}{}{}", fields[86], fields[38], fields[41], fields[85]);
        let binding = MasterData::def();
        let vs_code = &master_map.get(&concat).unwrap_or(&binding).vs_param;
        let account_opn_dt = NaiveDate::parse_from_str(&fields[42], "%Y-%m-%d")
            .unwrap_or(*config_param.as_on_date());
        let mut rep_date = rbdate::NaiveDate::parse_from_str(fields[8], "%Y-%m-%d")
            .unwrap_or(*config_param.as_on_date())
            .format("%d-%m-%Y")
            .to_string();
        let mut acc_mclr_int_strt_dt = *config_param.as_on_date();
        if repricing_map.contains_key(vs_code) {
            let rep_data = match repricing_map.get(vs_code) {
                Some(rep_data) => rep_data,
                _ => panic!("vsa codes is not present in repricing master file"),
            };
            let mut repricing_date = account_opn_dt;
            if !rep_data.repricing_day.is_empty() {
                let month = account_opn_dt.month();
                let year = account_opn_dt.year();
                let day = rep_data
                    .repricing_day
                    .to_string()
                    .parse::<u32>()
                    .unwrap_or(1);
                let date =
                    NaiveDate::from_ymd_opt(year, month, day).unwrap_or(*config_param.as_on_date());
                repricing_date = date;
            }
            while repricing_date <= *config_param.as_on_date() {
                let date = get_repricing_freq(&rep_data.repricing_freq, repricing_date);
                repricing_date = date;
            }
            acc_mclr_int_strt_dt = repricing_date;
            rep_date = acc_mclr_int_strt_dt.format("%d-%m-%Y").to_string();
        }
        op_line.push_str(&format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            rbdate::NaiveDate::parse_from_str(fields[0],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            fields[6],
            fields[7],
            rbdate::NaiveDate::parse_from_str(fields[8],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[9],
            fields[10],
            fields[11],
            fields[12],
            fields[13],
            fields[14],
            rbdate::NaiveDate::parse_from_str(fields[15],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[16],
            fields[17],
            fields[18],
            fields[19],
            rbdate::NaiveDate::parse_from_str(fields[20],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[21],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[22],
            fields[23],
            fields[24],
            rbdate::NaiveDate::parse_from_str(fields[25],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[26],
            rbdate::NaiveDate::parse_from_str(fields[27],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[28],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[29],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[30],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[31],
            fields[32],
            fields[33],
            rbdate::NaiveDate::parse_from_str(fields[34],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[35],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[36],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[37],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[38],
            fields[39],
            fields[40],
            fields[41],
            rbdate::NaiveDate::parse_from_str(fields[42],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[43],
            fields[44],
            fields[45],
            fields[46],
            fields[47],
            fields[48],
            fields[49],
            fields[50],
            fields[51],
            fields[52],
            fields[53],
            fields[54],
            fields[55],
            fields[56],
            fields[57],
            fields[58],
            fields[59],
            fields[60],
            fields[61],
            fields[62],
            fields[63],
            fields[64],
            fields[65],
            fields[66],
            rbdate::NaiveDate::parse_from_str(fields[67],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[68],
            fields[69],
            fields[70],
            fields[71],
            rep_date,
            fields[73],
            fields[74],
            rbdate::NaiveDate::parse_from_str(fields[75],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[76],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            rbdate::NaiveDate::parse_from_str(fields[77],"%Y-%m-%d").unwrap_or(*config_param.as_on_date()).format("%d-%m-%Y").to_string(),
            fields[78],
            fields[79],
            fields[80],
            fields[81],
            fields[82],
            fields[83],
            fields[84],
            fields[85],
            fields[86],
            config_param.as_on_date().format("%d-%m-%Y").to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            config_param.as_on_date().format("%d-%m-%Y").to_string(),
            config_param.as_on_date().format("%d-%m-%Y").to_string(),
            concat,
            master_map.get(&concat).unwrap_or(&MasterData::def()).vs_param,
            master_map.get(&concat).unwrap_or(&MasterData::def()).vg_param,
            "",
            ""
        ));
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
    debug!(diag_log, "Writing  Term loans output file. {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec,
        skip_rec,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .trim()
        .to_string()
}

pub fn get_repricing_freq(rep_freq: &str, account_open_date: NaiveDate) -> NaiveDate {
    let next_rep_date = match rep_freq {
        "A" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 12)
            .expect("Cannot derive `next repricing date`."),
        "M" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 1)
            .expect("Cannot derive `next repricing date`."),
        "B" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 2)
            .expect("Cannot derive `next repricing date`."),
        "Q" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 3)
            .expect("Cannot derive `next repricing date`."),
        "H" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 6)
            .expect("Cannot derive `next repricing date`."),
        _ => panic!(
            "cannot derive repricing frequency for the `account open date: {}`",
            account_open_date
        ),
    };
    return next_rep_date;
}

