use super::recon::ReconKey;
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

pub fn append_alm_line(
    output_line: &mut String,
    ref_map1: &HashMap<String, String>,
    ref_map2: &HashMap<String, String>,
    cod_gl_regular_dep: &str,
    cost_center: &str,
    mis1: &str,
    ref_mis1: &str,
    log: &Logger,
    acc_num: &str,
) {
    let mut temp_concat: String = "".to_string();
    let alm_concat: String = match ref_map1.get(cod_gl_regular_dep) {
        Some(val) => val.to_string(),
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
        let concat_fields: Vec<&str> = alm_concat.split("_").collect();
        let prod_code: i64 = concat_fields[0]
            .parse()
            .expect("Cannot convert product code to integer.");
        if prod_code == 120415 || prod_code == 120416 || prod_code == 120417 || prod_code == 120418
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

    let mut alm_line: String = match ref_map2.get(&temp_concat[..]) {
        Some(val) => val.to_string(),
        None => {
            log_debug!(log, "Cannot get alm line for {}.", temp_concat);
            "".to_string()
        }
    };
    if alm_line.is_empty() {
        alm_line.push_str("NONE");
    }
    alm_line.push_str("|");
    output_line.push_str(&alm_line[..]);

    log_debug!(
        log,
        "Concat for account no: {} is {}.",
        acc_num,
        temp_concat
    );
    log_debug!(log, "ALM line for account no: {} is {}.", acc_num, alm_line);
}

pub fn append_currency(output_line: &mut String, currency_code: &str) -> String {
    let currency;
    match currency_code {
        "001" => {
            currency = "INR";
        }
        "002" => {
            currency = "USD";
        }
        _ => {
            currency = "FCY";
        }
    }
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
) {
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

pub fn append_int_rate(output_line: &mut String, rat_int: &str, rat_int_var: &str) {
    let val1: f64 = rat_int.parse().unwrap_or(DEFAULT_FLOAT);
    let val2: f64 = rat_int_var.parse().unwrap_or(DEFAULT_FLOAT);
    let total_str = (val1 + val2).to_string();
    output_line.push_str(&total_str);
    output_line.push_str("|");
}
