use macros;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_op_line(
    rec: &mut Vec<&str>,
    ccy: &str,
    div: &String,
    alm_line: &String,
    ia_line: &String,
    log: &Logger,
) -> String {
    log_debug!(
        log,
        "account: `{}`, division: `{}`, alm_line:`{}`, ia line: `{}`",
        rec[4],
        div,
        alm_line,
        ia_line
    );

    get_line(rec, ccy, div, alm_line,ia_line)
}

fn get_line(val: &Vec<&str>, ccy: &str, div: &String, alm_line: &String,ia_line: &String) -> String {
    let mut output_line = String::new();
    output_line.push_str(val[4]);
    output_line.push('|');
    output_line.push_str(val[2]);
    output_line.push('|');
    output_line.push_str(ccy);
    output_line.push('|');
    output_line.push_str(val[24]);
    output_line.push('|');
    output_line.push_str(val[26]);
    output_line.push('|');
    output_line.push_str(val[8]);
    output_line.push('|');

    let st_dt = if let Ok(dt) = NaiveDate::parse_from_str(val[5], "%d-%m-%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else if let Ok(dt) = NaiveDate::parse_from_str(val[5], "%d/%m/%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        "".to_string()
    };
    output_line.push_str(&st_dt);
    output_line.push('|');

    let mat_dt = if let Ok(dt) = NaiveDate::parse_from_str(val[6], "%d-%m-%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else if let Ok(dt) = NaiveDate::parse_from_str(val[6], "%d/%m/%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        "".to_string()
    };
    output_line.push_str(&mat_dt);
    output_line.push('|');

    output_line.push_str(val[0]);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push('|');
    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(val[23]);
    output_line.push('|');
    output_line.push_str(val[18]);
    output_line.push('|');
    output_line.push_str(val[19]);
    output_line.push('|');
    output_line.push_str(val[20]);
    output_line.push('|');
    output_line.push_str(val[0]);
    output_line.push('|');
    output_line.push_str(ia_line);
    output_line
}
