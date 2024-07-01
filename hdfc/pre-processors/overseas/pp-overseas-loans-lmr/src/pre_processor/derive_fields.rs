use macros;
use rbdate::{num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut Vec<String>,
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
    ora_mis1.push_str(rec[20].as_str());

    let ora_prod = t_ora_prod
        .entry(rec[11].to_string())
        .or_insert_with(|| "".to_string());

    let ora_gl = t_ora_gl
        .entry(rec[12].to_string())
        .or_insert_with(|| "".to_string());

    let alm_concat = get_alm_concat(&ora_mis1, ora_prod, ora_gl);

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
        rec[1],
        alm_concat,
        alm_line,
        div,
        coa_concat,
        coa
    );
    get_line(rec, as_on_dt, div, alm_line, log)
}

fn get_line(
    val: &mut Vec<String>,
    as_on_dt: NaiveDate,
    div: &mut String,
    alm_line: &mut String,
    log: &Logger,
) -> String {
    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

    let mut output_line = String::new();
    for index in 0..23 {
        output_line.push_str(val[index].as_str());
        output_line.push('|');
    }

    let prod_code = vec!["LN02", "LN03", "LN08", "LN022"];
    let rt_flg = if prod_code.contains(&val[11].as_str()) {
        "V"
    } else if val[23] == "A" {
        "A"
    } else {
        "F"
    };
    output_line.push_str("|||||");
    output_line.push_str(rt_flg);
    output_line.push('|');

    output_line.push_str(val[24].as_str());
    output_line.push('|');
    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push_str("|||");

    let rt_chng_freq: f64 = if let (Ok(st_dt), Ok(end_dt)) = (
        NaiveDate::parse_from_str(val[25].as_str(), "%m-%d-%y"),
        NaiveDate::parse_from_str(val[26].as_str(), "%d-%b-%y"),
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
    output_line.push_str(&rt_chng_freq.to_string());
    output_line.push('|');

    let last_rep_dt = match NaiveDate::parse_from_str(val[25].as_str(), "%d-%b-%Y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(error) => {
            log_error!(
                log,
                "Unable to parse `last_repricing_date: `{}` in `DD-MMM-YYYY` for account: `{}`: {}.",
                val[25],
                val[0],
                error
            );
            "".to_string()
        }
    };

    let nxt_rep_dt = match NaiveDate::parse_from_str(val[26].as_str(), "%d-%b-%Y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(error) => {
            log_error!(
                log,
                "Unable to parse `next_repricing_date: `{}` in `DD-MMM-YYYY` for account: `{}`: {}.",
                val[26],
                val[0],
                error
            );
            "".to_string()
        }
    };
    output_line.push_str(&nxt_rep_dt);
    output_line.push('|');
    output_line.push_str(&last_rep_dt);
    output_line.push('|');
    output_line.push_str(&as_on_dt);
    output_line.push_str("||||||");
    output_line.push_str(&val[27]);
    output_line.push('|');
    output_line.push_str(&val[28]);
    output_line.push('|');
    output_line.push_str(&val[29]);
    output_line.push('|');
    output_line.push_str(&val[30]);
    //takes first 10 chars from utc date, Ex: from 2020-06-30T00:00:00.000+05:30 to 2020-06-30
    let mut int_payout_dt_temp: String = val[31].chars().skip(0).take(10).collect();
    if int_payout_dt_temp == ""{
        int_payout_dt_temp=NaiveDate::parse_from_str(&val[6], "%d-%m-%Y")
        .expect("cannot parse int payout date")
        .format("%Y-%m-%d")
        .to_string();
    }
    let int_payout_dt: String = NaiveDate::parse_from_str(&int_payout_dt_temp, "%Y-%m-%d")
        .expect("cannot parse int payout date")
        .format("%d-%m-%Y")
        .to_string();
    output_line.push('|');
    output_line.push_str(&int_payout_dt);
    output_line.push_str("\n");
    output_line
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
