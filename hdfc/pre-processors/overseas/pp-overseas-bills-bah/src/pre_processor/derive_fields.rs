use super::remove_comma;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut [String],
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, Vec<String>>,
    t_bdp_coa: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    mut alm_line: String,
    as_on_dt: NaiveDate,
    log: &Logger,
    sma_data: &HashMap<String, String>,
) -> (String, String) {
    let mut ora_mis1 = String::new();
    ora_mis1.push('1');
    ora_mis1.push_str(&rec[30]);

    let ora_prod = t_ora_prod
        .entry(rec[4].clone())
        .or_insert_with(|| "".to_string());

    let mut ora_gl = "_".to_string();
    let mut ora_gl_code = &"".to_string();
    if t_ora_gl.contains_key(&rec[16]) {
        ora_gl_code = &t_ora_gl.get(&rec[16]).expect("Could Not Find Value")[0];
        ora_gl = t_ora_gl.get(&rec[16]).expect("Could Not Find Value")[1].to_owned()
            + "_"
            + &t_ora_gl.get(&rec[16]).expect("Could Not Find Value")[0].to_owned();
    }

    let alm_concat = get_alm_concat(&ora_mis1, &ora_gl);

    let mut coa_concat = String::new();
    coa_concat.push_str(ora_prod);
    coa_concat.push('_');
    coa_concat.push_str(ora_gl_code);

    let coa = t_bdp_coa
        .entry(coa_concat.to_string())
        .or_insert_with(|| "".to_string());

    let div = div
        .entry(ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, div: `{}`, coa_concat: `{}`, coa: `{}`.",
        rec[4],
        alm_concat,
        alm_line,
        div,
        coa_concat,
        coa
    );
    let concat: String = rec[1].to_string() + "|" + &alm_concat + "|" + &alm_line + "\n";
    (
        get_line(rec, as_on_dt, div, &mut alm_line, log, sma_data),
        concat,
    )
}

fn get_line(
    val: &mut [String],
    as_on_dt: NaiveDate,
    div: &mut str,
    alm_line: &mut str,
    log: &Logger,
    sma_data: &HashMap<String, String>,
) -> String {
    let val_dt = if val[15].parse::<f64>().is_err() {
        get_date(&val[15], "value_date", &val[4], log)
    } else {
        datevalue_to_date(val[15].clone())
    };

    let mat_dt = if val[16].parse::<f64>().is_err() {
        get_date(&val[16], "maturity_date", &val[4], log)
    } else {
        datevalue_to_date(val[16].clone())
    };

    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

    let mut output_line = String::new();
    output_line.push_str(&val[4]);
    output_line.push('|');
    output_line.push_str(&val[7]);
    output_line.push('|');
    output_line.push_str(&val[14]);
    output_line.push('|');
    output_line.push_str(&val_dt);
    output_line.push('|');
    output_line.push_str(&mat_dt);
    output_line.push_str("||");
    output_line.push_str(&val[22]);
    output_line.push_str("||||");
    output_line.push_str(&remove_comma(val[21].clone()));
    output_line.push_str("||");
    // TODO: missing gl currently taken as empty
    output_line.push('|');
    output_line.push_str(&remove_comma(val[37].clone()));
    output_line.push_str("||");
    output_line.push_str(&val[30]);
    output_line.push('|');
    output_line.push_str(&val[1]);
    output_line.push('|');
    output_line.push_str(&val[2]);
    output_line.push_str("|1|F|");
    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push_str("|||");
    output_line.push_str(&as_on_dt);
    output_line.push_str("|||||||||||");
    output_line.push_str(
        &sma_data
            .get(&val[4].to_string())
            .unwrap_or(&"P".to_string())
            .to_string(),
    );
    output_line.push('\n');
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

fn get_date(val: &String, name: &str, acc: &String, log: &Logger) -> String {
    if let Ok(dt) = NaiveDate::parse_from_str(val, "%d-%m-%Y") {
        dt.format("%d-%m-%Y").to_string()
    } else {
        log_error!(
            log,
            "`{}`: `{}` is not well-formatted for account: `{}`, expected format: `DD-MM-YYYY`.",
            name,
            val,
            acc
        );
        "".to_string()
    }
}

fn get_alm_concat(ora_mis1: &str, ora_gl: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat
}
