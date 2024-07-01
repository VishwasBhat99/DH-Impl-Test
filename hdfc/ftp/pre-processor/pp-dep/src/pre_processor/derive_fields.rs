use super::recon::ReconKey;
use super::ConcatFields;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn cost_center(
    ref_map3: &HashMap<String, String>,
    cod_gl_regular_dep: &str,
    cost_center_ftp: &str,
) -> String {
    let val_of_gl: i64 = cod_gl_regular_dep.parse().unwrap_or(DEFAULT_INT);
    let val_cost_center_ftp: i64 = cost_center_ftp.parse().unwrap_or(DEFAULT_INT);
    if val_of_gl == 22478
        || val_of_gl == 22480
        || val_of_gl == 22560
        || val_of_gl == 22561
        || val_of_gl == 22562
        || val_of_gl == 23180
    {
        if val_cost_center_ftp > 100 {
            cost_center_ftp.to_string()
        } else {
            "700".to_string()
        }
    } else {
        let cost_center: String = match ref_map3.get(cod_gl_regular_dep) {
            Some(val) => val.to_string(),
            None => "999".to_string(),
        };
        cost_center
    }
}

pub fn append_as_on_date(output_line: &mut String, as_on_date: NaiveDate) {
    output_line.push_str(&as_on_date.format("%d-%m-%Y").to_string());
    output_line.push_str("|");
}

pub fn append_alm_balm_ia_line(
    output_line: &mut String,
    ref_map1: &HashMap<String, ConcatFields>,
    alm_llg: &HashMap<String, String>,
    ia_llg: &HashMap<String, String>,
    balm_llg: &HashMap<String, String>,
    cod_gl_regular_dep: &str,
    cost_center: &str,
    mis1: &str,
    ref_mis1: &str,
    log: &Logger,
    acc_num: &str,
    gl: &str,
) -> String {
    let mut temp_concat: String = "".to_string();
    let alm_concat: String = match ref_map1.get(cod_gl_regular_dep) {
        Some(val) => val.alm_concat.to_string(),
        None => {
            log_debug!(log, "Cannot get alm concat for {}.", cod_gl_regular_dep);
            "".to_string()
        }
    };
    if mis1 != "" {
        temp_concat.push_str("1");
        temp_concat.push_str(&mis1);
        temp_concat.push_str("_");
        temp_concat.push_str(&alm_concat);
    } else {
        let concat_fields: Vec<&str> = alm_concat.split('_').collect();
        let prod_code: i64 = concat_fields[0]
            .parse()
            .expect("Cannot convert product code to integer.");
        if prod_code == 120_415
            || prod_code == 120_416
            || prod_code == 120_417
            || prod_code == 120_418
        {
            temp_concat.push_str("1");
            temp_concat.push_str(&cost_center);
            temp_concat.push_str("_");
            temp_concat.push_str(&alm_concat);
        } else {
            temp_concat.push_str(&ref_mis1);
            temp_concat.push_str("_");
            temp_concat.push_str(&alm_concat);
        }
    }

    let mut alm_line: String = match alm_llg.get(&temp_concat[..]) {
        Some(val) => val.to_string(),
        None => {
            log_debug!(log, "Cannot get alm line for {}.", temp_concat);
            "".to_string()
        }
    };
    let mut ia_line: String = match ia_llg.get(&temp_concat[..]) {
        Some(val) => val.to_string(),
        None => {
            log_debug!(log, "Cannot get ia line for {}.", temp_concat);
            "".to_string()
        }
    };
    let mut balm_line: String = match balm_llg.get(&temp_concat[..]) {
        Some(val) => val.to_string(),
        None => {
            log_debug!(log, "Cannot get balm line for {}.", temp_concat);
            "".to_string()
        }
    };
    if alm_line.is_empty() {
        alm_line.push_str("NONE");
    }
    alm_line.push_str("|");
    output_line.push_str(&alm_line[..]);

    if ia_line.is_empty() {
        ia_line.push_str("NONE");
    }
    ia_line.push_str("|");
    output_line.push_str(&ia_line[..]);

    if balm_line.is_empty() {
        balm_line.push_str("NONE");
    }
    balm_line.push_str("|");
    output_line.push_str(&balm_line[..]);

    log_debug!(
        log,
        "Concat for account no: {} is {}.",
        acc_num,
        temp_concat
    );
    log_debug!(
        log,
        "ALM line, IA Line and Balm Line for account no: {} are {}, {} and {} respectively.",
        acc_num,
        alm_line,
        ia_line,
        balm_line
    );

    if alm_line == "NONE|" {
        get_concat_line(acc_num, gl, &alm_concat)
    } else {
        String::new()
    }
}

fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("EDWDeposits|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

pub fn append_currency(
    output_line: &mut String,
    currency_code: &str,
    ccy_codes: &mut HashMap<String, String>,
) -> String {
    let currency = ccy_codes
        .entry(currency_code.to_string())
        .or_insert("OTH".to_string());
    output_line.push_str(currency);
    output_line.push_str("|");
    currency.to_string()
}

pub fn append_current_book_balance(
    output_line: &mut String,
    bal_prin: &str,
    bal_ont_comp: &str,
    cip_recon_key: ReconKey,
    cod_recon_key: ReconKey,
    rec_map: &mut HashMap<ReconKey, f64>,
) -> f64 {
    let val1: f64 = bal_prin.parse().unwrap_or(DEFAULT_FLOAT);
    let val2: f64 = bal_ont_comp.parse().unwrap_or(DEFAULT_FLOAT);
    let total_str = (val1 + val2).to_string();
    output_line.push_str(&total_str);
    output_line.push_str("|");
    rec_map
        .entry(cod_recon_key)
        .and_modify(|amt| *amt += val1)
        .or_insert(val1);
    rec_map
        .entry(cip_recon_key)
        .and_modify(|amt| *amt += val2)
        .or_insert(val2);
    val1 + val2
}

pub fn append_gl_acc(output_line: &mut String, cod_gl_regular_dep: &str) {
    let mut final_gl: String = "".to_string();
    if cod_gl_regular_dep.len() == 5 {
        final_gl.push_str("25");
        final_gl.push_str(cod_gl_regular_dep);
        final_gl.push_str("00");
    } else {
        final_gl.push_str(cod_gl_regular_dep);
    }
    output_line.push_str(&final_gl);
    output_line.push_str("|");
}

pub fn append_int_rate(
    output_line: &mut String,
    rat_int: &str,
    rat_int_var: &str,
    rat_prod_var: &str,
) {
    let val1: f64 = rat_int.parse().unwrap_or(DEFAULT_FLOAT);
    let val2: f64 = rat_int_var.parse().unwrap_or(DEFAULT_FLOAT);
    let val3: f64 = rat_prod_var.parse().unwrap_or(DEFAULT_FLOAT);
    let total_str = (val1 + val2 + val3).to_string();
    output_line.push_str(&total_str);
    output_line.push_str("|");
}
