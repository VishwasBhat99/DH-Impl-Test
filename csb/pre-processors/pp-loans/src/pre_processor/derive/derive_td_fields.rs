use super::{
    AdditionalPassThroughs, AlmMaster, AlmMasterKey, InputAccount, NPAData, NPAMap,
    TdCrCustMasterData, TdCrCustMasterMap, DEFAULT_FLOAT,
};
pub use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;

pub fn get_td_op_line(
    acc: &mut InputAccount,
    cust_master: &mut TdCrCustMasterMap,
    npa: &mut NPAMap,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    concats: &mut String,
    additional_passthroughs: &AdditionalPassThroughs,
    as_on_date: NaiveDate,
    loan_additional_map: &HashMap<String, String>,
    loan_additional_map_org: &HashMap<String, String>,
) -> String {
    let mut op_line = String::new();
    acc.last_paid_emi_dt = as_on_date.format("%d-%m-%Y").to_string();
    op_line.push_str(&acc.print_td());

    let def_cust_master_data = TdCrCustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.acnts_client_num.to_string())
        .or_insert(def_cust_master_data);
    op_line.push_str(&cust_master_data.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.bal.parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("C"));
    };
    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());

    let def_npa_data = NPAData::new();
    let npa_data = npa
        .store
        .entry(acc.acc_no.to_string())
        .or_insert(def_npa_data);
    op_line.push_str(&npa_data.print());
    op_line.push_str(&additional_passthroughs.print());
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!(
            "TermLoans|{}|{}|{}\n",
            acc.acc_no, acc.gl_cd, acc.bal
        ));
    }
    op_line
}
