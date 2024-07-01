pub mod file_reader;
use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::string::ToString;
mod io;
use health_report::HealthReport;
use std::io::Write;

fn curr_map(data_info: &file_reader::InputData) -> HashMap<String, String> {
    let mut currency_map = HashMap::new();
    for line in data_info.curr_data.lines() {
        let curr_data: Vec<&str> = line.split("|").collect();
        currency_map.insert(curr_data[1].to_string(), curr_data[0].to_string());
    }
    currency_map
}

pub fn find_llg_id(currency_map: HashMap<String, String>, curr_detail: &str) -> String {
    match currency_map.get(curr_detail) {
        Some(result) => return result.clone(),
        None => return "none".to_string(),
    }
}

pub fn file_write(data_info: file_reader::InputData, config: &ConfigurationParameters) {
    let mut file_writer = get_writer(&config.output_file_path());
    let currency_map = curr_map(&data_info);
    let mut tot_acc_encntrd = 1;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;
    for line in data_info.input_data.lines().skip(1) {
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
        let input_data: Vec<&str> = line.split("~#~").collect();
        let info_curr = &input_data[6].to_string();
        let mut llg_id: i64 = 0;
        let mut amount: f64 = 0.0;
        llg_id = find_llg_id(currency_map.clone(), &info_curr)
            .parse::<i64>()
            .unwrap_or(0);
        if input_data[6].to_string().eq(config.base_currency()) {
            amount = input_data[5].parse::<f64>().unwrap_or(0.0);
        } else {
            amount = input_data[3].parse::<f64>().unwrap_or(0.0);
        }
        let as_on_dt = NaiveDate::parse_from_str(input_data[0], "%Y-%m-%d %H:%M:%S %.6f")
            .expect("Invalid date hence not able to convert as on date");
        let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

        let curr_id = input_data[6].to_string();
        tot_amt += amount;
        let str_write = format!(
            "{}|{}|{}|ALL|ALL|ALL|ALL|ALL|{}|0 \n",
            &llg_id.to_string(),
            &as_on_dt,
            curr_id,
            amount
        );
        write!(file_writer, "{}", str_write);
    }
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config.output_file_path());
}
