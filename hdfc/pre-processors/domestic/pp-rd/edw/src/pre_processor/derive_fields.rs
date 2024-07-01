use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut Vec<&str>,
    gl: &mut HashMap<String, String>,
    alm_concat_map: &mut HashMap<String, String>,
    div: &str,
    alm_line: &str,
    log: &Logger,
) -> String {
    let gl = gl
        .entry(rec[3].to_string())
        .or_insert_with(|| rec[3].to_string());
    let alm_concat = match alm_concat_map.get(gl) {
        Some(val) => val,
        None => {
            log_error!(log, "Could not get alm_concat for source gl: {}", &gl);
            "NA"
        }
    };
    log_debug!(
        log,
        "account: `{}`, gl: `{}`, division: `{}`, alm_line:`{}`, alm_concat: `{}`",
        rec[1],
        gl,
        div,
        alm_line,
        alm_concat
    );
    get_line(rec, gl, div, alm_line, alm_concat, log)
}

fn get_line(
    val: &[&str],
    gl: &mut String,
    div: &str,
    alm_line: &str,
    alm_concat: &str,
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

    let st_dt = NaiveDate::parse_from_str(val[7], "%d-%b-%Y");
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

    let mat_dt = NaiveDate::parse_from_str(val[8], "%d-%b-%Y");
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
    output_line.push('|');
    output_line.push_str(val[2]);
    output_line.push('|');
    output_line.push_str(val[3]);
    output_line.push('|');
    let cod_mis_comp_1 = if let Some(val) = val.get(15) { val } else { "" };
    output_line.push_str(&cod_mis_comp_1);
    output_line.push('|');
    let val_date = NaiveDate::parse_from_str(val[7], "%d-%b-%Y");
    if let Ok(dt) = val_date {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_error!(
            log,
            "`value_date` is not well-formatted for account: `{}`",
            val[1]
        );
    }
    output_line.push('|');
    output_line.push_str(alm_concat);
    output_line.push('\n');
    output_line
}
