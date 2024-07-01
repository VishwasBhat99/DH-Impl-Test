use super::account::*;
use crate::aggregator::rate_bucket::*;
use aggregator::account_field_names::AccFieldNames;
use aggregator::llg_key::LLGKey;
use aggregator::tenor::*;
use configuration_parameters::ConfigurationParameters;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;

#[allow(dead_code, unused_imports)]
pub fn fetch_acc_data(
    tenor_map: &HashMap<Tenor, String>,
    exrt: &f64,
    source: &str,
    account: AccountWithCFs,
    keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
    alco_map: &mut HashMap<String, String>,
    psl_map: &mut HashMap<String, String>,
    rate_bucket_map: &mut HashMap<Bucket, String>,
    is_neg: bool,
    logger: &Logger,
    is_cf_passed: bool,
    cf_amt: f64,
) -> AccData {
    let def_val = "NA".to_string();
    let val_dt = account.get_i64_for_key(&keys.val_date).unwrap_or(0);
    let mat_dt = account.get_i64_for_key(&keys.mat_date).unwrap_or(0);
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
    let rate_bucket = account.get_f64_for_key(&keys.yield_rate).unwrap_or(0.0);

    let grp_key = LLGKey::new(
        *config_params.as_on_date(),
        source.to_string(),
        account
            .get_string_for_key(&keys.ccy)
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
        get_tenor_desc((mat_dt - val_dt) / 86400, tenor_map),
        alco_map
            .entry(alm_line.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        psl_map
            .entry(psl_code.to_string())
            .or_insert(def_val.to_string())
            .to_string(),
        get_amt_cat(rate_bucket, rate_bucket_map, config_params, logger),
    );

    let mut amt = if is_cf_passed {
        cf_amt
    } else {
        account.get_f64_for_key(&keys.amt).unwrap_or(0.0)
    };
    if is_neg {
        amt *= -1.0;
    }
    let yield_rate = account.get_f64_for_key(&keys.yield_rate).unwrap_or(0.0);
    let mut aggr_data = AggrVal {
        tot_amt: amt,
        wt_yield_rate: yield_rate * amt,
    };
    aggr_data.values_multiplied_by(*exrt);
    AccData { grp_key, aggr_data }
}

fn get_string_from_i64(val: i64) -> String {
    if val == 0 {
        String::from("NA")
    } else {
        val.to_string()
    }
}
