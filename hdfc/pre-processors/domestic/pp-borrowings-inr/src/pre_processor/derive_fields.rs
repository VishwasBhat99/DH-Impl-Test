use super::output_lines::OutputLines;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut Vec<&str>,
    gl: &mut HashMap<String, String>,
    ora_mis1: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    alm_concat: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> OutputLines {
    let mut output_lines = OutputLines::new();
    let dt = as_on_dt.format("%d-%m-%Y");

    let o_sys_gl = gl.entry(rec[1].to_string()).or_insert("".to_string());

    let ora_mis1 = ora_mis1
        .entry(o_sys_gl.to_string())
        .or_insert_with(|| "".to_string());

    let alm_concat = alm_concat
        .entry(o_sys_gl.to_string())
        .or_insert_with(|| "NONE".to_string());

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let div = div
        .entry(ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, operation type: `{}`, gl: `{}`, alm_concat: `{}`, alm_line: `{}`, div: `{}`.",
        rec[0],
        rec[2],
        o_sys_gl,
        alm_concat,
        alm_line,
        div
    );
    output_lines.processed_lines = get_line(rec, &dt.to_string(), o_sys_gl, div, alm_line);

    if alm_line == "NONE" {
        output_lines.concat_lines.push(alm_concat.to_string());
    }

    output_lines
}

fn get_line(
    val: &Vec<&str>,
    dt: &str,
    gl: &mut String,
    div: &mut String,
    alm_line: &mut String,
) -> String {
    let deal_dt = match NaiveDate::parse_from_str(val[6], "%d-%b-%y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(_) => "".to_string(),
    };

    let val_dt = match NaiveDate::parse_from_str(val[5], "%d-%b-%y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(_) => "".to_string(),
    };

    let mat_dt = match NaiveDate::parse_from_str(val[13], "%d-%b-%y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(_) => "".to_string(),
    };

    let mut output_line = String::new();
    output_line.push_str(&val[0].to_string());
    output_line.push_str("|F|HDFCC|");
    output_line.push_str(&gl);
    output_line.push('|');
    output_line.push_str(&val[11].to_string());
    output_line.push('|');
    output_line.push_str(&deal_dt);
    output_line.push('|');
    output_line.push_str(&val_dt);
    output_line.push('|');
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(&val[7].to_string());
    output_line.push('|');
    output_line.push_str(&val[10].to_string());
    output_line.push('|');
    output_line.push_str(&val[8].to_string());
    output_line.push('|');
    output_line.push_str(&val[10].to_string());
    output_line.push('|');
    output_line.push_str(&val[8].to_string());
    output_line.push('|');
    output_line.push_str(&val[10].to_string());
    output_line.push('|');
    output_line.push_str(&val[8].to_string());
    output_line.push('|');
    output_line.push_str(&val[15].to_string());
    output_line.push('|');
    output_line.push_str(&val[15].to_string());
    output_line.push('|');
    output_line.push_str(&val[14].to_string());
    output_line.push('|');
    output_line.push_str(&val[14].to_string());
    output_line.push('|');
    output_line.push_str(&val[1].to_string());
    output_line.push('|');
    output_line.push_str(&val[1].to_string());
    output_line.push('|');
    output_line.push_str(&val[3].to_string());
    output_line.push('|');
    output_line.push_str(dt);
    output_line.push_str("|9|F|FIXED|0|");
    output_line.push_str(&alm_line);
    output_line.push('|');
    output_line.push_str(&div);
    output_line.push('\n');
    output_line
}
