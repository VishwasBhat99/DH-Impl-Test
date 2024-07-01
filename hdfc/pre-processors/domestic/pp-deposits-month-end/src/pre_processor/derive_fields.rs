use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

pub fn append_as_on_date(output_line: &mut String, as_on_date: NaiveDate) {
    output_line.push_str(&as_on_date.format("%d-%m-%Y").to_string());
    output_line.push_str("|");
}

pub fn append_alm_balm_ia_line(
    output_line: &mut String,
    ref_map1: &HashMap<String, String>,
    alm_llg: &HashMap<String, String>,
    ia_llg: &HashMap<String, String>,
    balm_llg: &HashMap<String, String>,
    gl: &str,
    cost_center: &str,
    mis1: &str,
    ref_mis1: &str,
    log: &Logger,
    acc_num: &str,
) -> String {
    let mut temp_concat: String = "".to_string();
    let alm_concat: String = match ref_map1.get(gl) {
        Some(val) => val.to_string(),
        None => {
            log_debug!(log, "Cannot get alm concat for {}.", gl);
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
            log_debug!(log, "Cannot get alm line for {}.", temp_concat);
            "".to_string()
        }
    };
    let mut balm_line: String = match balm_llg.get(&temp_concat[..]) {
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
        // temp_concat
        get_concat_line(acc_num, gl, &temp_concat)
    } else {
        String::new()
    }
}
fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("DepositsValDt|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}
