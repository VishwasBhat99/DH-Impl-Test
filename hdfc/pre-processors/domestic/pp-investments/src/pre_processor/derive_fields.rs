use super::OutputLines;
use calamine::DataType;
use chrono::Datelike;
use macros;
use rbdate::{date_from_timestamp, increment_date_by_months, num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &[DataType],
    t_ora_mis1: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_ora_cat: &mut HashMap<String, String>,
    o_sys_gl: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> OutputLines {
    let mut output_lines = OutputLines::new();
    let pd_cd = rec[17].to_string().as_str().replace("\u{a0}", " ");
    let o_sys_gl = o_sys_gl.entry(pd_cd).or_insert_with(|| "".to_string());

    let ora_mis1 = t_ora_mis1
        .entry(o_sys_gl.to_string())
        .or_insert_with(|| "".to_string());
    let alm_concat = get_alm_concat(
        ora_mis1,
        t_ora_prod
            .entry(o_sys_gl.to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_gl
            .entry(o_sys_gl.to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_cat
            .entry(o_sys_gl.to_string())
            .or_insert_with(|| "".to_string()),
    );

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let div = div
        .entry(ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, gl: `{}`, alm_concat: `{}`, alm_line: `{}`, div: `{}`.",
        rec[0],
        rec[19],
        alm_concat,
        alm_line,
        div
    );
    output_lines.processed_lines = get_line(rec, as_on_dt, div, alm_line, log);

    if alm_line == "NONE" {
        output_lines.concat_lines.push(alm_concat);
    }
    output_lines
}

fn get_line(
    val: &[DataType],
    as_on_dt: NaiveDate,
    div: &mut String,
    alm_line: &mut String,
    log: &Logger,
) -> String {
    let nxt_rep_dt = datevalue_to_date(val[10].to_string());
    let deal_dt = datevalue_to_date(val[20].to_string());
    let nxt_coup_dt = datevalue_to_date(val[9].to_string());
    let mat_dt = datevalue_to_date(val[13].to_string());
    let lst_int_dt = datevalue_to_date(val[8].to_string());
    let val_dt = datevalue_to_date(val[22].to_string());
    let call_dt = datevalue_to_date(val[14].to_string());
    let put_dt = datevalue_to_date(val[15].to_string());
    let mut output_line = String::new();
    output_line.push_str(&val[0].to_string());
    output_line.push('|');
    output_line.push_str(&val[2].to_string().trim());
    output_line.push('|');
    output_line.push_str(&nxt_rep_dt);
    output_line.push('|');
    output_line.push_str(&call_dt);
    output_line.push('|');
    output_line.push_str(&put_dt);
    output_line.push('|');
    output_line.push_str(&deal_dt);
    output_line.push('|');
    output_line.push_str(&val[24].to_string());
    output_line.push('|');
    output_line.push_str(&val[27].to_string());
    output_line.push('|');
    output_line.push_str(&val[29].to_string());
    output_line.push('|');
    output_line.push_str(&val[30].to_string());
    output_line.push('|');
    output_line.push_str(&val[31].to_string());
    output_line.push('|');
    output_line.push_str(&val[33].to_string());
    output_line.push('|');
    output_line.push_str(&val[39].to_string());
    output_line.push('|');

    let int_basis: i32 = match val[6].to_string().to_uppercase().as_str() {
        "30EBY360" => 5,
        "ACTUALBY365" => 4,
        "ANNUAL" => 4,
        "ANNUALBY2" => 5,
        _ => 0,
    };
    output_line.push_str(&int_basis.to_string());
    output_line.push('|');

    output_line.push_str(&val[28].to_string());
    output_line.push('|');
    output_line.push_str(&val[28].to_string());
    output_line.push('|');
    output_line.push_str(&val[38].to_string());
    output_line.push('|');
    output_line.push_str(&val[31].to_string());
    output_line.push('|');
    output_line.push_str(&val[7].to_string());
    output_line.push('|');
    output_line.push_str(&nxt_coup_dt);
    output_line.push('|');
    output_line.push_str(&val[19].to_string());
    output_line.push('|');
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(&val[2].to_string().trim());
    output_line.push('|');
    output_line.push_str(&val[18].to_string());
    output_line.push('|');
    output_line.push_str(&val[17].to_string());
    output_line.push('|');
    output_line.push_str(&lst_int_dt);
    output_line.push_str("|NULL|");
    output_line.push_str(&val[4].to_string());
    output_line.push('|');
    output_line.push_str(&val_dt);
    output_line.push_str("|DAILY|||INR|");

    let org_term: f64 = if let (Some(st_dt), Some(end_dt)) = (
        datevalue_to_naive_date(val[22].to_string()),
        datevalue_to_naive_date(val[13].to_string()),
    ) {
        if st_dt < end_dt {
            ((num_days_start_to_end(st_dt, end_dt) as f64) / 365.0) * 12.0
        } else {
            log_error!(
                log,
                "`start date`: `{}` is greater than `end date`: `{}` for account: `{}`",
                st_dt,
                end_dt,
                val[0]
            );
            0.0
        }
    } else {
        0.0
    };
    output_line.push_str(&org_term.to_string());
    output_line.push('|');

    let acc_basis: &str = match int_basis {
        1 => "B",
        2 => "D",
        3 => "E",
        4 => "F",
        5 => "B",
        6 => "E",
        7 => "F",
        0 => "F",
        9 => "2",
        _ => "F",
    };
    output_line.push_str(acc_basis);
    output_line.push('|');

    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push('|');

    let invst_typ = val[17].to_string();
    let comp_freq: i32 = match invst_typ.as_str() {
        "COMMP" => 3000,
        "TBILL" => 3000,
        "ZCORP" => 3000,
        "ZECBD" => 3000,
        _ => 0,
    };
    output_line.push_str(&comp_freq.to_string());
    output_line.push('|');

    let nxt_comp_dt: Option<NaiveDate> = match invst_typ.as_str() {
        "COMMP" => as_on_dt.succ_opt(),
        "TBILL" => as_on_dt.succ_opt(),
        "ZCORP" => as_on_dt.succ_opt(),
        "ZECBD" => as_on_dt.succ_opt(),
        _ => None,
    };
    let nxt_comp_dt = match nxt_comp_dt {
        Some(dt) => dt.format("%d-%m-%Y").to_string(),
        None => "NULL".to_string(),
    };
    output_line.push_str(&nxt_comp_dt);
    output_line.push('|');

    let rt_chng_freq: i32 = match invst_typ.as_str() {
        "ZCORP" => 3000,
        "GSFRB" => 6,
        _ => 0,
    };
    output_line.push_str(&rt_chng_freq.to_string());
    output_line.push('|');

    let int_type = val[3].to_string();
    let put_date = datevalue_to_naive_date(val[15].to_string());
    let nxt_rep_dt = datevalue_to_naive_date(val[10].to_string());
    let rt_flag: &str = if (int_type == "Fixed" || int_type == "None") && put_date.is_none() {
        "F"
    } else if (int_type == "Fixed" || int_type == "None") && put_date.is_some() {
        "P"
    } else if int_type == "Floating" && nxt_rep_dt.is_none() {
        "V"
    } else if int_type == "Floating" && nxt_rep_dt.is_some() {
        "A"
    } else {
        "NULL"
    };
    output_line.push_str(rt_flag);
    output_line.push('|');
    output_line.push_str(&invst_typ);
    output_line.push('|');

    let mat_dt = datevalue_to_naive_date(val[13].to_string());
    let n_c_dt = datevalue_to_naive_date(val[9].to_string());
    let nxt_pay_dt: Option<NaiveDate> = if invst_typ == "ZCORP" {
        if n_c_dt.is_some() {
            n_c_dt
        } else if mat_dt.is_some()
            && mat_dt.expect("Error while parsing maturity date.")
                <= increment_date_by_months(as_on_dt, 1)
        {
            mat_dt
        } else {
            Some(increment_date_by_months(as_on_dt, 1))
        }
    } else if invst_typ == "GSFRB" {
        if n_c_dt.is_some() {
            n_c_dt
        } else {
            mat_dt
        }
    } else {
        None
    };

    if let Some(dt) = nxt_pay_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        output_line.push_str("NULL");
    }
    output_line.push('|');

    let prev_rep_dt: String = match invst_typ.as_str() {
        "ZCORP" => as_on_dt.format("%d-%m-%Y").to_string(),
        "GSFRB" => nxt_coup_dt.to_string(),
        _ => "NULL".to_string(),
    };
    output_line.push_str(&prev_rep_dt);
    output_line.push('|');

    let coup_freq = val[4].to_string();
    let int_pay_freq: &str = if invst_typ == "ZCORP" || coup_freq == "Monthly" {
        "1"
    } else if coup_freq == "Quarterly" {
        "3"
    } else if coup_freq == "HalfYearly" {
        "6"
    } else if coup_freq == "Yearly" {
        "12"
    } else {
        "0"
    };
    output_line.push_str(int_pay_freq);
    output_line.push('|');

    let int_rt = match invst_typ.as_str() {
        "ZCORP" => &val[26],
        "TBILL" => &val[26],
        "COMMP" => &val[26],
        "ZECBD" => &val[26],
        _ => &val[39],
    };
    output_line.push_str(&int_rt.to_string());
    output_line.push('|');
    output_line.push_str(&as_on_dt.format("%d-%m-%Y").to_string());
    output_line.push('|');
    output_line.push_str(&val[25].to_string());
    output_line.push('\n');
    output_line
}

fn datevalue_to_date(date: String) -> String {
    let date = match date.parse::<f64>() {
        Ok(timestamp) => date_from_timestamp(((timestamp as i64) - 25569) * 86400)
            .format("%d-%m-%Y")
            .to_string(),
        Err(_) => "".to_string(),
    };

    if date != "" {
        let date = NaiveDate::parse_from_str(&date.to_string(), "%d-%m-%Y")
            .expect("Unable to convert date");
        let mut dt_yr = date.year();
        if dt_yr < 2000 {
            dt_yr = 2000 + (dt_yr % 100);
        }
        NaiveDate::from_ymd(dt_yr, date.month(), date.day())
            .format("%d-%m-%Y")
            .to_string()
    } else {
        "".to_string()
    }
}

fn datevalue_to_naive_date(date: String) -> Option<NaiveDate> {
    if let Ok(timestamp) = date.parse::<f64>() {
        Some(date_from_timestamp(((timestamp as i64) - 25569) * 86400))
    } else {
        None
    }
}

fn get_alm_concat(ora_mis1: &str, ora_prod: &str, ora_gl: &str, ora_cat: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_prod);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat.push('_');
    alm_concat.push_str(ora_cat);
    alm_concat
}
