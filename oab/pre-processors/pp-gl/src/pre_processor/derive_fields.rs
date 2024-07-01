use super::{AlmMaster, AlmMasterKey, InputAccount, DEFAULT_FLOAT};
use calamine::DataType;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    concats: &mut String,
    cf_type: &str,
    is_acc_gl: &str,
) -> String {
    let mut op_line = String::new();
    op_line.push_str(&acc.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.os_bal_lcy.parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(acc.gl_cd.to_string(), String::from("C"));
    };

    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());
    op_line.push_str(cf_type);
    op_line.push('|');
    op_line.push_str(is_acc_gl);
    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!("GL|{}|{}\n", acc.gl_cd, acc.os_bal_lcy));
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
            gl_desc: get_data(&row[1]),
            w4b_cd: get_data(&row[3]),
            w4b_desc: get_data(&row[4]),
            balm_llg: get_data(&row[5]),
            care_llg: get_data(&row[6]),
            ba_llg: get_data(&row[7]),
        },
    );
}
