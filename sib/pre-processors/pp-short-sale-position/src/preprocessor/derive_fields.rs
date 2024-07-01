extern crate chrono;

use rbdate::{num_days_start_to_end, NaiveDate};

use std::io::{self, *};
use std::{collections::HashMap, fs::File, io::BufRead};

use crate::configuration_parameter::ConfigurationParameters;

use health_report::HealthReport;

use super::hashvalue_struct::OutputDatas;
use super::output_account::{get_writer, OutputData};

pub fn derive_output(
    input_file_reader: io::BufReader<File>,
    master_map: &HashMap<(i64, i64), OutputDatas>,
    config_params: &ConfigurationParameters,
) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    for line in input_file_reader.lines() {
        let line = line
            .expect("Could Not Read Line from input file")
            .to_string();
        let input_fields: Vec<&str> = line.split('|').collect();

        acc_enc += 1;

        let maturity_date = NaiveDate::parse_from_str(input_fields[7].trim(), "%d-%m-%Y")
            .unwrap_or(*config_params.as_on_date());
        let position_date = NaiveDate::parse_from_str(input_fields[1], "%d/%m/%Y")
            .unwrap_or(*config_params.as_on_date());

        let residual_tenor = num_days_start_to_end(position_date, maturity_date);
        let mut time_band = String::new();
        let mut period = String::new();
        let mut zone = String::new();
        for key in master_map.keys() {
            if residual_tenor >= key.0 && residual_tenor < key.1 {
                let master_value = master_map.get(key).expect("NO such value in master file");
                time_band = master_value.time_band.clone();
                period = master_value.period.clone();
                zone = master_value.zone.clone();
            }
        }
        let outstanding = input_fields[9].parse::<f64>().unwrap_or(0.0);
        let face_value = input_fields[10].parse::<f64>().unwrap_or(0.0);
        let book_value_lcy = input_fields[11].parse::<f64>().unwrap_or(0.0);
        let weighte_avg_price = input_fields[12].parse::<f64>().unwrap_or(0.0);
        let market_price = input_fields[13].parse::<f64>().unwrap_or(0.0);
        let market_value_lcy = input_fields[14].parse::<f64>().unwrap_or(0.0);
        let appr_depr = input_fields[15].parse::<f64>().unwrap_or(0.0);
        let frequency_rate = input_fields[17].parse::<f64>().unwrap_or(0.0);

        let op = OutputData {
            position_date: position_date.format("%d-%m-%Y").to_string(),
            scrip_code: input_fields[2].trim().to_string(),
            scrip_name: input_fields[3].trim().to_string(),
            portfolio: input_fields[4].trim().to_string(),
            instrument_name: input_fields[5].trim().to_string(),
            maturity_date: maturity_date.format("%d-%m-%Y").to_string(),
            currency: input_fields[8].trim().to_string(),
            isin: input_fields[6].trim().to_string(),
            outstanding_q_ty: outstanding,
            face_value_lcy: face_value,
            book_value_lcy: book_value_lcy,
            weighte_avg_price: weighte_avg_price,
            market_price: market_price,
            market_value_lcy: market_value_lcy,
            appr_depr: appr_depr,
            coupon: input_fields[16].trim().to_string(),
            frequency: frequency_rate,
            res_tenor: residual_tenor.to_string(),
            yields: "".to_string(),
            mduration: "".to_string(),
            timeband: time_band,
            period: period,
            zone: zone,
            position: "Short".to_string(),
        };
        let result = writeln!(op_writer, "{}", op.format_with_separator());
        match result {
            Ok(_) => acc_proc += 1,
            Err(err) => {
                panic!("Error in writing the output {:?}", err);
            }
        }
    }
    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
