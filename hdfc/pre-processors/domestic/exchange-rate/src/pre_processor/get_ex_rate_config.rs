use chrono::{Datelike, Local, NaiveDate};

pub fn write_config_output(config_str: &str, input_vec: &[&str], as_on_dt: NaiveDate) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");
    let now = Local::now();
    let crnt_dt = NaiveDate::from_ymd(now.year(), now.month(), now.day())
        .format("%d-%m-%Y")
        .to_string();
    let recipro = 1.000 / (input_vec[2].parse::<f32>().unwrap());
    let output_line = format!(
        "{}|{}|{}|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n{}|{}|{:.4}|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        input_vec[0],
        config_str,
        input_vec[2],
        &dt.to_string(),
        &crnt_dt,
        &crnt_dt,
        config_str,
        input_vec[0],
        recipro,
        &dt.to_string(),
        &crnt_dt,
        &crnt_dt,
    );
    output_line
}

pub fn write_config_exrt(config_str: &str, input_vec: &[&str]) -> String {
    let recipro = 1.000 / (input_vec[2].parse::<f32>().unwrap());
    let mut exrt_line = format!(
        "{}|{}|{}\n{}|{}|{:.4}\n",
        input_vec[0], config_str, input_vec[2], config_str, input_vec[0], recipro,
    );
    exrt_line
}
