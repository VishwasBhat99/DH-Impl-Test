use rbdate::NaiveDate;

pub fn get_op_line(rec: &mut Vec<&str>, as_on_dt: NaiveDate) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");
    get_line(rec, &dt.to_string())
}

fn get_line(val: &Vec<&str>, dt: &str) -> String {
    let mut output_line = String::new();

    output_line.push_str(val[0]);
    output_line.push_str("|");
    output_line.push_str(val[1]);
    output_line.push('|');
    output_line.push_str(val[2]);
    output_line.push('|');
    output_line.push_str(val[3]);
    output_line.push('|');
    let deal_dt = if let Ok(dt) = NaiveDate::parse_from_str(val[4], "%d-%b-%y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        "".to_string()
    };
    output_line.push_str(&deal_dt);
    output_line.push('|');
    let val_dt = if let Ok(dt) = NaiveDate::parse_from_str(val[5], "%d-%b-%y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        "".to_string()
    };
    output_line.push_str(&val_dt);
    output_line.push('|');
    output_line.push_str(val[6]);
    output_line.push('|');
    output_line.push_str(val[7]);
    output_line.push('|');
    output_line.push_str(val[8]);
    output_line.push('|');
    output_line.push_str(val[9]);
    output_line.push('|');
    let mat_dt = if let Ok(dt) = NaiveDate::parse_from_str(val[10], "%d-%b-%y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        "".to_string()
    };
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(val[11]);
    output_line.push('|');
    output_line.push_str(val[12]);
    output_line.push('|');
    output_line.push_str(val[13]);
    output_line.push('|');
    output_line.push_str(val[14]);
    output_line.push('|');
    output_line.push_str(val[15]);
    output_line.push('|');
    output_line.push_str(val[16]);
    output_line.push('|');
    output_line.push_str(val[17]);
    output_line.push('|');
    output_line.push_str(val[18]);
    output_line.push('|');
    output_line.push_str(val[19]);
    output_line.push('|');
    output_line.push_str(dt);
    output_line.push('|');
    output_line.push_str("INR");
    output_line.push('\n');
    output_line
}
