use std::collections::HashMap;

use super::input_account::{EitData, InputAccount, IntRateData};
use rbdate::NaiveDate;

pub fn get_op_line(
    out_acc: &mut InputAccount,
    int_rate_map: &HashMap<String, IntRateData>,
    eit_map: &HashMap<String, EitData>,
    as_on_date: &NaiveDate,
    tot_amt: &mut f64,
    append_data: String,
) -> String {
    let mut int_rate_data = IntRateData::new();
    if int_rate_map.contains_key(&out_acc.acid) {
        int_rate_data = int_rate_map
            .get(&out_acc.acid)
            .expect("Cannot get int rate data from Int Rate File.")
            .to_owned();
    }
    let mat_dt = NaiveDate::parse_from_str(int_rate_data.maturity_date.as_str(), "%d-%m-%Y")
        .unwrap_or(*as_on_date);
    let overdue_flg: String = if mat_dt <= *as_on_date {
        "Y".to_string()
    } else {
        "N".to_string()
    };
    let final_int_rate = int_rate_data
        .cust_cr_pref_pcnt
        .parse::<f64>()
        .unwrap_or(0.0)
        + int_rate_data.id_cr_pref_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.base_pcnt.parse::<f64>().unwrap_or(0.0)
        + int_rate_data.nrml_int_pcnt.parse::<f64>().unwrap_or(0.0);
    *tot_amt += out_acc.out_bal_amt.parse::<f64>().unwrap_or(0.0);

    //Modify output fields with EIT data if exists
    if !eit_map.is_empty() {
        out_acc.out_bal_amt_con = 0.0.to_string();
        let default_eit = EitData::new();
        let eit_data = eit_map.get(&out_acc.acid).unwrap_or(&default_eit);
        int_rate_data.base_pcnt_dr = eit_data.nrml_accrued_amount_cr.to_string();
        int_rate_data.base_pcnt_cr = eit_data.nrml_interest_amount_cr.to_string();
        if out_acc.schm_code.starts_with("KN") && !out_acc.gl_sub_head_code.starts_with('0') {
            out_acc.out_bal_amt_con =
                (eit_data.nrml_accrued_amount_cr - eit_data.nrml_interest_amount_cr).to_string()
        }
    }

    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
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
        &out_acc.out_bal_amt,
        &out_acc.cust_grp_id,
        &out_acc.ucif_cust_const,
        &out_acc.exch_rt,
        &out_acc.out_bal_amt_con,
        &out_acc.segment_code,
        &out_acc.nfs,
        &int_rate_data.del_flg,
        &int_rate_data.open_effective_date,
        &int_rate_data.schm_type,
        &int_rate_data.int_tbl_code,
        &int_rate_data.int_version,
        &int_rate_data.int_tbl_ver_num,
        &int_rate_data.min_int_pcnt_cr,
        &int_rate_data.max_int_pcnt_cr,
        &int_rate_data.cust_cr_pref_pcnt,
        &int_rate_data.id_cr_pref_pcnt,
        &int_rate_data.nrml_int_pcnt,
        &int_rate_data.id_dr_pref_pcnt,
        &int_rate_data.base_int_tbl_code,
        &int_rate_data.base_pcnt_dr,
        &int_rate_data.base_pcnt_cr,
        &int_rate_data.base_pcnt,
        &int_rate_data.deposit_period_mths,
        &int_rate_data.deposit_period_days,
        &int_rate_data.deposit_amount,
        &int_rate_data.acct_crncy_code,
        &int_rate_data.deposit_type,
        &int_rate_data.spl_catg_ind,
        &int_rate_data.nrml_int_pcnt_cr,
        &int_rate_data.base_differential_exists,
        &int_rate_data.deposit_status,
        &int_rate_data.maturity_amount,
        &int_rate_data.maturity_date,
        &int_rate_data.rcre_time,
        &int_rate_data.auto_renewed_counter,
        overdue_flg,
        final_int_rate,
        append_data
    )
}
