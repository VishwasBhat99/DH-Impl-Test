use super::{AlmMaster, AlmMasterKey};
use calamine::DataType;
use std::collections::HashMap;

pub fn get_output_line(
    fields: &[&str],
    alm_master: &mut AlmMaster,
) -> String {
    let mut output_acc_info: String = String::new();

    for field in fields.iter() {
        output_acc_info.push_str(field);
        output_acc_info.push('|');
    }

    output_acc_info.push_str(&alm_master.print());

    output_acc_info.push_str("\n");

    output_acc_info
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
