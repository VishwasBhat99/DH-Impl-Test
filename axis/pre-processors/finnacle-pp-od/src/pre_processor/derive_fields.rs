use std::collections::{HashMap, HashSet};

use super::input_account::{Benchmark, InputAccount, IntRateData, NPAData};

pub fn get_op_line(
    out_acc: &InputAccount,
    int_rate_map: &HashMap<String, IntRateData>,
    benchmark_map: &HashMap<String, Benchmark>,
    benchmark_foracid: &HashMap<String, String>,
    tblcodes_set: &HashSet<String>,
    npa_map: &HashMap<String, NPAData>,
    config_vec: &[String],
) -> String {
    let mut int_rate_data = IntRateData::new();
    let mut benchmark_data = Benchmark::new();
    if int_rate_map.contains_key(&out_acc.acid) {
        int_rate_data = int_rate_map
            .get(&out_acc.acid)
            .unwrap_or(&IntRateData::new())
            .to_owned();
    }
    if benchmark_map.contains_key(&out_acc.acid) {
        benchmark_data = benchmark_map
            .get(&out_acc.acid)
            .unwrap_or(&Benchmark::new())
            .to_owned();
    }
    let final_int_rt = int_rate_data.base_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.id_pref_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.cust_pref_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.nrml_int_pcnt.parse::<f64>().unwrap_or(0.0);
    let next_repricing_date = if out_acc.acid == benchmark_data.acid {
        benchmark_data.next_repricing_date.to_owned()
    } else {
        "".to_string()
    };
    let final_next_repricing_date = if !out_acc.next_reprice_dt.is_empty() {
        benchmark_data.next_repricing_date.to_owned()
    } else if int_rate_data.end_date != "NULL".to_string() {
        int_rate_data.end_date
    } else {
        "31-12-2099".to_string()
    };
    let mut pegged_flg = match int_rate_data.int_tbl_code.as_str() {
        "" | "NULL" => {
            if int_rate_data.pegged_flg.is_empty() || int_rate_data.pegged_flg == "NULL" {
                "Y".to_string()
            } else {
                int_rate_data.pegged_flg.to_string()
            }
        }
        _ => {
            if int_rate_data.pegged_flg.is_empty() || int_rate_data.pegged_flg == "NULL" {
                "N".to_string()
            } else {
                int_rate_data.pegged_flg.to_string()
            }
        }
    };
    if int_rate_data.acid != out_acc.acid {
        pegged_flg = "N".to_string();
    }
    let der_pegged_flg = if tblcodes_set.contains(&int_rate_data.int_tbl_code) {
        "N1".to_string()
    } else if benchmark_foracid.contains_key(&out_acc.foracid) {
        "N".to_string()
    } else if pegged_flg.to_owned().is_empty() || pegged_flg.to_owned() == "NULL" {
        "N".to_string()
    } else {
        pegged_flg.to_owned()
    };
    let npa_data = npa_map
        .get(&out_acc.foracid)
        .unwrap_or(&NPAData::new())
        .to_owned();
    let mut final_foracid_suffix = "".to_string();
    let mut final_foracid_prefix = "".to_string();
    let config_lookup = &out_acc.foracid.to_string();
    for value in config_vec {
        if config_lookup.starts_with(value) {
            final_foracid_suffix = value.to_string();
        }
        if config_lookup.ends_with(value) {
            final_foracid_prefix = value.to_string();
        }
    }
    let npa_amount = npa_data.npa_amount.parse().unwrap_or(
        out_acc.clr_bal_amt.parse().unwrap_or(0.0) + out_acc.un_clr_bal_amt.parse().unwrap_or(0.0),
    );
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        &out_acc.acid,
        &out_acc.foracid,
        &out_acc.bacid,
        &out_acc.clr_bal_amt,
        &out_acc.un_clr_bal_amt,
        &out_acc.sol_id,
        &out_acc.cust_id,
        &out_acc.acct_ownership,
        &out_acc.ledg_num,
        &out_acc.drwng_power,
        &out_acc.mode_of_oper_code,
        &out_acc.lien_amt,
        &out_acc.sanct_lim,
        &out_acc.gl_sub_head_code,
        &out_acc.schm_code,
        &out_acc.schm_type,
        &out_acc.crncy_code,
        &out_acc.acct_crncy_code,
        &out_acc.acct_cls_flg,
        &out_acc.del_flg,
        &out_acc.acct_opn_date,
        &out_acc.entity_cre_flg,
        &out_acc.acct_cls_date,
        &out_acc.last_tran_date,
        &out_acc.notional_rate_code,
        &out_acc.emp_id,
        &out_acc.notional_rate,
        &out_acc.limit_b2kid,
        &out_acc.adim1_gam,
        &out_acc.adim2_gam,
        &out_acc.adim3_gam,
        &out_acc.int_rate,
        &out_acc.bm_id,
        &out_acc.spread,
        &out_acc.reprice_freq,
        &out_acc.last_reprice_dt,
        &out_acc.next_reprice_dt,
        &out_acc.code1,
        &out_acc.code2,
        &out_acc.code3,
        &out_acc.code4,
        &out_acc.adim1_gac,
        &out_acc.adim2_gac,
        &out_acc.adim3_gac,
        &out_acc.cust_name,
        &out_acc.cmg_pan_gir_num,
        &out_acc.cmg_cust_const,
        &out_acc.adim1_cmg,
        &out_acc.adim2_cmg,
        &out_acc.adim3_cmg,
        if npa_data.npa_classification == "0" {out_acc.out_bal_amt.clone()} else {npa_amount.to_string()},
        &out_acc.cust_grp_id,
        &out_acc.ucif_cust_const,
        &out_acc.exch_rt,
        &out_acc.out_bal_amt_con,
        &out_acc.segment_code,
        &out_acc.nfs,
        &int_rate_data.base_pcnt,
        &int_rate_data.nrml_int_pcnt,
        &int_rate_data.id_pref_pcnt,
        &int_rate_data.cust_pref_pcnt,
        &int_rate_data.min_int_pcnt,
        &int_rate_data.max_int_pcnt,
        final_int_rt,
        &int_rate_data.int_tbl_code,
        pegged_flg,
        der_pegged_flg,
        &benchmark_data.repricing_plan,
        &benchmark_data.floating_type,
        next_repricing_date,
        final_next_repricing_date,
        &npa_data.npa_classification,
        &npa_data.cust_hlth_code,
        &npa_data.cust_npa_class,
        &npa_data.final_npa_class,
        &final_foracid_suffix,
        &final_foracid_prefix,
        npa_amount,
        )
}
