use std::{collections::HashMap, fs};

use calamine::{open_workbook_auto, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod struct_data;
use self::struct_data::Data;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_line: String = String::new();
    let currency = config_params.currency();
    let mut rate_multiplier = 1.0;
    let is_consolidated_flag = config_params.is_consolidated_flag().unwrap_or(true);
    let def_base_ccy = "INR".to_string();
    let base_ccy = match config_params.base_ccy() {
        Some(val) => val,
        _ => &def_base_ccy,
    };
    if is_consolidated_flag == false {
        //Reading exchange rate file
        let exchange_rate_file_path = match config_params.exchange_rate_file_path() {
            Some(val) => val,
            _ => panic!("Can not get the file path for exchange rate"),
        };
        let exchange_rate_file_reader = fs::read_to_string(exchange_rate_file_path)
            .expect("Cannot get the exchange rate file path");
        for (line_no, line) in exchange_rate_file_reader.lines().enumerate() {
            let exchange_rate_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
            let from_ccy = get_str(&exchange_rate_file_path, &exchange_rate_vec, 0, line_no);
            let to_ccy = get_str(&exchange_rate_file_path, &exchange_rate_vec, 1, line_no);
            let rate_str = get_str(&exchange_rate_file_path, &exchange_rate_vec, 2, line_no);
            let rate = rate_str.parse::<f64>().unwrap_or(1.0);
            if from_ccy.to_uppercase() == currency.to_uppercase()
                && to_ccy.to_uppercase() == base_ccy.clone()
            {
                rate_multiplier = rate;
                break;
            }
        }
    }
    let as_on = config_params.as_on_date().format("%d-%m-%Y");
    let mut workbook = open_workbook_auto(config_params.input_file_path())
        .expect("Error while opening the input file");
    let denomination: f64 = config_params
        .denomination()
        .parse()
        .expect("Error while parsing the denomination amount.");

    let input_reader = io::read_file(config_params.excel_config_file());
    let mut out_file = io::create_file(config_params.output_file_path());
    let mut rdr = csv::Reader::from_reader(input_reader);
    for line in rdr.records() {
        let record = line.unwrap();
        let col_no: usize = record[0]
            .parse()
            .expect("Error while getting the col number.");
        let row_no: usize = record[1]
            .parse()
            .expect("Error while getting the row number.");
        let amt: f64;
        if let Some(Ok(reader)) = workbook.worksheet_range(config_params.input_sheet_name()) {
            for rows in reader.rows().skip(row_no - 1) {
                if rows[col_no - 1].is_empty() {
                    amt = 0.0;
                } else {
                    amt = rows[col_no - 1]
                        .to_string()
                        .parse()
                        .expect("Error while parsing the amount.");
                }
                let mut outbal_fcy: f64 = amt * denomination;
                let outbal_con = outbal_fcy;
                if is_consolidated_flag == false {
                    outbal_fcy = outbal_fcy * rate_multiplier;
                }
                let data: Data = Data {
                    as_on_date: as_on.to_string(),
                    subsidiary_id: config_params.subsidiary_id().to_string(),
                    currency: config_params.currency().to_string(),
                    llg_code: record[2].to_string(),
                    outbal_con: outbal_con,
                    outbal_fcy: outbal_fcy,
                };
                op_line.push_str(&get_op_line(&data));
                break;
            }
        }else {
            panic!(" `{}` sheet is not present in input file.", config_params.input_sheet_name())
        }
    }
    io::output_writer(&mut out_file, op_line);
}

pub fn get_op_line(data: &Data) -> String {
    let mut op_line = String::new();
    op_line.push_str(&data.print());
    op_line.push('\n');
    op_line
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
