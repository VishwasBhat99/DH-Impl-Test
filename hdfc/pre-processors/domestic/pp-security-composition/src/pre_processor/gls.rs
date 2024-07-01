use super::InputAccount;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Gls {
    pub fv_gl: String,
    pub prem_gl: String,
    pub prem_amt_field: String,
    pub prod_concat: String,
}

impl Gls {
    pub fn new() -> Gls {
        Gls {
            ..Default::default()
        }
    }
}

pub fn get_gl(
    acc: &InputAccount,
    fv_gl: &mut HashMap<String, String>,
    prem_gl: &mut HashMap<String, String>,
    prem_amt: &mut HashMap<String, String>,
) -> Gls {
    let mut concat = String::new();
    concat.push_str(&acc.entity);
    concat.push('_');
    concat.push_str(&acc.prod);
    concat.push('_');
    concat.push_str(&acc.prod_desc);
    concat.push('_');
    concat.push_str(&acc.desk);
    concat.push('_');
    concat.push_str(&acc.acc_sec_igaap);

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
        prod_concat: concat,
    }
}
