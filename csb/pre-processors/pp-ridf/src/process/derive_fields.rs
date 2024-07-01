use super::{AlmMaster, AlmMasterMap, RIDFData, RIDFInput};
use calamine::DataType;
pub use chrono::NaiveDateTime;

pub fn get_td_op_line(ridf_data: &mut RIDFData, alm_master: &mut AlmMasterMap) -> String {
    let mut op_line = String::new();
    op_line.push_str(&ridf_data.print());
    let def_alm_master_data = AlmMaster::new();
    let alm_master_data = alm_master
        .store
        .entry(ridf_data.gl_code.to_string())
        .or_insert(def_alm_master_data);
    op_line.push_str(&alm_master_data.print());

    op_line.push('\n');

    op_line
}

pub fn get_alm_master_data(row: &[DataType], alm_master_map: &mut AlmMasterMap) {
    fn get_data(data: &DataType) -> String {
        data.to_string().replace("\u{a0}", " ")
    }
    let alm_master = AlmMaster {
        gl_desc: get_data(&row[1]),
        w4b_cd: get_data(&row[3]),
        w4b_desc: get_data(&row[4]),
        balm_llg: get_data(&row[5]),
        care_llg: get_data(&row[6]),
        ba_llg: get_data(&row[7]),
    };
    alm_master_map.store.insert(get_data(&row[0]), alm_master);
}

pub fn get_cust_master_data(cust_master_input: &mut RIDFInput, cust_master_data: &mut RIDFData) {
    cust_master_data.insert(cust_master_input.clone());
}
