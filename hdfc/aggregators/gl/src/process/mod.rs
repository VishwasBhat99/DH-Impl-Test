use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook, Reader, Xlsx};
use health_report::HealthReport;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
mod exchange_rate;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;

    let mut master_file: Xlsx<_> =
        open_workbook(&config_params.master_file()).expect("Unable To Open `Master File`.");
    let sheet_name = master_file
        .sheet_names()
        .first()
        .expect("Master File Is Empty")
        .to_owned();
    let mut master_fields: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(Ok(reader)) = master_file.worksheet_range(sheet_name.as_str()) {
        for record in reader.rows().skip(1) {
            let mut rec_vec: Vec<String> = Vec::new();
            rec_vec.push(record[0].to_string());
            rec_vec.push(record[2].to_string());
            rec_vec.push(record[3].to_string());
            rec_vec.push(record[4].to_string());
            master_fields.insert(record[1].to_string(), rec_vec);
        }
    }

    let input_file = File::open(&config_params.input_file()).expect("Could Not Read File");
    let input_reader = BufReader::new(input_file);
    let exchange_rate_file = exchange_rate::read_exchange_rate(config_params.exchange_rate_file());
    let mut final_records: HashMap<String, Vec<f64>> = HashMap::new();
    let merge_file = File::open(&config_params.merge_file()).expect("Could Not Read File");
    let merge_reader = BufReader::new(merge_file);
    for (index, record) in merge_reader.lines().enumerate() {
        let record = record.expect("Could Not Read Line").to_string();
        let merge_fields: Vec<&str> = record.split('|').collect();
        let mut op_key = String::new();
        op_key.push_str(merge_fields[0]);
        op_key.push('|');
        op_key.push_str(merge_fields[1]);
        op_key.push('|');
        op_key.push_str(merge_fields[2]);
        op_key.push('|');
        op_key.push_str(merge_fields[3]);
        op_key.push('|');
        op_key.push_str(merge_fields[4]);
        op_key.push('|');
        op_key.push_str(merge_fields[5]);
        let mut op_val: Vec<f64> = Vec::new();
        op_val.push(merge_fields[6].parse::<f64>().unwrap_or(0.0));
        op_val.push(merge_fields[7].parse::<f64>().unwrap_or(0.0));
        op_val.push(merge_fields[8].parse::<f64>().unwrap_or(0.0));
        final_records.insert(op_key, op_val);
    }
    for (index, record) in input_reader.lines().enumerate() {
        let record = record.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = record.split('|').collect();
        if (input_fields[3] == "IRS") {
            continue;
        }
        tot_acc_encntrd += 1;
        let exchange_rate = exchange_rate::get_exch_rate(
            input_fields[2].to_string(),
            config_params.base_currency(),
            &exchange_rate_file,
        );
        if master_fields.contains_key(&input_fields[0].to_string()) {
            acc_pro_suc += 1;
            let mut op_key = String::new();
            if input_fields[2] == "INR" || input_fields[2] == "FCY" {
                continue;
            }
            op_key.push_str(
                &master_fields
                    .get(&input_fields[0].to_string())
                    .expect("Could Not Find Record")[0],
            );
            op_key.push('|');
            op_key.push_str(&config_params.as_on_date().format("%d-%m-%Y").to_string());
            op_key.push('|');
            op_key.push_str(
                &master_fields
                    .get(&input_fields[0].to_string())
                    .expect("Could Not Find Record")[1],
            );
            op_key.push('|');
            op_key.push_str(
                &master_fields
                    .get(&input_fields[0].to_string())
                    .expect("Could Not Find Record")[2],
            );
            op_key.push('|');
            op_key.push_str(
                &master_fields
                    .get(&input_fields[0].to_string())
                    .expect("Could Not Find Record")[3],
            );
            op_key.push('|');
            if input_fields[2] == "RUP" {
                op_key.push_str("INR");
            } else {
                op_key.push_str(input_fields[2]);
            }

            if final_records.contains_key(&op_key.to_owned()) {
                let op_val = &mut final_records[&op_key.to_owned()].to_owned();
                op_val[0] += input_fields[6].parse::<f64>().unwrap_or(0.0);
                op_val[1] += input_fields[6].parse::<f64>().unwrap_or(0.0) * exchange_rate;
                final_records.insert(op_key.to_owned(), op_val.to_vec());
            } else {
                let mut op_val: Vec<f64> = Vec::new();
                op_val.push(input_fields[6].parse::<f64>().unwrap_or(0.0));
                op_val.push(input_fields[6].parse::<f64>().unwrap_or(0.0) * exchange_rate);
                op_val.push(0.0);
                final_records.insert(op_key.to_owned(), op_val);
            }
        }
    }
    for key_val in final_records.keys() {
        write!(
            op_writer,
            "{}|{}|{}|{}\n",
            key_val,
            final_records[key_val][0],
            final_records[key_val][1],
            final_records[key_val][2],
        );
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
