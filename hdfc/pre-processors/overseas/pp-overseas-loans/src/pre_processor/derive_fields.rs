use macros;
use rbdate::{num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut [String],
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, Vec<String>>,
    t_bdp_coa: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
    sma_data: &HashMap<String, String>
) -> Vec<String> {
    let mut ora_mis1 = String::new();
    ora_mis1.push('1');
    ora_mis1.push_str(rec[20].as_str());

    let ora_prod = t_ora_prod
        .entry(rec[11].to_string())
        .or_insert_with(|| "".to_string());

    let mut ora_gl = "_".to_string();
    let mut ora_gl_code = &"".to_string();
    if t_ora_gl.contains_key(&rec[12].to_string()) {
        ora_gl_code = &t_ora_gl
            .get(&rec[12].to_string())
            .expect("Could Not Find Value")[0];
        ora_gl = t_ora_gl
            .get(&rec[12].to_string())
            .expect("Could Not Find Value")[1]
            .to_owned()
            + "_"
            + &t_ora_gl
                .get(&rec[12].to_string())
                .expect("Could Not Find Value")[0]
                .to_owned();
    }

    let alm_concat = get_alm_concat(&ora_mis1, &ora_gl);

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

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
        rec[1],
        alm_concat,
        alm_line,
        div,
        coa_concat,
        coa
    );
    let mut loans = vec![rec[1].to_string() + "|" + &alm_concat + "|" + alm_line + "\n"];
    loans.push(get_line(rec, as_on_dt, div, alm_line, log, sma_data));
    loans
}

fn get_line(
    val: &mut [String],
    as_on_dt: NaiveDate,
    div: &mut str,
    alm_line: &mut str,
    log: &Logger,
    sma_data: &HashMap<String, String>
) -> String {
    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

    let mut output_line = String::new();
    for index in 0..23 {
        output_line.push_str(val[index].as_str());
        output_line.push('|');
    }

    let prod_code = ["LN02", "LN03", "LN08", "LN022"];
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
    output_line.push_str("||");
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

    let last_rep_dt = get_date(val, 25_usize, "last_repricing_date", log);

    let nxt_rep_dt = get_date(val, 26_usize, "next_repricing_date", log);
    output_line.push_str(&nxt_rep_dt);
    output_line.push('|');
    output_line.push_str(&last_rep_dt);
    output_line.push('|');
    output_line.push_str(&as_on_dt);
    output_line.push_str("|||");
    output_line.push_str(val[27].as_str());
    output_line.push_str("|||||||||||||");
    output_line.push_str(&sma_data.get(&val[4].to_string()).unwrap_or(&"P".to_string()).to_string());
    output_line.push('\n');
    output_line
}

fn get_alm_concat(ora_mis1: &str, ora_gl: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat
}

fn get_date(val: &[String], row_num: usize, name: &str, log: &Logger) -> String {
    let error_msg = format!(
        "Unable to parse `{}: `{}` in `DD-MMM-YYYY` for account: `{}`:",
        name, val[row_num], val[0]
    );
    let date = match NaiveDate::parse_from_str(val[row_num].as_str(), "%d-%b-%y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(_error) => {
            let date = match NaiveDate::parse_from_str(val[row_num].as_str(), "%d-%b-%Y") {
                Ok(dtt) => dtt.format("%d-%m-%Y").to_string(),
                Err(_error) => {
                    let date = match NaiveDate::parse_from_str(val[row_num].as_str(), "%d-%m-%y") {
                        Ok(dtt) => dtt.format("%d-%m-%Y").to_string(),
                        Err(_error) => {
                            let date = match NaiveDate::parse_from_str(
                                val[row_num].as_str(),
                                "%d-%m-%Y",
                            ) {
                                Ok(dtt) => dtt.format("%d-%m-%Y").to_string(),
                                Err(_error) => {
                                    let date = match NaiveDate::parse_from_str(
                                        val[row_num].as_str(),
                                        "%+",
                                    ) {
                                        Ok(dtt) => dtt.format("%d-%m-%Y").to_string(),
                                        Err(error) => {
                                            log_error!(log, "{}:`{}`", error_msg, error);
                                            "".to_string()
                                        }
                                    };
                                    date
                                }
                            };
                            date
                        }
                    };
                    date
                }
            };
            date
        }
    };
    date
}
