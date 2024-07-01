use super::structs::{LLGKey, LLGVal};
use std::collections::HashMap;

pub fn aggregate(op_map: &mut HashMap<LLGKey, LLGVal>, llg_key: LLGKey, llg_val: LLGVal) {
    let new_val: LLGVal = if op_map.contains_key(&llg_key) {
        let existing_val = op_map.get(&llg_key).unwrap();
        let new_tot_bal_ccy = existing_val.tot_bal_ccy.parse::<f64>().expect(&format!(
            "Cannot fetch amount for account number : {} and currency : {}",
            llg_key.llg, llg_key.ccy
        )) + llg_val.tot_bal_ccy.parse::<f64>().expect(&format!(
            "Cannot fetch amount for account number : {} and currency : {}",
            llg_key.llg, llg_key.ccy
        ));
        let new_tot_bal_hcy = existing_val.tot_bal_hcy.parse::<f64>().expect(&format!(
            "Cannot fetch amount for account number : {} and currency : {}",
            llg_key.llg, llg_key.ccy
        )) + llg_val.tot_bal_hcy.parse::<f64>().expect(&format!(
            "Cannot fetch amount for account number : {} and currency : {}",
            llg_key.llg, llg_key.ccy
        ));
        let new_weighted_dur = existing_val.weighted_dur.parse::<f64>().expect(&format!(
            "Cannot fetch amount for account number : {} and currency : {}",
            llg_key.llg, llg_key.ccy
        )) + llg_val.weighted_dur.parse::<f64>().expect(&format!(
            "Cannot fetch amount for account number : {} and currency : {}",
            llg_key.llg, llg_key.ccy
        ));
        LLGVal {
            tot_bal_ccy: new_tot_bal_ccy.to_string(),
            tot_bal_hcy: new_tot_bal_hcy.to_string(),
            weighted_dur: new_weighted_dur.to_string(),
        }
    } else {
        LLGVal {
            tot_bal_ccy: llg_val.tot_bal_ccy,
            tot_bal_hcy: llg_val.tot_bal_hcy,
            weighted_dur: llg_val.weighted_dur,
        }
    };
    op_map.insert(llg_key, new_val);
}
