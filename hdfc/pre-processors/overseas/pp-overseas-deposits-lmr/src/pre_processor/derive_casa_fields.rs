use super::remove_comma;
use calamine::DataType;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_casa_op_line(
    rec: &mut Vec<&str>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_bdp_coa: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    log: &Logger,
) -> String {
    let mut ora_mis1 = String::new();
    ora_mis1.push('1');
    ora_mis1.push_str(&rec[23].to_string());

    let ora_prod = t_ora_prod
        .entry(rec[5].to_string())
        .or_insert("".to_string());

    let ora_gl = t_ora_gl
        .entry(rec[16].to_string())
        .or_insert("".to_string());

    let alm_concat = get_alm_concat(&ora_mis1, ora_prod, ora_gl);

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert("NONE".to_string());

    let mut coa_concat = String::new();
    coa_concat.push_str(ora_prod);
    coa_concat.push('_');
    coa_concat.push_str(ora_gl);

    let coa = t_bdp_coa
        .entry(coa_concat.to_string())
        .or_insert("".to_string());

    let div = div.entry(ora_mis1.to_string()).or_insert("".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, div: `{}`, coa_concat: `{}`, coa: `{}`.",
        rec[1],
        alm_concat,
        alm_line,
        div,
        coa_concat,
        coa
    );
    get_line(rec, div, alm_line, log)
}

fn get_line(val: &mut Vec<&str>, div: &mut String, alm_line: &mut String, log: &Logger) -> String {
    let a_o_dt: String;
    if val[7].to_string().parse::<f64>().is_err() {
        a_o_dt = get_date(val[7], "account opening date", val[1], log)
    } else {
        a_o_dt = datevalue_to_date(val[7].to_string());
    }

    let mut output_line = String::new();
    output_line.push_str(&val[1].to_string());
    output_line.push('|');
    output_line.push_str(&val[0].to_string());
    output_line.push('|');
    output_line.push_str(&val[5].to_string());
    output_line.push('|');
    output_line.push_str(&val[14].to_string());
    output_line.push('|');
    output_line.push_str(&remove_comma(val[20].to_string()).to_string());
    output_line.push('|');
    output_line.push_str(&val[21].to_string());
    output_line.push_str("|||");
    output_line.push_str(&a_o_dt);
    output_line.push('|');
    output_line.push_str(&val[3].to_string());
    output_line.push('|');
    output_line.push_str(&val[4].to_string());
    output_line.push_str("||||||||||");
    output_line.push_str(&val[9].to_string());
    output_line.push_str("||F|");
    output_line.push_str(&val[6].to_string());
    output_line.push('|');
    output_line.push_str(&remove_comma(val[20].to_string()).to_string());
    output_line.push_str("||");
    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push_str("|||||||");
    output_line.push_str(&val[16].to_string());
    output_line.push_str("\n");

    output_line
}

fn get_date(val: &str, name: &str, acc: &str, log: &Logger) -> String {
    if let Ok(dt) = NaiveDate::parse_from_str(&val.to_string(), "%d-%b-%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        log_error!(
            log,
            "`{}`: `{}` is not well-formatted for account: `{}`, expected format: `DD-MMM-YYYY`.",
            name,
            val,
            acc
        );
        "".to_string()
    }
}

fn datevalue_to_date(date: String) -> String {
    match date.parse::<f64>() {
        Ok(timestamp) => date_from_timestamp(((timestamp as i64) - 25569) * 86400)
            .format("%d-%m-%Y")
            .to_string(),
        Err(_) => "".to_string(),
    }
}

fn get_alm_concat(ora_mis1: &str, ora_prod: &str, ora_gl: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_prod);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat
}
