use super::remove_comma;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn get_op_line(
    rec: &mut Vec<String>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    ia_llg: &mut HashMap<String, String>,
    balm_llg: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_ora_cat: &mut HashMap<String, String>,
    npa_flg: &mut Vec<String>,
    int_idx_name: &mut HashMap<String, String>,
    log: &Logger,
) -> String {
    let mut t_ora_mis1: String = String::new();
    t_ora_mis1.push('1');
    t_ora_mis1.push_str(&rec[17]);

    let div = div
        .entry(t_ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    let book_bal = rec[5].parse().unwrap_or(DEFAULT_FLOAT);
    let gl_acc_no = if book_bal >= 0.0 { &rec[15] } else { &rec[13] };

    let alm_concat = get_alm_concat(
        &t_ora_mis1,
        t_ora_prod
            .entry(rec[0].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_gl
            .entry(gl_acc_no.to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_cat
            .entry(gl_acc_no.to_string())
            .or_insert_with(|| "".to_string()),
    );

    let idx_name = int_idx_name
        .entry(rec[23].to_string())
        .or_insert_with(|| "".to_string());

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let ia_llg = ia_llg
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let balm_llg = balm_llg
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, division: `{}`.",
        rec[1],
        alm_concat,
        alm_line,
        div
    );

    get_line(
        rec, &div, alm_line, ia_llg, balm_llg, &idx_name, npa_flg, &log,
    )
}

fn get_line(
    rec: &mut Vec<String>,
    div: &str,
    alm_line: &mut String,
    ia_llg: &mut String,
    balm_llg: &mut String,
    idx_name: &str,
    npa_flg: &mut Vec<String>,
    log: &Logger,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(&rec[1]);
    output_line.push('|');
    output_line.push_str(&rec[0]);
    output_line.push('|');
    output_line.push_str(&rec[2]);
    output_line.push('|');
    output_line.push_str(&rec[4]);
    output_line.push('|');
    output_line.push_str(&remove_comma(&rec[5]));
    output_line.push('|');
    output_line.push_str(&remove_comma(&rec[6]));
    output_line.push('|');
    output_line.push_str(&rec[7]);
    output_line.push('|');
    output_line.push_str(&remove_comma(&rec[8]));
    output_line.push('|');

    let acc_open_dt = NaiveDate::parse_from_str(&rec[9], "%d-%b-%y");
    if let Ok(dt) = acc_open_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_error!(
            log,
            "`account_open_date` is not well-formatted for account: `{}`.",
            rec[1]
        );
    }
    output_line.push('|');

    output_line.push_str(&rec[10]);
    output_line.push('|');
    output_line.push_str(&rec[13]);
    output_line.push('|');
    output_line.push_str(&rec[15]);
    output_line.push_str("|NULL|");

    let cbr_num_1 = rec[17].parse().unwrap_or(DEFAULT_INT);
    if cbr_num_1 == 0 {
        output_line.push_str("888");
    } else {
        output_line.push_str(&cbr_num_1.to_string());
    }
    output_line.push('|');

    let cbr_num_2 = rec[18].parse().unwrap_or(DEFAULT_INT);
    if cbr_num_2 == 0 {
        output_line.push_str("100");
    } else {
        output_line.push_str(&cbr_num_2.to_string());
    }
    output_line.push('|');

    output_line.push_str(&rec[19]);
    output_line.push('|');

    let cr_int_var = rec[11].parse().unwrap_or(DEFAULT_FLOAT);
    let min_int_var = rec[21].parse().unwrap_or(DEFAULT_FLOAT);
    let max_int_var = rec[22].parse().unwrap_or(DEFAULT_FLOAT);
    let idx_rt = rec[20].parse().unwrap_or(DEFAULT_FLOAT);
    let int_var = if cr_int_var < min_int_var {
        min_int_var
    } else if cr_int_var > max_int_var {
        max_int_var
    } else {
        cr_int_var
    };
    let rt = idx_rt + int_var;
    output_line.push_str(&rt.to_string());
    output_line.push('|');

    let rt_int_eff = rec[14].parse().unwrap_or(DEFAULT_FLOAT);
    let dr_rt = if rt_int_eff == 0.0 { rt } else { rt_int_eff };
    output_line.push_str(&dr_rt.to_string());
    output_line.push('|');

    let book_bal = rec[5].parse().unwrap_or(DEFAULT_FLOAT);
    let act_typ = if book_bal >= 0.0 { 2 } else { 1 };
    output_line.push_str(&act_typ.to_string());
    output_line.push('|');

    output_line.push_str(&rec[16]);
    output_line.push_str("|0|");

    let int_idx_cd = rec[23].parse().unwrap_or(DEFAULT_INT);
    let cds: Vec<i64> = vec![61, 62, 63, 64, 65, 66, 9015, 9023];

    if !cds.contains(&int_idx_cd) {
        output_line.push_str("UBSDD|F|");
    } else {
        output_line.push_str("UBSOD|F|");
    }

    match rec[3].as_str() {
        "1" => output_line.push_str("INR|"),
        _ => output_line.push_str("FCY|"),
    };

    output_line.push_str(&remove_comma(&rec[5]));
    output_line.push_str("|A|");
    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(&alm_line);
    output_line.push('|');
    output_line.push_str(&ia_llg);
    output_line.push('|');
    output_line.push_str(&balm_llg);
    output_line.push('|');
    output_line.push_str(&rec[23]);
    output_line.push('|');
    output_line.push_str(idx_name);
    output_line.push('|');
    output_line.push_str(&rec[24]);
    output_line.push('|');

    if npa_flg.contains(&rec[1].to_string()) {
        output_line.push_str("Y|");
    } else {
        output_line.push_str("N|");
    }

    let gl = if book_bal >= 0.0 { &rec[15] } else { &rec[13] };
    output_line.push_str(gl);
    output_line.push('\n');

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
