pub mod file_reader;
use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::string::ToString;
mod io;
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
    for line in data_info.input_data.lines().skip(1) {
        let input_data: Vec<&str> = line.split("~#~").collect();
        if input_data.len() == 6 {
            let info_curr = &input_data[5].to_string();
            let llg_id = find_llg_id(currency_map.clone(), &info_curr)
                .parse::<i64>()
                .unwrap_or(0);
            let amount = if input_data[5].to_string().eq(config.base_currency()) {
                input_data[0].parse::<f64>().unwrap_or(0.0)
            } else {
                input_data[1].parse::<f64>().unwrap_or(0.0)
            };
            let as_on_dt = NaiveDate::parse_from_str(input_data[3], "%Y-%m-%d %H:%M:%S %.6f")
                .expect("Invalid date hence not able to convert as on date");
            let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();
            let series_dt = NaiveDate::parse_from_str(input_data[4], "%Y-%m-%d %H:%M:%S %.6f")
                .expect("Invalid date hence not able to convert series date");
            let series_dt = series_dt.format("%d-%m-%Y").to_string();
            let curr_id = input_data[5].to_string();
            //here the concatenation fields with a pipe delimeter and appending it with next line are performed for ouptut file
            let str_write = format!(
                "{0}{3}{1}{3}{7}{3}{2}{3}{4}{3}{4}{3}{4}{3}{4}{3}{4}{3}{5}{6}", //the numbers here indicate the position of the pamaters being used in the output file
                &llg_id.to_string(),
                &as_on_dt,
                &curr_id,
                "|",
                "ALL",
                &(amount.to_string()),
                "\n",
                &series_dt
            );
            //here we are using io library for writing the content present str_write variable to the output file
            write!(file_writer, "{}", str_write).expect("Error while writing output.");
        }
    }
}
