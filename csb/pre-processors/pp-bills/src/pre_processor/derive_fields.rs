use super::structs::extra_fields::*;
use super::{
    AlmMaster, AlmMasterKey, CustMasterData, CustMasterInput, CustMasterMap, ExtraFieldData,
    InputAccount, NPAData, NPAInput, NPAMap, DEFAULT_FLOAT,
};
use calamine::DataType;
use chrono::{Duration, NaiveDate};
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    npa: &mut NPAMap,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    bank_master: &mut HashMap<String, BankData>,
    concats: &mut String,
    cust_master: &mut CustMasterMap,
    dyn_bkp: &mut HashMap<String, i64>,
    as_on_dt: NaiveDate,
    src_file_name: &str,
    extra_field_map: &HashMap<String, ExtraFieldData>,
    ltv_map: &HashMap<String, String>,
    care_cust_map: &HashMap<String, String>,
    care_acc_map: &HashMap<String, String>,
    loan_additional_map: &HashMap<String, String>,
) -> String {
    let mut op_line = String::new();

    let mat_dt = NaiveDate::parse_from_str(&acc.mat_dt, "%d-%m-%Y");
    if mat_dt.is_err() && dyn_bkp.contains_key(&acc.lbm_gl) {
        let days = dyn_bkp.get(&acc.lbm_gl).unwrap_or(&0);
        acc.mat_dt = (as_on_dt + Duration::days(*days))
            .format("%d-%m-%Y")
            .to_string();
    }
    op_line.push_str(&acc.print());

    let def_cust_master_data = CustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.client_id.to_string())
        .or_insert(def_cust_master_data);

    op_line.push_str(&cust_master_data.print());
    let mut alm_master_key = AlmMasterKey::new();
    if acc.bal_lcy.parse().unwrap_or(DEFAULT_FLOAT) > 0.0 {
        alm_master_key.insert(acc.lbm_gl.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(acc.lbm_gl.to_string(), String::from("C"));
    };
    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());

    let def_npa_data = NPAData::new();
    let npa_data = npa
        .store
        .entry(acc.bill_id.to_string())
        .or_insert(def_npa_data);
    op_line.push_str(&npa_data.print());

    let pwo = npa_data.pwo.parse().unwrap_or(DEFAULT_FLOAT);
    let ho_bal = npa_data.ho_balance.parse().unwrap_or(DEFAULT_FLOAT);
    let ho_prov = npa_data.ho_provision.parse().unwrap_or(DEFAULT_FLOAT);
    let claim = npa_data.claim.parse().unwrap_or(DEFAULT_FLOAT);
    let npa_amt = ho_bal + ho_prov - claim;

    op_line.push_str(&npa_amt.to_string());
    let def_additional_data = ExtraFieldData {
        ..Default::default()
    };
    let additional_data = extra_field_map
        .get(&acc.bill_id)
        .unwrap_or(&def_additional_data);
    op_line.push_str(&additional_data.print());
    let def_ltv_data = format!("1000.0");
    let ltv_data = ltv_map.get(&acc.bill_id).unwrap_or(&def_ltv_data);
    op_line.push_str(&ltv_data);
    // calculation of NPA provision percentage
    let prov_prct = if (pwo + ho_bal) != 0.0 {
        ((pwo + ho_prov + claim) / (pwo + ho_bal)) * 100.0
    } else {
        0.0
    };
    let def_bank_data = BankData {
        cet: 0.0,
        class1: "NA".to_string(),
        class2: "NA".to_string(),
        class3: "NA".to_string(),
    };
    let bank_data = bank_master
        .get(&acc.bank_name.trim().to_uppercase())
        .unwrap_or(&def_bank_data);
    let def_str_val = String::from("NA");
    let actual_cust_care_value = care_cust_map.get(&acc.client_id).unwrap_or(&def_str_val);
    let actual_acc_care_value = care_acc_map.get(&acc.bill_id).unwrap_or(&def_str_val);
    // 15 additional passthrough for future purpose
    // 5 Int, 5 Float, 5 String
    // pt_f64_1 is provision percentage
    // pt_f64_2 is cet percentage
    // pt_str_1 is clients group code
    // pt_str_2 is bank classification 1
    // pt_str_3 is bank classification 2
    // pt_str_4 is bank classification 3
    // pt_str_5 is int benchmark

    op_line.push_str("||||||");
    op_line.push_str(&prov_prct.to_string());
    op_line.push_str("|");
    op_line.push_str(&bank_data.cet.to_string());
    op_line.push_str("||||");
    op_line.push_str(&cust_master_data.clients_group_code);
    op_line.push_str("|");
    op_line.push_str(&bank_data.class1);
    op_line.push_str("|");
    op_line.push_str(&bank_data.class2);
    op_line.push_str("|");
    op_line.push_str(&bank_data.class3);
    op_line.push_str("|");
    op_line.push_str(&actual_cust_care_value);
    op_line.push_str("|");
    op_line.push_str(&actual_acc_care_value);
    op_line.push_str("|");
    //loan_additional_file_fields:
    let def_int_benchmark = String::from("NA");
    let int_benchmark = loan_additional_map
        .get(&acc.bill_id)
        .unwrap_or(&def_int_benchmark);
    op_line.push_str(int_benchmark);
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!(
            "{}|{}|{}|{}\n",
            src_file_name, acc.bill_id, acc.lbm_gl, acc.bal
        ));
    }
    op_line
}

pub fn get_alm_master_data(row: &[DataType], alm_master: &mut HashMap<AlmMasterKey, AlmMaster>) {
    fn get_data(data: &DataType) -> String {
        data.to_string().replace("\u{a0}", " ")
    }

    alm_master.insert(
        AlmMasterKey {
            gl_cd: get_data(&row[0]),
            dr_cr: get_data(&row[2]),
        },
        AlmMaster {
            w4b_cd: get_data(&row[3]),
            balm_llg: get_data(&row[5]),
            care_llg: get_data(&row[6]),
            ba_llg: get_data(&row[7]),
        },
    );
}

pub fn get_npa_data(npa_input: NPAInput, npa_map: &mut NPAMap) {
    let mut npa_data = NPAData::new();
    npa_data.insert(npa_input.clone());
    npa_map
        .store
        .insert(npa_input.account_no.trim().to_string(), npa_data);
}

pub fn get_cust_master_data(
    cust_master_input: CustMasterInput,
    cust_master_map: &mut CustMasterMap,
) {
    let mut cust_master_data = CustMasterData::new();
    cust_master_data.insert(cust_master_input.clone());
    cust_master_map.store.insert(
        cust_master_input.clients_code.trim().to_string(),
        cust_master_data,
    );
}
