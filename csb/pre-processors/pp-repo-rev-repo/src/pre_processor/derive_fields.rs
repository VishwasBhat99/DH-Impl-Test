use super::{
    AlmMaster, AlmMasterKey, CustMasterData, CustMasterInput, CustMasterMap, GLMasterMap,
    InputAccount, DEFAULT_FLOAT,
};
use calamine::DataType;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    cust_master: &mut CustMasterMap,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    concats: &mut String,
    gl_cd: &str,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());

    let def_cust_master_data = CustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.cntr_party_id.to_string())
        .or_insert(def_cust_master_data);
    op_line.push_str(&cust_master_data.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.book_value.parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
        alm_master_key.insert(String::from(gl_cd), String::from("D"));
    } else {
        alm_master_key.insert(String::from(gl_cd), String::from("C"));
    };

    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!(
            "{}|{}|{}|{}\n",
            "RepoRevRepo", acc.deal_no, gl_cd, acc.book_value
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

pub fn get_gl_code(gl_cd: String, gl_master_map: &mut GLMasterMap) -> String {
    gl_master_map
        .store
        .entry(gl_cd)
        .or_insert_with(|| String::default())
        .to_string()
}
