extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acid: String,
    pub foracid: String,
    pub bacid: String,
    pub clr_bal_amt: String,
    pub un_clr_bal_amt: String,
    pub sol_id: String,
    pub cust_id: String,
    pub acct_ownership: String,
    pub ledg_num: String,
    pub drwng_power: String,
    pub mode_of_oper_code: String,
    pub lien_amt: String,
    pub sanct_lim: String,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub schm_type: String,
    pub crncy_code: String,
    pub acct_crncy_code: String,
    pub acct_cls_flg: String,
    pub del_flg: String,
    pub acct_opn_date: String,
    pub entity_cre_flg: String,
    pub acct_cls_date: String,
    pub last_tran_date: String,
    pub notional_rate_code: String,
    pub emp_id: String,
    pub notional_rate: String,
    pub limit_b2kid: String,
    pub adim1_gam: String,
    pub adim2_gam: String,
    pub adim3_gam: String,
    pub int_rate: String,
    pub bm_id: String,
    pub spread: String,
    pub reprice_freq: String,
    pub last_reprice_dt: String,
    pub next_reprice_dt: String,
    pub code1: String,
    pub code2: String,
    pub code3: String,
    pub code4: String,
    pub adim1_gac: String,
    pub adim2_gac: String,
    pub adim3_gac: String,
    pub cust_name: String,
    pub cmg_pan_gir_num: String,
    pub cmg_cust_const: String,
    pub adim1_cmg: String,
    pub adim2_cmg: String,
    pub adim3_cmg: String,
    pub out_bal_amt: String,
    pub cust_grp_id: String,
    pub ucif_cust_const: String,
    pub exch_rt: String,
    pub out_bal_amt_con: String,
    pub segment_code: String,
    pub nfs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntRateData {
    pub acid: String,
    pub del_flg: String,
    pub open_effective_date: String,
    pub schm_type: String,
    pub int_tbl_code: String,
    pub int_version: String,
    pub int_tbl_ver_num: String,
    pub min_int_pcnt_cr: String,
    pub max_int_pcnt_cr: String,
    pub cust_cr_pref_pcnt: String,
    pub id_cr_pref_pcnt: String,
    pub nrml_int_pcnt: String,
    pub id_dr_pref_pcnt: String,
    pub base_int_tbl_code: String,
    pub base_pcnt_dr: String,
    pub base_pcnt_cr: String,
    pub base_pcnt: String,
    pub deposit_period_mths: String,
    pub deposit_period_days: String,
    pub deposit_amount: String,
    pub acct_crncy_code: String,
    pub deposit_type: String,
    pub spl_catg_ind: String,
    pub nrml_int_pcnt_cr: String,
    pub base_differential_exists: String,
    pub deposit_status: String,
    pub maturity_amount: String,
    pub maturity_date: String,
    pub rcre_time: String,
    pub auto_renewed_counter: String,
}

impl IntRateData {
    pub fn new() -> IntRateData {
        IntRateData {
            acid: "0".to_string(),
            del_flg: "0".to_string(),
            open_effective_date: "0".to_string(),
            schm_type: "0".to_string(),
            int_tbl_code: "0".to_string(),
            int_version: "0".to_string(),
            int_tbl_ver_num: "0".to_string(),
            min_int_pcnt_cr: "0".to_string(),
            max_int_pcnt_cr: "0".to_string(),
            cust_cr_pref_pcnt: "0".to_string(),
            id_cr_pref_pcnt: "0".to_string(),
            nrml_int_pcnt: "0".to_string(),
            id_dr_pref_pcnt: "0".to_string(),
            base_int_tbl_code: "0".to_string(),
            base_pcnt_dr: "0".to_string(),
            base_pcnt_cr: "0".to_string(),
            base_pcnt: "0".to_string(),
            deposit_period_mths: "0".to_string(),
            deposit_period_days: "0".to_string(),
            deposit_amount: "0".to_string(),
            acct_crncy_code: "0".to_string(),
            deposit_type: "0".to_string(),
            spl_catg_ind: "0".to_string(),
            nrml_int_pcnt_cr: "0".to_string(),
            base_differential_exists: "0".to_string(),
            deposit_status: "0".to_string(),
            maturity_amount: "0".to_string(),
            maturity_date: "0".to_string(),
            rcre_time: "0".to_string(),
            auto_renewed_counter: "0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EitData {
    pub nrml_accrued_amount_cr: f64,
    pub nrml_interest_amount_cr: f64,
}

impl EitData {
    pub fn new() -> EitData {
        EitData {
            nrml_accrued_amount_cr: 0.0,
            nrml_interest_amount_cr: 0.0,
        }
    }
}
