use super::{
    AlmMaster, AlmMasterKey, CustMasterData, CustMasterInput, CustMasterMap, InputAccount,
    DEFAULT_FLOAT,
};
use calamine::DataType;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    cust_master: &mut CustMasterMap,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    treasury_gl_map: &mut HashMap<String, String>,
    concats: &mut String,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());

    let def_treasury_gl: String = String::from("NA");
    let cbs_gl_code = treasury_gl_map
        .get(&acc.treasury_gl_code)
        .unwrap_or(&def_treasury_gl);

    op_line.push_str(cbs_gl_code);

    let mut alm_master_key = AlmMasterKey::new();
    if acc.os_bal.parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
        alm_master_key.insert(cbs_gl_code.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(cbs_gl_code.to_string(), String::from("C"));
    };
    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());

    let def_cust_master_data = CustMasterData::new();
    let cust_master_data = cust_master
        .store
        .entry(acc.counter_party_id.to_string())
        .or_insert(def_cust_master_data);
    op_line.push_str(&cust_master_data.print());

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!(
            "{}|{}|{}\n",
            acc.deal_num, acc.treasury_gl_code, acc.os_bal
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
