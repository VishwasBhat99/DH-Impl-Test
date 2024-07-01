use chrono::{Datelike, Local, NaiveDate};
use configuration_parameters::ConfigurationParameters;

pub fn get_op_line(val: &[&str], as_on_dt: NaiveDate) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");

    let now = Local::now();
    let crnt_dt = NaiveDate::from_ymd(now.year(), now.month(), now.day())
        .format("%d-%m-%Y")
        .to_string();

    let mut output_line = String::new();
    output_line.push_str(val[0]);
    output_line.push('|');
    output_line.push_str(val[1]);
    output_line.push('|');
    output_line.push_str(val[2]);
    output_line.push('|');
    output_line.push_str(&dt.to_string());
    output_line.push_str("|Y|dataharmonizer|");
    output_line.push_str(&crnt_dt);
    output_line.push_str("|dataharmonizer|");
    output_line.push_str(&crnt_dt);
    output_line.push('\n');

    output_line
}

pub fn append_op_line(lines: &mut String, config_param: &ConfigurationParameters) {
    let dt = config_param.as_on_date().format("%d-%m-%Y");

    let now = Local::now();
    let crnt_dt = NaiveDate::from_ymd(now.year(), now.month(), now.day())
        .format("%d-%m-%Y")
        .to_string();

    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.ccy(),
        config_param.lcy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.lcy(),
        config_param.ccy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.ccy(),
        config_param.ccy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.fcy(),
        config_param.ccy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.ccy(),
        config_param.fcy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
}
