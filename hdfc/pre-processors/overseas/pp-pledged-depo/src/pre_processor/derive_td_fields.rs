use super::remove_comma;
use calamine::DataType;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn get_td_op_line(
    rec: &Vec<&str>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_bdp_coa: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
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
    get_line(rec, as_on_dt, alm_line, log)
}

fn get_line(val: &Vec<&str>, as_on_dt: NaiveDate, alm_line: &mut String, log: &Logger) -> String {
    let a_o_dt: String;
    if val[7].to_string().parse::<f64>().is_err() {
        a_o_dt = get_date(&val[7], "account opening date", &val[1], log)
    } else {
        a_o_dt = datevalue_to_date(val[7].to_string());
    }

    let mat_dt: String;
    if val[8].to_string().parse::<f64>().is_err() {
        mat_dt = get_date(&val[8], "maturity_date", &val[1], log)
    } else {
        mat_dt = datevalue_to_date(val[8].to_string());
    }

    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();
    let lien_amount = get_lien_amount(&val[21].to_string(), &val[19].to_string());
    let aip_amount = get_aip_amount(&lien_amount, &val[37].to_string());
    log_debug!(
        log,
        "ACY_AVL_BAL(T:21):{} - ACY_CURR_BAL(V:19):{} + ACCRUED_CR_IC_LCY_AMOUNT(AL:37):{}= {}",
        &val[21].to_string(),
        &val[19].to_string(),
        &val[37].to_string(),
        aip_amount
    );
    let mut output_line = String::new();
    output_line.push_str(&val[1].to_string());
    output_line.push_str("||");
    output_line.push_str(&val[5].to_string());
    output_line.push('|');
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(&val[9].to_string());
    output_line.push('|');
    output_line.push_str(&val[13].to_string());
    output_line.push_str("|||");
    output_line.push_str(&a_o_dt);
    output_line.push_str("||");
    output_line.push_str(&val[4].to_string());
    output_line.push_str("|||");
    output_line.push_str(&val[18].to_string());
    output_line.push_str("||");
    output_line.push_str(&val[3].to_string());
    output_line.push_str("||");
    output_line.push_str(&as_on_dt);
    output_line.push_str("||");
    output_line.push_str(&val[0].to_string());
    output_line.push_str("|F||0|");
    output_line.push_str(&val[6].to_string());
    output_line.push_str("||");

    let rt_acc_int: f64 = val[9].to_string().parse().unwrap_or(DEFAULT_FLOAT);
    let float_rt: f64 = val[13].to_string().parse().unwrap_or(DEFAULT_FLOAT);
    let int_rt = rt_acc_int + float_rt;
    output_line.push_str(&int_rt.to_string());
    output_line.push('|');

    output_line.push_str(alm_line);
    output_line.push_str("|||");
    output_line.push_str(&remove_comma(val[20].to_string()).to_string());
    output_line.push_str("|100|0|");
    output_line.push_str("|");
    output_line.push_str(&val[18].to_string());
    output_line.push_str("|");
    output_line.push_str(&aip_amount);
    output_line.push('\n');
    output_line
}
fn get_lien_amount(acy_curr_bal: &String, acy_avl_bal: &String) -> String {
    let curr_bal = acy_curr_bal
        .parse::<f64>()
        .expect("unable to convert acy_curr_bal");
    let avl_bal = acy_avl_bal
        .parse::<f64>()
        .expect("unable to convert acy_curr_bal");
    let lien_amount = curr_bal - avl_bal;
    lien_amount.to_string()
}

fn get_aip_amount(lien_amt: &String, accrued_lcy_amt: &String) -> String {
    let lien_bal = lien_amt.parse::<f64>().expect("unable to convert lien_amt");
    let accrued_bal = accrued_lcy_amt
        .parse::<f64>()
        .expect("unable to convert accrued_lcy_amt");
    let mut aip_amount = lien_bal + accrued_bal;
    if lien_bal == 0.0 {
        aip_amount = 0.0;
    }
    aip_amount.to_string()
}
fn datevalue_to_date(date: String) -> String {
    match date.parse::<f64>() {
        Ok(timestamp) => date_from_timestamp(((timestamp as i64) - 25569) * 86400)
            .format("%d-%m-%Y")
            .to_string(),
        Err(_) => "".to_string(),
    }
}

fn get_date(val: &str, name: &str, acc: &str, log: &Logger) -> String {
    if let Ok(dt) = NaiveDate::parse_from_str(&val.to_string(), "%d-%m-%Y") {
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

fn get_alm_concat(ora_mis1: &str, ora_prod: &str, ora_gl: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_prod);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat
}
