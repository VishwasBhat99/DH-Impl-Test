use super::output_lines::OutputLines;
use chrono::NaiveDate;
use macros;
use slog::Logger;
use std::collections::{HashMap, HashSet};

pub fn get_op_line(
    rec: &mut [&str],
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    ia_llg: &mut HashMap<String, String>,
    balm_llg: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_ora_cat: &mut HashMap<String, String>,
    asset_class: &mut HashMap<String, String>,
    weaker_section: &mut HashSet<String>,
    ews_map: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
    sma_data: &HashMap<String, String>,
) -> OutputLines {
    let mut output_lines = OutputLines::new();
    let dt = as_on_dt.format("%d-%m-%Y");

    let mut c_m_1 = rec[32].parse::<i64>().unwrap_or(888);
    if c_m_1 == 0 {
        c_m_1 = 888;
    }

    let mut c_m_2 = rec[33].parse::<i64>().unwrap_or(100);
    if c_m_2 == 0 {
        c_m_2 = 100;
    }

    let mut t_ora_mis1: String = String::new();
    t_ora_mis1.push('1');
    t_ora_mis1.push_str(&c_m_1.to_string());

    let alm_concat = get_alm_concat(
        &t_ora_mis1,
        t_ora_prod
            .entry(rec[10].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_gl
            .entry(rec[21].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_cat
            .entry(rec[21].to_string())
            .or_insert_with(|| "".to_string()),
    );

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let ia_llg = ia_llg
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let balm_llg = balm_llg
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let div = div.entry(t_ora_mis1).or_insert_with(|| "".to_string());

    let asset_class = asset_class
        .entry(rec[4].to_string())
        .or_insert_with(|| "P".to_string());

    let is_acc_weaker = if weaker_section.contains(&rec[4].to_string()) {
        "Y"
    } else {
        "N"
    };
    let ews_weaker_value = if (match ews_map.get(&rec[4].to_string()) {
        Some(val) => val,
        None => "Others",
    }) == "SEK_WK"
    {
        "SEK-WEAKER"
    } else {
        "Others"
    };
    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}` division: `{}`, asset_class: `{}`.",
        rec[4],
        alm_concat,
        alm_line,
        div,
        asset_class
    );
    output_lines.processed_lines = get_line(
        rec,
        div,
        alm_line,
        ia_llg,
        balm_llg,
        &dt.to_string(),
        log,
        c_m_1,
        c_m_2,
        asset_class,
        &alm_concat,
        is_acc_weaker,
        ews_weaker_value,
        sma_data,
    );

    if alm_line == "NONE" {
        output_lines
            .concat_lines
            .push(get_concat_line(rec[4], rec[21], &alm_concat));
    }

    output_lines
}

fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("UBSBills|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

fn get_line(
    rec: &mut [&str],
    div: &mut str,
    alm_line: &mut str,
    ia_llg: &mut str,
    balm_llg: &mut str,
    dt: &str,
    log: &Logger,
    comp_mis1: i64,
    comp_mis2: i64,
    asset_class: &str,
    concat: &str,
    is_acc_weaker: &str,
    ews_weaker_value: &str,
    sma_data: &HashMap<String, String>,
) -> String {
    let b_o_a_l: f64 = get_f64_val(rec[14], rec[4], "bill_os_amt_lcy", log);

    let b_o_a: f64 = get_f64_val(rec[13], rec[4], "bill_os_amt", log);
    let mut ex_rt: f64 = 1.0;
    if b_o_a == 0.0 {
        log_debug!(
            log,
            "`bill_ost_amt` is zero for account : `{}` and exchange rate : `{}`.",
            rec[4],
            ex_rt
        );
    } else {
        ex_rt = b_o_a_l / b_o_a;
    }

    log_debug!(
        log,
        "`exchange rate` for account: `{}` is `{}`",
        rec[4],
        ex_rt
    );

    let bill_amt = get_f64_val(rec[12], rec[4], "bill_amt", log);
    let l_a = bill_amt * ex_rt;

    let mut output_line = String::new();
    output_line.push_str(rec[4]);
    output_line.push('|');
    output_line.push_str(rec[2]);
    output_line.push('|');
    output_line.push_str(rec[7]);
    output_line.push('|');

    output_line.push_str(&get_date(rec[16], "value_date", rec[4], log));
    output_line.push('|');

    output_line.push_str(&get_date(rec[18], "maturity_date", rec[4], log));
    output_line.push('|');

    output_line.push_str(&get_date(rec[17], "bkgvldt", rec[4], log));
    output_line.push('|');

    output_line.push_str(rec[19]);
    output_line.push('|');
    output_line.push_str(rec[20]);
    output_line.push('|');
    output_line.push_str(&get_date(rec[23], "closure_date", rec[4], log));
    output_line.push('|');
    output_line.push_str(&get_date(rec[25], "due_dt_principal", rec[4], log));
    output_line.push('|');
    output_line.push_str(rec[13]);
    output_line.push('|');
    output_line.push_str(&l_a.to_string());
    output_line.push('|');
    output_line.push_str(rec[21]);
    output_line.push('|');
    output_line.push_str(rec[29]);
    output_line.push('|');
    output_line.push_str(rec[1]);
    output_line.push('|');
    output_line.push_str(&comp_mis1.to_string());
    output_line.push('|');
    output_line.push_str(&comp_mis2.to_string());
    output_line.push('|');
    output_line.push_str(rec[10]);
    output_line.push('|');
    if rec[5].len() < 4 {
        output_line.push_str(rec[5]);
    } else {
        output_line.push_str(&rec[5][0..3]);
    }
    output_line.push_str("|F|");
    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push('|');
    output_line.push_str(ia_llg);
    output_line.push('|');
    output_line.push_str(balm_llg);
    output_line.push('|');
    output_line.push_str(dt);
    output_line.push('|');
    output_line.push_str(&ex_rt.to_string());
    output_line.push('|');
    output_line.push_str(asset_class);
    output_line.push('|');
    output_line.push_str(&get_date(rec[37], "value_date", rec[4], log));
    output_line.push('|');
    output_line.push_str(&b_o_a_l.to_string());
    output_line.push('|');
    output_line.push_str(rec[12]);
    output_line.push('|');
    output_line.push_str(concat);
    output_line.push('|');
    output_line.push_str(rec[26]);
    output_line.push('|');
    output_line.push_str(rec[34]);
    output_line.push('|');
    output_line.push_str(is_acc_weaker);
    output_line.push('|');
    output_line.push_str(ews_weaker_value);
    output_line.push('|');
    output_line.push_str(
        &sma_data
            .get(&rec[4].to_string())
            .unwrap_or(&"P".to_string())
            .to_string(),
    );
    output_line.push('\n');
    output_line
}

fn get_date(date: &str, field_name: &str, acc_no: &str, log: &Logger) -> String {
    match NaiveDate::parse_from_str(date, "%d-%b-%Y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(_) => {
            log_debug!(
                log,
                "`{}`: `{}` is not well-formatted as `DD-MMM-YY` for account: `{}`.",
                field_name,
                date,
                acc_no
            );
            "".to_string()
        }
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

fn get_f64_val(val: &str, acc: &str, field_name: &str, log: &Logger) -> f64 {
    let mut val_f64: f64 = 1.00;
    if let Ok(val) = val.parse::<f64>() {
        val_f64 = val;
    } else {
        log_error!(
            log,
            "`{}` not well formatted for account : `{}`.",
            field_name,
            acc
        );
    }
    val_f64
}
