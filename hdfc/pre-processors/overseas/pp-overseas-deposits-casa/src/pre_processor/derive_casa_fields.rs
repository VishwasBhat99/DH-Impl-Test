use super::remove_comma;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn get_casa_op_line(
    rec: &mut Vec<&str>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, Vec<String>>,
    t_bdp_coa: &mut HashMap<String, String>,
    prod_map: &mut HashMap<String, String>,
    ia_llg_map: &mut HashMap<String, String>,
    balm_llg_map: &mut HashMap<String, String>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    as_on_dt: NaiveDate,
    log: &Logger,
    mf_master_fields: &HashMap<String, String>,
    instance: &str,
) -> Vec<String> {
    let mut ora_mis1 = String::new();
    ora_mis1.push_str(
        &t_ora_gl
            .get(&rec[16].to_string())
            .expect("Could Not Find Value")[2],
    );

    let ora_prod = t_ora_prod
        .entry(rec[5].to_string())
        .or_insert("".to_string());

    let mut ora_gl = "_".to_string();
    let mut ora_gl_code = &"".to_string();
    if t_ora_gl.contains_key(&rec[16].to_string()) {
        ora_gl_code = &t_ora_gl
            .get(&rec[16].to_string())
            .expect("Could Not Find Value")[0];
        ora_gl = t_ora_gl
            .get(&rec[16].to_string())
            .expect("Could Not Find Value")[1]
            .to_owned()
            + "_"
            + &t_ora_gl
                .get(&rec[16].to_string())
                .expect("Could Not Find Value")[0]
                .to_owned();
    }

    let alm_concat = get_alm_concat(&ora_mis1, &ora_gl);
    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert("NONE".to_string());

    let mut coa_concat = String::new();
    coa_concat.push_str(ora_prod);
    coa_concat.push('_');
    coa_concat.push_str(&ora_gl_code);

    let coa = t_bdp_coa
        .entry(coa_concat.to_string())
        .or_insert("".to_string());

    let div = div.entry(ora_mis1.to_string()).or_insert("".to_string());
    let product_code = match prod_map.get(rec[16]) {
        Some(val) => val,
        None => {
            log_error!(log, "Could not get product code for source gl:{}", rec[16]);
            ""
        }
    };

    let ia_llg = match ia_llg_map.get(&alm_concat) {
        Some(val) => val,
        None => {
            log_error!(log, "Could not get ia_llg for alm_concat:{}", alm_concat);
            ""
        }
    };
    let balm_llg = match balm_llg_map.get(&alm_concat) {
        Some(val) => val,
        None => {
            log_error!(log, "Could not get balm_llg for alm_concat:{}", alm_concat);
            ""
        }
    };

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, div: `{}`, product code: `{}`,ia_llg: `{}`, balm_llg: `{}`, coa_concat: `{}`, coa: `{}`.",
        rec[1],
        alm_concat,
        alm_line,
        div,
        product_code,
        ia_llg,
        balm_llg,
        coa_concat,
        coa
    );
    let mut casa = Vec::new();
    casa.push(rec[1].to_string() + "|" + &alm_concat + "|" + alm_line + "\n");
    casa.push(get_line(
        rec,
        as_on_dt,
        alm_line,
        log,
        div.to_string(),
        product_code.to_string(),
        ia_llg.to_string(),
        balm_llg.to_string(),
        alm_concat,
        instance,
        &mf_master_fields,
    ));
    casa
}

fn get_line(
    val: &mut Vec<&str>,
    as_on_dt: NaiveDate,
    alm_line: &mut String,
    log: &Logger,
    div: String,
    product_code: String,
    ia_llg: String,
    balm_llg: String,
    alm_concat: String,
    instance: &str,
    mf_master_fields: &HashMap<String, String>,
) -> String {
    let a_o_dt: String = val[7].to_string();
    let mat_dt: String = val[8].to_string();

    let as_on_dt = as_on_dt.format("%d-%m-%Y").to_string();

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
    output_line.push('|');
    output_line.push_str(&val[16].to_string());
    output_line.push('|');
    let rt_acc_int: f64 = val[9].to_string().parse().unwrap_or(DEFAULT_FLOAT);
    let float_rt: f64 = val[13].to_string().parse().unwrap_or(DEFAULT_FLOAT);
    let int_rt = rt_acc_int + float_rt;
    output_line.push_str(&int_rt.to_string());
    output_line.push('|');
    output_line.push_str(alm_line);
    output_line.push('|');
    output_line.push_str(&ia_llg);
    output_line.push('|');
    output_line.push_str(&balm_llg);
    output_line.push('|');
    output_line.push_str(&remove_comma(val[20].to_string()).to_string());
    output_line.push_str("|100|0|");
    output_line.push_str(&val[38].to_string());
    output_line.push('|');
    output_line.push_str(&val[18].to_string());
    output_line.push('|');
    output_line.push_str(&product_code);
    output_line.push_str("|");
    output_line.push_str(&val[23].to_string());
    output_line.push_str("||");
    output_line.push_str(&a_o_dt);
    output_line.push('|');
    output_line.push_str(&alm_concat);
    output_line.push_str("|||||");
    let input_custno_entity = format!("{}_{}", &val[4].to_string(), instance.to_string());
    output_line.push_str(
        mf_master_fields
            .get(&input_custno_entity)
            .unwrap_or(&"NA".to_string()),
    );
    output_line.push('|');
    output_line.push_str(&a_o_dt);
    output_line.push_str("||");
    output_line.push_str(&div);
    output_line.push_str("||");
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
