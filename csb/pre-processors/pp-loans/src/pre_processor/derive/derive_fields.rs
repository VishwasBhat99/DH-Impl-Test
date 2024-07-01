use super::extra_fields::ExtraFieldData;

use super::{
    AlmMaster, AlmMasterKey, Cashflows, CustMasterData, CustMasterMap, InputAccount, NPAData,
    NPAMap, DEFAULT_FLOAT,
};
pub use chrono::NaiveDateTime;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    cust_master: &mut CustMasterMap,
    npa: &mut NPAMap,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    cashflows: &mut HashMap<String, Vec<Cashflows>>,
    extra_field_map: &HashMap<String, ExtraFieldData>,
    ltv_map: &HashMap<String, String>,
    concats: &mut String,
    care_cust_map: &HashMap<String, String>,
    care_acc_map: &HashMap<String, String>,
    loan_additional_map: &HashMap<String, String>,
    loan_additional_map_org: &HashMap<String, String>,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.bal.parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("C"));
    };
    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());

    let def_cust_master_data = CustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.acnts_client_num.to_string())
        .or_insert(def_cust_master_data);
    op_line.push_str(&cust_master_data.print());

    let def_npa_data = NPAData::new();
    let npa_data = npa
        .store
        .entry(acc.acc_no.to_string())
        .or_insert(def_npa_data);
    op_line.push_str(&npa_data.print());

    let pwo = npa_data.pwo.parse().unwrap_or(DEFAULT_FLOAT);
    let ho_bal = npa_data.ho_balance.parse().unwrap_or(DEFAULT_FLOAT);
    let ho_prov = npa_data.ho_provision.parse().unwrap_or(DEFAULT_FLOAT);
    let claim = npa_data.claim.parse().unwrap_or(DEFAULT_FLOAT);
    let npa_amt = ho_bal - ho_prov - claim;
    op_line.push_str(&npa_amt.to_string());
    op_line.push('|');

    if let Some(cfs) = cashflows.get(&acc.acc_no) {
        for cf in cfs.iter() {
            op_line.push_str(&cf.print());
        }
        op_line.pop();
    }
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
    // calculation of NPA provision percentage
    let prov_prct = if (pwo + ho_bal) != 0.0 {
        ((pwo + ho_prov + claim) / (pwo + ho_bal)) * 100.0
    } else {
        0.0
    };
    let def_str_val = String::from("NA");
    let actual_cust_care_value = care_cust_map
        .get(&acc.acnts_client_num)
        .unwrap_or(&def_str_val);
    let actual_acc_care_value = care_acc_map.get(&acc.acc_no).unwrap_or(&def_str_val);
    // 15 additional passthrough for future purpose
    // 5 Int, 5 Float, 5 String
    // pt_str_1 is clients group code
    // pt_str_2 is blank
    // pt_str_3 is int benchmark
    // pt_str_4 is actual cust care value
    // pt_str_5 is actual acc care value

    op_line.push_str("||||||");
    op_line.push_str(&prov_prct.to_string());
    op_line.push_str("|||||");
    op_line.push_str(&cust_master_data.clients_group_code);
    op_line.push_str("||");
    let def_int_benchmark_val = String::from("NA");
    let int_benchmark = loan_additional_map
        .get(&acc.acc_no)
        .unwrap_or(&def_int_benchmark_val);
    op_line.push_str(&int_benchmark);
    op_line.push_str("|");
    op_line.push_str(&actual_cust_care_value);
    op_line.push_str("|");
    op_line.push_str(&actual_acc_care_value);
    op_line.push_str("|");
    let def_org_code = String::from("NA");
    let org_code = loan_additional_map_org
        .get(&acc.acc_no)
        .unwrap_or(&def_org_code);
    op_line.push_str(&org_code);
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!(
            "TermLoans|{}|{}|{}\n",
            acc.acc_no, acc.gl_cd, acc.bal
        ));
    }
    op_line
}
