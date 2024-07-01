use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut Vec<&str>,
    gl: &mut HashMap<String, String>,
    div: &String,
    alm_line: &String,
    log: &Logger,
) -> String {
    let gl = gl
        .entry(rec[3].to_string())
        .or_insert_with(|| rec[3].to_string());

    log_debug!(
        log,
        "account: `{}`, gl: `{}`, division: `{}`, alm_line:`{}`",
        rec[1],
        gl,
        div,
        alm_line
    );
    get_line(rec, gl, div, alm_line, log)
}

fn get_line(
    val: &Vec<&str>,
    gl: &mut String,
    div: &String,
    alm_line: &String,
    log: &Logger,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(val[1]);
    output_line.push('|');
    output_line.push_str(val[13]);
    output_line.push_str("|INR|");
    output_line.push_str(&gl);
    output_line.push('|');
    output_line.push_str(val[6]);
    output_line.push('|');
    output_line.push_str(val[12]);
    output_line.push('|');

    let st_dt = NaiveDate::parse_from_str(val[7], "%d-%b-%y");
    if let Ok(dt) = st_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_error!(
            log,
            "`start_date` is not well-formatted for account: `{}`",
            val[1]
        );
    }
    output_line.push('|');

    let mat_dt = NaiveDate::parse_from_str(val[8], "%d-%b-%y");
    if let Ok(dt) = mat_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_error!(
            log,
            "`maturity_date` is not well-formatted for account: `{}`",
            val[1]
        );
    }
    output_line.push('|');

    output_line.push_str(alm_line);
    output_line.push('|');
    output_line.push_str(div);
    output_line.push('\n');
    output_line
}
