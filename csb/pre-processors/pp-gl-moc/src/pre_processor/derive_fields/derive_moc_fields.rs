use super::{AlmMaster, AlmMasterKey, MocInputAccount, DEFAULT_FLOAT};
use std::collections::HashMap;

pub fn get_moc_op_line(
    acc: &mut MocInputAccount,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    concats: &mut String,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.amt.parse().unwrap_or(DEFAULT_FLOAT) >= 0.0 {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("C"));
    };

    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!("GL|{}|{}\n", acc.gl_cd, acc.amt));
    }
    op_line
}
