use super::account::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::npa::account_field_names::AccFieldNames;
use crate::npa::llg_key::LLGKey;
use crate::npa::tenor::*;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::collections::HashMap;

#[allow(dead_code, unused_imports)]
pub fn fetch_acc_data(
    tenor_map: &HashMap<Tenor, String>,
    exrt: &f64,
    source: &str,
    account: AccountWithCFs,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
    npa_flag_map: &mut HashMap<String, String>,
    alco_map: &mut HashMap<String, String>,
    psl_map: &mut HashMap<String, String>,
    npa_amt_map: &mut HashMap<String, f64>,
    npa_unser_int_sus_map: &mut HashMap<String, String>,
    npa_unser_oth_inc_map: &mut HashMap<String, String>,
    npa_date_map: &mut HashMap<String, String>,
    is_neg: bool,
) -> AccData {
    let def_val = "NA".to_string();
    let opn_dt = account.get_i64_for_key(&keys.open_date).unwrap_or(0);
    let mat_dt = account.get_i64_for_key(&keys.mat_date).unwrap_or(0);
    let acc_no = match account.get_i64_for_key(&keys.acc_no) {
        Ok(code) => get_string_from_i64(code),
        Err(_) => match account.get_string_for_key(&keys.acc_no) {
            Ok(code) => code.to_string(),
            Err(_) => def_val.to_string(),
        },
    };
    let alm_line = account
        .get_string_for_key(&keys.alm_line)
        .unwrap_or(&def_val);
    let psl_code = match account.get_i64_for_key(&keys.psl_code) {
        Ok(code) => get_string_from_i64(code),
        Err(_) => match account.get_string_for_key(&keys.psl_code) {
            Ok(code) => code.to_string(),
            Err(_) => def_val.to_string(),
        },
    };

    let grp_key = LLGKey::new(
        *config_params.as_on_date(),
        source.to_string(),
        account
            .get_string_for_key(&keys.acc_no)
            .unwrap_or(&def_val)
            .to_string(),
        account
            .get_string_for_key(&keys.prod_code)
            .unwrap_or(&def_val)
            .to_string(),
        account
            .get_string_for_key(&keys.scheme_id)
            .unwrap_or(&def_val)
            .to_string(),
        npa_flag_map
            .entry(acc_no.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        match account.get_i64_for_key(&keys.mis1) {
            Ok(code) => get_string_from_i64(code),
            Err(_) => match account.get_string_for_key(&keys.mis1) {
                Ok(code) => code.to_string(),
                Err(_) => def_val.to_string(),
            },
        },
        match account.get_i64_for_key(&keys.mis2) {
            Ok(code) => get_string_from_i64(code),
            Err(_) => match account.get_string_for_key(&keys.mis2) {
                Ok(code) => code.to_string(),
                Err(_) => def_val.to_string(),
            },
        },
        match account.get_i64_for_key(&keys.mis3) {
            Ok(code) => get_string_from_i64(code),
            Err(_) => match account.get_string_for_key(&keys.mis3) {
                Ok(code) => code.to_string(),
                Err(_) => def_val.to_string(),
            },
        },
        account
            .get_string_for_key(&keys.raw_bm)
            .unwrap_or(&def_val)
            .to_string(),
        account
            .get_string_for_key(&keys.final_bm)
            .unwrap_or(&def_val)
            .to_string(),
        match account.get_i64_for_key(&keys.concat) {
            Ok(code) => get_string_from_i64(code),
            Err(_) => match account.get_string_for_key(&keys.concat) {
                Ok(code) => code.to_string(),
                Err(_) => def_val.to_string(),
            },
        },
        account
            .get_string_for_key(&keys.npa_flag)
            .unwrap_or(&def_val)
            .to_string(),
        account
            .get_string_for_key(&keys.div)
            .unwrap_or(&def_val)
            .to_string(),
        account
            .get_string_for_key(&keys.alm_line)
            .unwrap_or(&def_val)
            .to_string(),
        account
            .get_string_for_key(&keys.ia_line)
            .unwrap_or(&def_val)
            .to_string(),
        get_tenor_desc((mat_dt - opn_dt) / 86400, tenor_map),
        alco_map
            .entry(alm_line.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        psl_map
            .entry(psl_code.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
    );

    let mut amt = account.get_f64_for_key(&keys.amt_as_per_src).unwrap_or(0.0);
    let amt_as_per_npa = npa_amt_map
        .entry(acc_no.to_string())
        .or_insert(0.0)
        .to_string()
        .parse::<f64>()
        .unwrap_or(0.0);
    let unser_int_sus = npa_unser_int_sus_map
        .entry(acc_no.to_string())
        .or_insert(def_val.to_string())
        .to_string();
    let unser_oth_inc = npa_unser_oth_inc_map
        .entry(acc_no.to_string())
        .or_insert(def_val.to_string())
        .to_string();
    let npa_date = npa_date_map
        .entry(acc_no.to_string())
        .or_insert(def_val.to_string())
        .to_string();
    if is_neg {
        amt *= -1.0;
    }
    let yield_rate = account.get_f64_for_key(&keys.yield_rate).unwrap_or(0.0);
    let mut npa_data = NpaVal {
        amt_as_per_npa: amt_as_per_npa,
        amt_as_per_src: amt,
        yield_rate: yield_rate * amt,
        unser_int_sus: unser_int_sus,
        unser_oth_inc: unser_oth_inc,
        npa_date: npa_date,
    };
    npa_data.values_multiplied_by(*exrt);
    AccData { grp_key, npa_data }
}

fn get_string_from_i64(val: i64) -> String {
    if val == 0 {
        String::from("NA")
    } else {
        val.to_string()
    }
}
