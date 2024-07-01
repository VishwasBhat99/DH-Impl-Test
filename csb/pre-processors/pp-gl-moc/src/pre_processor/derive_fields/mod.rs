pub mod derive_moc_fields;
use std::collections::HashMap;

use super::{AlmMaster, AlmMasterKey, MocInputAccount, DEFAULT_FLOAT};
use calamine::DataType;

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
