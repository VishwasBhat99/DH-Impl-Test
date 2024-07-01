use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook, Reader, Xlsx};
use health_report::HealthReport;
use rbdate::datevalue_to_naive_date;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    let mut input_file: Xlsx<_> =
        open_workbook(&config_params.input_file()).expect("Unable to open `Input File`.");
    let input_sheet_name = input_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();

    let mut rating_master: Xlsx<_> = open_workbook(&config_params.rating_master())
        .expect("Unable to open `Rating Master File`.");
    let rating_master_sheet_name = rating_master
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    let mut rating_master_fields: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = rating_master.worksheet_range(rating_master_sheet_name.as_str()) {
        for rating_fields in reader.rows().skip(1) {
            let mut rating_key: String = String::new();
            rating_key.push_str(&rating_fields[0].to_string().trim());
            rating_key.push('|');
            rating_key.push_str(&rating_fields[1].to_string().trim());
            rating_master_fields.insert(rating_key, rating_fields[2].to_string());
        }
    }
    if let Some(Ok(reader)) = input_file.worksheet_range(input_sheet_name.as_str()) {
        for input_fields in reader.rows().skip(1) {
            let mut input_key: String = String::new();
            input_key.push_str(&input_fields[35].to_string().trim());
            input_key.push('|');
            input_key.push_str(&input_fields[34].to_string().trim());
            tot_acc_encntrd += 1;
            acc_pro_suc += 1;
            write!(
            op_writer,
            "{}|{}|{}|||{}|||{}|{}|{}|{}|{}|{}|{}||||{}|{}|{}||{}||{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||||{}|{}|{}|{}|{}||||||{}|{}|{}|{}|{}|{}\n",
            input_fields[0],
            input_fields[14],
            input_fields[13],
            input_fields[10],
            input_fields[6],
            input_fields[7],
            datevalue_to_naive_date(&input_fields[15].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[16].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[17].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[18],
            input_fields[19],
            input_fields[3],
            input_fields[1],
            input_fields[2],
            input_fields[20],
            input_fields[21],
            input_fields[22],
            input_fields[23],
            input_fields[24],
            input_fields[25],
            datevalue_to_naive_date(&input_fields[26].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[27].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[28].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[29].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[30].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            datevalue_to_naive_date(&input_fields[31].to_string()).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
            input_fields[32],
            input_fields[9],
            input_fields[8],
            input_fields[35],
            input_fields[34],
            input_fields[11],
            input_fields[12],
            input_fields[32],
            input_fields[4],
            input_fields[5],
            if input_fields[17].to_string() !=""{
                "N"
            }else{
                "Y"
            },
            rating_master_fields.get(&input_key).unwrap_or(&" ".to_string()),
            input_fields[33],
            input_fields[2].to_string().parse::<f64>().unwrap_or(0.0)-input_fields[1].to_string().parse::<f64>().unwrap_or(0.0)
        );
        }
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
