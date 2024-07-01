use calamine::DataType;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &[DataType],
    t_ora_mis1: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_bdp_coa: &mut HashMap<String, String>,
    ccy: &str,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> String {
    let ora_mis1 = t_ora_mis1
        .entry("".to_string())
        .or_insert_with(|| "".to_string());
    let ora_prod = t_ora_prod
        .entry("".to_string())
        .or_insert_with(|| "".to_string());
    let ora_gl = t_ora_gl
        .entry("".to_string())
        .or_insert_with(|| "".to_string());

    let alm_concat = get_alm_concat(ora_mis1, ora_prod, ora_gl);

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let mut coa_concat = String::new();
    coa_concat.push_str(ora_prod);
    coa_concat.push('_');
    coa_concat.push_str(ora_gl);

    let coa = t_bdp_coa
        .entry(coa_concat.to_string())
        .or_insert_with(|| "".to_string());

    let div = div
        .entry(ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, div: `{}`, coa_concat: `{}`, coa: `{}`.",
        rec[0],
        alm_concat,
        alm_line,
        div,
        coa_concat,
        coa
    );
    get_line(rec, as_on_dt, alm_line, div, ccy, log)
}

fn get_line(
    val: &[DataType],
    as_on_dt: NaiveDate,
    alm_line: &mut String,
    div: &mut String,
    ccy: &str,
    log: &Logger,
) -> String {
    let deal_dt: String;
    if val[4].to_string().parse::<f64>().is_err() {
        deal_dt = get_date(&val[4], "deal_date", &val[0], log)
    } else {
        deal_dt = datevalue_to_date(val[4].to_string());
    }

    let val_dt: String;
    if val[5].to_string().parse::<f64>().is_err() {
        val_dt = get_date(&val[5], "value_date", &val[0], log)
    } else {
        val_dt = datevalue_to_date(val[5].to_string());
    }

    let mat_dt: String;
    if val[10].to_string().parse::<f64>().is_err() {
        mat_dt = get_date(&val[10], "maturity_date", &val[0], log)
    } else {
        mat_dt = datevalue_to_date(val[10].to_string());
    }

    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

    let mut output_line = String::new();
    output_line.push_str(&val[0].to_string());
    output_line.push_str("||||");
    output_line.push_str(&val[8].to_string());
    output_line.push('|');
    output_line.push_str(&deal_dt);
    output_line.push('|');
    output_line.push_str(&val_dt);
    output_line.push('|');
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(ccy);
    output_line.push('|');

    let oper_type = val[2].to_string().to_uppercase();
    let orgballcy = if oper_type == "BORROWING" {
        val[19].to_string()
    } else if oper_type == "LENDING" {
        val[18].to_string()
    } else {
        "0.0".to_string()
    };
    output_line.push_str(&orgballcy);
    output_line.push_str("|||||");

    output_line.push_str(&val[12].to_string());
    output_line.push_str("|||0||");
    output_line.push_str(&val[2].to_string());
    output_line.push_str("||");
    output_line.push_str(&val[3].to_string());
    output_line.push('|');
    output_line.push_str(&as_on_dt);
    output_line.push_str("||F|FIXED|0|");
    output_line.push_str(ccy);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push('|');
    output_line.push_str(div);
    output_line.push_str("\n");
    output_line
}

fn datevalue_to_date(date: String) -> String {
    match date.parse::<f64>() {
        Ok(timestamp) => date_from_timestamp(((timestamp as i64) - 25569) * 86400)
            .format("%d-%m-%Y")
            .to_string(),
        Err(_) => "".to_string(),
    }
}

fn get_date(val: &DataType, name: &str, acc: &DataType, log: &Logger) -> String {
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

fn get_alm_concat(ora_mis1: &str, ora_prod: &str, ora_gl: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_prod);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat
}
