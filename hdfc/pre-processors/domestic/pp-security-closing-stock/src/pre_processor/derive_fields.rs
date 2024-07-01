use super::InputAccount;
use super::MasterLLGFields;
use rbdate::NaiveDate;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Gls {
    pub fv_gl: String,
    pub prem_gl: String,
    pub prem_amt_field: String,
    pub product_concat: String,
}

pub fn get_op_line(
    acc: &InputAccount,
    as_on_dt: NaiveDate,
    master_llg_value: &MasterLLGFields,
    alm_concat: &str,
    product_concat: &str,
    concat_line: &mut String,
    gl: &str,
) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");
    get_line(
        acc,
        &dt.to_string(),
        &master_llg_value,
        alm_concat,
        product_concat,
        concat_line,
        gl,
    )
}

fn get_line(
    acc: &InputAccount,
    dt: &str,
    master_llg_value: &MasterLLGFields,
    alm_concat: &str,
    product_concat: &str,
    concat_line: &mut String,
    gl: &str,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&product_concat);
    op_line.push('|');
    op_line.push_str(&acc.to_string());
    op_line.push('|');
    op_line.push_str(&dt);
    op_line.push('|');
    op_line.push_str(&alm_concat.to_string());
    op_line.push('|');
    if !master_llg_value.alm_line.is_empty() {
        op_line.push_str(&master_llg_value.alm_line.to_string());
    } else {
        op_line.push_str("NONE");
        concat_line.push_str(&get_concat_line(product_concat, gl, alm_concat));
        concat_line.push_str("\n");
    }
    op_line.push('|');
    if !master_llg_value.ia_line.is_empty() {
        op_line.push_str(&master_llg_value.ia_line.to_string());
    } else {
        op_line.push_str("NONE");
    }
    op_line.push('\n');
    op_line
}

pub fn get_gl(
    acc: &InputAccount,
    fv_gl: &mut HashMap<String, String>,
    prem_gl: &mut HashMap<String, String>,
    prem_amt: &mut HashMap<String, String>,
) -> Gls {
    let mut concat = String::new();
    concat.push_str(&acc.branch_entity);
    concat.push('_');
    concat.push_str(&acc.category);
    concat.push('_');
    concat.push_str(&acc.security_type);
    concat.push('_');
    concat.push_str(&acc.desk);
    concat.push('_');
    concat.push_str(&acc.portfolio_type);

    let fv_gl = fv_gl
        .entry(concat.to_string())
        .or_insert_with(|| "".to_string());
    let prem_gl = prem_gl
        .entry(concat.to_string())
        .or_insert_with(|| "".to_string());
    let prem_amt_field = prem_amt
        .entry(concat.to_string())
        .or_insert_with(|| "".to_string());

    Gls {
        fv_gl: fv_gl.to_string(),
        prem_gl: prem_gl.to_string(),
        prem_amt_field: prem_amt_field.replace(" ", ""),
        product_concat: concat.to_string(),
    }
}

pub fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("MurexSecCloseStock|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}
