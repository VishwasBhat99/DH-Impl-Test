use std::collections::HashSet;

use super::*;

pub fn get_op_line(
    casatd_fields: CASATDFields,
    adv_client_ids: &HashMap<String, bool>,
    ops_client_ids: &HashMap<String, bool>,
    mult_depo_cust_ids: &HashMap<String, bool>,
    const_desc_values: &HashMap<String, bool>,
    prod_code_values: &HashMap<String, bool>,
    mult_custid: &HashMap<String, String>,
    written_custid: &HashSet<String>,
) -> String {
    let mut op_line = String::new();
    let mut biu_fields = BIUFields::new();

    biu_fields.t1 = casatd_fields.prd_cd.to_string();
    biu_fields.acc_no = casatd_fields.cust_id.to_string();

    biu_fields.t2 = if match adv_client_ids.get(&casatd_fields.cust_id.trim().to_string()) {
        Some(value) => *value,
        None => false,
    } {
        String::from("Y")
    } else {
        biu_fields.t2
    };
    biu_fields.t3 = if match mult_depo_cust_ids.get(&casatd_fields.cust_id.trim().to_string()) {
        Some(value) => *value,
        None => false,
    } {
        String::from("Y")
    } else {
        biu_fields.t3
    };

    biu_fields.t4 = if match ops_client_ids.get(&casatd_fields.cust_id.trim().to_string()) {
        Some(value) => *value,
        None => false,
    } {
        String::from("Y")
    } else {
        biu_fields.t4
    };

    op_line.push_str(&biu_fields.print());

    op_line
}

pub fn get_op_line_hash_map_output(
    cust_id: &str,
    prod: &str,
    adv_client_ids: &HashMap<String, bool>,
    ops_client_ids: &HashMap<String, bool>,
    mult_depo_cust_ids: &HashMap<String, bool>,
) -> String {
    let mut op_line = String::new();
    let mut biu_fields = BIUFields::new();

    biu_fields.t1 = prod.to_string();

    biu_fields.acc_no = cust_id.to_string();

    biu_fields.t2 = if match adv_client_ids.get(&cust_id.trim().to_string()) {
        Some(value) => *value,
        None => false,
    } {
        String::from("Y")
    } else {
        biu_fields.t2
    };
    biu_fields.t3 = if match mult_depo_cust_ids.get(&cust_id.trim().to_string()) {
        Some(value) => *value,
        None => false,
    } {
        String::from("Y")
    } else {
        biu_fields.t3
    };

    biu_fields.t4 = if match ops_client_ids.get(&cust_id.trim().to_string()) {
        Some(value) => *value,
        None => false,
    } {
        String::from("Y")
    } else {
        biu_fields.t4
    };

    op_line.push_str(&biu_fields.print());

    op_line
}
