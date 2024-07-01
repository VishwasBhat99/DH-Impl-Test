use super::{
    AlmMaster, AlmMasterKey, GLMapData, GLMapInput, GLMapMap, InputAccount, DEFAULT_FLOAT,
};
use calamine::DataType;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &mut InputAccount,
    alm_master: &mut HashMap<AlmMasterKey, AlmMaster>,
    concats: &mut String,
    gl_map: &mut GLMapMap,
    src_file_name: &str,
) -> String {
    let mut op_line = String::new();
    let mut prin_amt = "0.0";
    let mut int_amt = "0.0";

    op_line.push_str(&acc.print());
    if acc.cf_typ.trim_matches('"').to_uppercase() == "PRINCIPAL" {
        prin_amt = &acc.cf_amt;
    } else if acc.cf_typ.trim_matches('"').to_uppercase() == "INTEREST" {
        int_amt = &acc.cf_amt;
    }
    op_line.push_str(prin_amt);
    op_line.push_str("|");
    op_line.push_str(int_amt);
    op_line.push_str("|");
    let flow_type = if acc.cf_amt.parse::<f64>().expect("Invalid cf amt") > 0.0 {
        "I"
    } else {
        "O"
    };
    let abs_cf_amt = acc.cf_amt.parse::<f64>().expect("Invalid cf amt").abs();
    let def_gl_map_data = GLMapData::new();
    op_line.push_str(flow_type);
    op_line.push_str("|");
    op_line.push_str(&abs_cf_amt.to_string());
    op_line.push_str("|");
    let gl_map_data = gl_map
        .store
        .entry(acc.trsy_gl.to_string())
        .or_insert(def_gl_map_data);
    op_line.push_str(&gl_map_data.print());

    let mut alm_master_key = AlmMasterKey::new();
    if acc.cf_amt.parse().unwrap_or(DEFAULT_FLOAT) > 0.0 {
        alm_master_key.insert(gl_map_data.cbs_gl_cd.to_string(), String::from("D"));
    } else {
        alm_master_key.insert(gl_map_data.cbs_gl_cd.to_string(), String::from("C"));
    };
    let def_alm_master = AlmMaster::new();
    let alm_master = alm_master.entry(alm_master_key).or_insert(def_alm_master);
    op_line.push_str(&alm_master.print());

    op_line.push('\n');

    if alm_master.balm_llg == "NONE" {
        concats.push_str(&format!(
            "{}|{}|{}|{}\n",
            src_file_name, acc.deal_ref, acc.trsy_gl, acc.cf_amt
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

pub fn get_gl_map_data(gl_map_input: GLMapInput, gl_map_map: &mut GLMapMap) {
    let mut gl_map_data = GLMapData::new();
    gl_map_data.insert(gl_map_input.clone());
    gl_map_map
        .store
        .insert(gl_map_input.gl_cd.trim().to_string(), gl_map_data);
}
