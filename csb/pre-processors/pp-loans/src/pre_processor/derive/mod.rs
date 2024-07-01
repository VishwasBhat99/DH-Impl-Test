pub mod derive_fields;
pub mod derive_td_fields;
pub mod extra_fields;
use super::{
    AdditionalPassThroughs, AlmMaster, AlmMasterKey, Cashflows, CustMasterData, CustMasterInput,
    CustMasterMap, InputAccount, NPAData, NPAInput, NPAMap, Schedules, TdCrCustMasterData,
    TdCrCustMasterInput, TdCrCustMasterMap, DEFAULT_FLOAT,
};
use calamine::DataType;
pub use chrono::NaiveDateTime;
use std::collections::HashMap;

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
    cust_master_map
        .store
        .insert(cust_master_input.clients_code, cust_master_data);
}

pub fn get_td_cr_cust_master_data(
    cust_master_input: TdCrCustMasterInput,
    cust_master_map: &mut TdCrCustMasterMap,
) {
    let mut cust_master_data = TdCrCustMasterData::new();
    cust_master_data.insert(cust_master_input.clone());
    cust_master_map
        .store
        .insert(cust_master_input.clients_code, cust_master_data);
}

pub fn get_schedule_data(
    schedules: &mut Schedules,
    cashflows: &mut HashMap<String, Vec<Cashflows>>,
) {
    let mut cfs: Cashflows = Cashflows::new();
    schedules.freq = get_freq(&schedules.freq);
    cfs.insert(schedules.clone());
    let cfs_mov = cfs.clone();

    cashflows
        .entry(schedules.acc_no.to_string())
        .and_modify(|cf| cf.push(cfs_mov))
        .or_insert(vec![cfs]);
}

fn get_freq(freq: &str) -> String {
    let freq = match freq.to_uppercase().as_str() {
        "MONTHLY" => 1,
        "BI MONTHLY" => 2,
        "QUARTERLY" => 3,
        "HALF YEARLY" => 6,
        "YEARLY" => 12,
        "AT MATURITY" => 0,
        _ => 0,
    };
    freq.to_string()
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
