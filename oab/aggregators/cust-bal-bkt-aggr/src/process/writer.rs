use crate::process::account::*;
use std::collections::HashMap;

pub fn write_data(
    total_amt: f64,
    fileid_llg_mapper: &mut HashMap<String, LLGMapper>,
    mut less_stable_vec: Vec<f64>,
    mut stable_vec: Vec<f64>,
    nwd_less_stable_amt: f64,
    nwd_stable_amt: f64,
    inp_acc: Account,
    is_nwd: bool,
    writer_map: &mut HashMap<(String, String), Vec<f64>>,
) {
    if total_amt == 0.0 && is_nwd {
        stable_vec = vec![0.0; 203];
        less_stable_vec = vec![0.0; 203];
    }
    let def_llgids = fileid_llg_mapper
        .get(&"DEF".to_string())
        .expect("Could not get Default LLG-IDs");
    let llgids = fileid_llg_mapper
        .get(&inp_acc.file_id)
        .unwrap_or(def_llgids);

    let (stable_llgid, less_stable_llgid) =
        if is_nwd && (nwd_less_stable_amt + nwd_stable_amt) != 0.0 {
            (
                llgids.nwd_stable_llgid.to_string(),
                llgids.nwd_less_stable_llgid.to_string(),
            )
        } else {
            (
                llgids.wd_stable_llgid.to_string(),
                llgids.wd_less_stable_llgid.to_string(),
            )
        };
    writer_map
        .entry((inp_acc.ccy.to_string(), stable_llgid))
        .and_modify(|amts| aggr_data(amts, stable_vec.clone()))
        .or_insert(stable_vec);
    writer_map
        .entry((inp_acc.ccy.to_string(), less_stable_llgid))
        .and_modify(|amts| aggr_data(amts, less_stable_vec.clone()))
        .or_insert(less_stable_vec);
}

pub fn aggr_data(present_data: &mut Vec<f64>, new_data: Vec<f64>) {
    for bucket in 0..=present_data.len() - 1 {
        present_data[bucket] += new_data[bucket];
    }
}
