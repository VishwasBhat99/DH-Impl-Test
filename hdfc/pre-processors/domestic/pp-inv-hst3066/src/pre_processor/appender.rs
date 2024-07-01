use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(rec: &mut Vec<&str>, as_on_dt: NaiveDate, log: &Logger) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");
    get_line(rec, &dt.to_string())
}

fn get_line(val: &Vec<&str>, dt: &str) -> String {
    let mut output_line = String::new();
    output_line.push_str(val[0]);
    output_line.push_str("|");
    output_line.push_str(val[1]);
    output_line.push('|');
    let mat_dt = if let Ok(dt) = NaiveDate::parse_from_str(val[2], "%d-%b-%y") {
        dt.format("%d-%m-%y").to_string()
    } else {
        "".to_string()
    };
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(val[3]);
    output_line.push('|');
    output_line.push_str(val[4]);
    output_line.push('|');
    output_line.push_str(val[5]);
    output_line.push('|');
    output_line.push_str(val[6]);
    output_line.push('|');
    output_line.push_str(dt);
    output_line.push('|');
    output_line.push_str("INR");
    output_line.push('\n');
    output_line
}
