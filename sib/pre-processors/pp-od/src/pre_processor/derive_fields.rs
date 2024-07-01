use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use rbdate::{incr_dt_by_days, num_days_start_to_end};

use super::input_account::{AdditionLoanFile, InputAccount, IntRateData, NPAData, RateCode};

pub fn get_op_line(
    out_acc: &InputAccount,
    int_rate_map: &HashMap<String, IntRateData>,
    ratecode_map: &HashMap<String, RateCode>,
    tblcodes_set: &HashSet<String>,
    npa_map: &HashMap<String, NPAData>,
    additional_loan_file_map: &HashMap<String, AdditionLoanFile>,
    config_vec: &[String],
    as_on_date: NaiveDate,
) -> String {
    let mut int_rate_data = IntRateData::new();
    let mut ratecode_data = RateCode::new();
    if int_rate_map.contains_key(&out_acc.acid) {
        int_rate_data = int_rate_map
            .get(&out_acc.acid)
            .unwrap_or(&IntRateData::new())
            .to_owned();
    }
    if ratecode_map.contains_key(&out_acc.bm_id) {
        ratecode_data = ratecode_map
            .get(&out_acc.bm_id)
            .unwrap_or(&RateCode::new())
            .to_owned();
    }
    let final_int_rt = int_rate_data.base_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.id_pref_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.cust_pref_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.nrml_int_pcnt.parse::<f64>().unwrap_or(0.0);

    let date_diff = num_days_start_to_end(
        as_on_date,
        NaiveDate::parse_from_str(&out_acc.ucif_cust_const, "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).expect("Unable to get Def-Date")),
    );
    let tran_date_diff = num_days_start_to_end(
        as_on_date,
        NaiveDate::parse_from_str(&out_acc.last_tran_date, "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).expect("Unable to get Def-Date")),
    );

    let next_repricing_date = if date_diff <= 365 {
        NaiveDate::parse_from_str(&out_acc.ucif_cust_const, "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).expect("Unable to get Def-Date"))
    } else if date_diff > 365 && tran_date_diff <= 365 {
        NaiveDate::parse_from_str(&out_acc.last_tran_date, "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).expect("Unable to get Def-Date"))
    } else {
        incr_dt_by_days(as_on_date, 365)
    };
    let final_next_repricing_date = if int_rate_data.end_date != *"NULL" {
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
    } else if pegged_flg.is_empty() || pegged_flg == "NULL" {
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
    
    
let mut loan_data:AdditionLoanFile= AdditionLoanFile::new();
let default_loan_data =AdditionLoanFile::new();
if additional_loan_file_map.contains_key(&out_acc.acid) {
    let mut loan_data = additional_loan_file_map
            .get(&out_acc.acid)
            .unwrap_or(&default_loan_data);
}
    format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}g",
        &out_acc.acid,
        &out_acc.foracid,
        &out_acc.bacid,
        &out_acc.clr_bal_amt.parse::<f64>().unwrap_or(0.0).abs(),
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
        if npa_data.npa_classification == "0" {
            out_acc.out_bal_amt.clone()
        } else {
            npa_amount.to_string()
        },
        &out_acc.cust_grp_id,
        &out_acc.ucif_cust_const,
        &out_acc.exch_rt,
        &out_acc.out_bal_amt_con.parse::<f64>().unwrap_or(0.0).abs(),
        &out_acc.segment_code.parse::<f64>().unwrap_or(0.0).abs(),
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
        &ratecode_data.fixed_floating,
        &ratecode_data.benchmark,
        next_repricing_date.format("%d-%m-%Y"),
        final_next_repricing_date,
        &npa_data.npa_classification,
        &npa_data.cust_hlth_code,
        &npa_data.cust_npa_class,
        &npa_data.final_npa_class,
        &final_foracid_suffix,
        &final_foracid_prefix,
        npa_amount,
        loan_data.gnt_type,
        loan_data.status_code,
        loan_data.occupation_code,
        loan_data.sector,
        loan_data.sector_code,
        loan_data.subsector_code,
        loan_data.staffflag,
        loan_data.cre_free_text_1,
        loan_data.prov_percent,
        (out_acc.out_bal_amt.parse::<f64>().unwrap_or(0.0).abs()/((loan_data.pres_val_sec)*(loan_data.paripassu_perc))) * 100.0,
        loan_data.dumm3,
        loan_data.dumm4,
        loan_data.dumm5,
        loan_data.dumm6,
        loan_data.dumm7,
        loan_data.dumm8,
        loan_data.dumm9.format("%d-%m-%Y"),
        loan_data.dumm10.format("%d-%m-%Y"),
        loan_data.const_catgory_code,
        loan_data.rating_agc,
        loan_data.rating,
        loan_data.super_annuation_flag,
        loan_data.turn_amt1,
        loan_data.turn_amt2,
        loan_data.turn_amt_3,
        loan_data.ftp_amt1,
        loan_data.ftp_amt2,
        loan_data.ftp_char1,
        loan_data.ftp_char2,
        loan_data.ftp_date1.format("%d-%m-%Y"),
        loan_data.ftp_date2.format("%d-%m-%Y")
    )
}
