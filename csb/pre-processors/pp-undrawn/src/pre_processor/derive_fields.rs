use super::{CustMasterData, CustMasterInput, CustMasterMap, ExtraFieldData, InputAccount};
use chrono::NaiveDate;
use std::collections::HashMap;
pub fn get_op_line(
    acc: &mut InputAccount,
    cust_master: &mut CustMasterMap,
    extra_field_map: &HashMap<String, ExtraFieldData>,
    ltv_map: &HashMap<String, String>,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());
    let def_cust_master_data = CustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.client_id.to_string())
        .or_insert(def_cust_master_data);
    op_line.push_str(&cust_master_data.print());

    let ccod_undrawn_lcr = acc.ccod_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.tl_ualimit.parse::<f64>().unwrap_or(0.0);
    let ccod_und_nsfr = acc.ccod_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.tl_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.pbg_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.fbg_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.loc_ualimit.parse::<f64>().unwrap_or(0.0);
    let care_funded = acc.pbg_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.fbg_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.loc_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.bliab_bill_ualimit.parse::<f64>().unwrap_or(0.0);
    let care_lcbg = acc.pbg_blnc.parse::<f64>().unwrap_or(0.0)
        + acc.fbg_blnc.parse::<f64>().unwrap_or(0.0)
        + acc.bliab_bill_blnc.parse::<f64>().unwrap_or(0.0);
    op_line.push_str(&ccod_undrawn_lcr.to_string());
    op_line.push('|');
    op_line.push_str(&ccod_und_nsfr.to_string());
    op_line.push('|');
    op_line.push_str(&care_funded.to_string());
    op_line.push('|');
    op_line.push_str(&care_lcbg.to_string());
    let def_additional_data = ExtraFieldData {
        ..Default::default()
    };
    let additional_data = extra_field_map
        .get(&acc.acc_no)
        .unwrap_or(&def_additional_data);
    op_line.push_str(&additional_data.print());
    let def_ltv_data = format!("1000.0");
    let ltv_data = ltv_map.get(&acc.acc_no).unwrap_or(&def_ltv_data);
    op_line.push_str(&ltv_data);

    let gl: i64;
    if acc.acc_no != "" {
        let acc_info: Vec<&str> = acc.acc_no.split('-').collect();
        gl = acc_info[acc_info.len() - 1][0..4].parse().unwrap_or(0);
    } else {
        gl = 0;
    }

    let total_care_bal = acc.ccod_ualimit.parse::<f64>().unwrap_or(0.0)
        + acc.tl_ualimit.parse::<f64>().unwrap_or(0.0).abs()
        + acc.pbg_ualimit.parse::<f64>().unwrap_or(0.0).abs()
        + acc.fbg_ualimit.parse::<f64>().unwrap_or(0.0).abs()
        + acc.loc_ualimit.parse::<f64>().unwrap_or(0.0).abs()
        + acc.bliab_bill_ualimit.parse::<f64>().unwrap_or(0.0).abs()
        + acc.pbg_blnc.parse::<f64>().unwrap_or(0.0).abs()
        + acc.fbg_blnc.parse::<f64>().unwrap_or(0.0).abs()
        + acc.loc_blnc.parse::<f64>().unwrap_or(0.0).abs()
        + acc.bliab_bill_blnc.parse::<f64>().unwrap_or(0.0).abs();

    // 15 additional passthrough for future purpose
    // 5 Int, 5 Float, 5 String
    op_line.push('|');
    // Int 1 is gl code
    op_line.push_str(&gl.to_string());
    op_line.push_str("|||||");
    op_line.push_str(&total_care_bal.to_string());
    op_line.push_str("|||||");
    op_line.push_str(&cust_master_data.clients_group_code);
    op_line.push_str("|||||");
    let maturity_dt = NaiveDate::parse_from_str(&acc.maturity_dt, "%d-%m-%Y")
        .unwrap_or(NaiveDate::from_ymd(1970, 1, 1))
        .format("%d-%m-%Y")
        .to_string();
    op_line.push_str(&maturity_dt.to_string());
    op_line.push('\n');
    op_line
}

pub fn get_cust_master_data(
    cust_master_input: CustMasterInput,
    cust_master_map: &mut CustMasterMap,
) {
    let mut cust_master_data = CustMasterData::new();
    cust_master_data.insert(cust_master_input.clone());
    cust_master_map
        .store
        .insert(cust_master_input.clients_code, cust_master_data);
}
