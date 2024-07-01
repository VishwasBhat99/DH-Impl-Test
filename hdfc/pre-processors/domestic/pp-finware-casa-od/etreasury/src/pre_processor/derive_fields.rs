use macros;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut Vec<&str>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_ora_cat: &mut HashMap<String, String>,
    cost_center: &mut HashMap<String, String>,
    npa_flg: &mut Vec<String>,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> String {
    let mut t_ora_mis1 = String::new();
    if rec[30].trim() != "" {
        t_ora_mis1.push('1');
        t_ora_mis1.push_str(rec[30]);
    } else {
        t_ora_mis1.push_str("9999");
    }

    let div = div
        .entry(t_ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    let alm_concat = get_alm_concat(
        &t_ora_mis1,
        t_ora_prod
            .entry(rec[16].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_gl
            .entry(rec[16].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_cat
            .entry(rec[16].to_string())
            .or_insert_with(|| "".to_string()),
    );

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let cost_center = cost_center
        .entry(rec[16].to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, division: `{}`.",
        rec[0],
        alm_concat,
        alm_line,
        div
    );

    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();
    get_line(rec, &div, alm_line, cost_center, npa_flg, &as_on_dt, log)
}

fn get_line(
    rec: &mut Vec<&str>,
    div: &str,
    alm_line: &mut String,
    cost_center: &str,
    npa_flg: &mut Vec<String>,
    dt: &str,
    log: &Logger,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(rec[0]);
    output_line.push('|');
    output_line.push_str(rec[1]);
    output_line.push('|');
    output_line.push_str(rec[2]);
    output_line.push('|');
    output_line.push_str(rec[4]);
    output_line.push('|');
    output_line.push_str(rec[5]);
    output_line.push('|');
    output_line.push_str(rec[6]);
    output_line.push('|');
    output_line.push_str(rec[7]);
    output_line.push('|');
    output_line.push_str(rec[8]);
    output_line.push('|');
    output_line.push_str(rec[9]);
    output_line.push('|');

    let acc_open_dt = NaiveDate::parse_from_str(rec[10], "%d-%b-%Y");
    if let Ok(dt) = acc_open_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_error!(
            log,
            "`account_open_date` is not well-formatted for account: `{}`.",
            rec[0]
        );
    }
    output_line.push('|');

    output_line.push_str(rec[11]);
    output_line.push('|');
    output_line.push_str(rec[14]);
    output_line.push('|');

    let acc_close_dt = NaiveDate::parse_from_str(rec[20], "%d-%b-%Y");
    if let Ok(dt) = acc_close_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_debug!(
            log,
            "`account_close_date` is not well-formatted for account: `{}`.",
            rec[0]
        );
    }
    output_line.push('|');

    output_line.push_str(rec[23]);
    output_line.push('|');
    output_line.push_str(rec[24]);
    output_line.push('|');
    output_line.push_str(dt);
    output_line.push('|');
    output_line.push_str(cost_center);
    output_line.push_str("|15");
    output_line.push_str(rec[16]);
    output_line.push_str("00|V|");

    let inst = match rec[3].parse::<i64>().unwrap_or(DEFAULT_INT) {
        1 => "INR",
        2 => "USD",
        _ => "FCY",
    };
    output_line.push_str(inst);
    output_line.push('|');
    output_line.push_str(rec[5]);
    output_line.push('|');
    output_line.push_str(rec[11]);
    output_line.push('|');

    let int_rt: f64 = if rec[19].parse::<f64>().unwrap_or(DEFAULT_FLOAT) == 0.0 {
        rec[18].parse().unwrap_or(DEFAULT_FLOAT)
    } else {
        rec[19].parse().unwrap_or(DEFAULT_FLOAT)
    };
    output_line.push_str(&int_rt.to_string());
    output_line.push('|');

    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(&alm_line);
    output_line.push('|');
    output_line.push_str(rec[30]);
    output_line.push('|');

    if npa_flg.contains(&rec[0].to_string()) {
        output_line.push_str("Y");
    } else {
        output_line.push_str("N");
    }
    output_line.push_str("|||||\n");
    output_line
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
