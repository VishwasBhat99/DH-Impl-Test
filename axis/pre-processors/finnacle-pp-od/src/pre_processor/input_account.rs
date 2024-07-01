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
    pub int_slab_dr_cr_flg: String,
    pub itc_lchg_time: String,
    pub schm_type: String,
    pub int_tbl_code: String,
    pub int_tbl_code_srl_num: String,
    pub icv_int_tbl_ver_num: String,
    pub int_tbl_ver_num: String,
    pub min_int_pcnt: String,
    pub max_int_pcnt: String,
    pub cust_pref_pcnt: String,
    pub id_pref_pcnt: String,
    pub nrml_int_pcnt: String,
    pub base_int_tbl_code: String,
    pub base_pcnt: String,
    pub acct_crncy_code: String,
    pub datachanged: String,
    pub end_date: String,
    pub pegged_flg: String,
    pub npa_classification: String,
    pub npa_amount: String,
    pub foracid: String,
    pub cust_id: String,
    pub gam_last_tran_date: String,
    pub gam_clr_bal_amt: String,
}

impl IntRateData {
    pub fn new() -> IntRateData {
        IntRateData {
            acid: "0".to_string(),
            del_flg: "0".to_string(),
            int_slab_dr_cr_flg: "0".to_string(),
            itc_lchg_time: "0".to_string(),
            schm_type: "0".to_string(),
            int_tbl_code: "".to_string(),
            int_tbl_code_srl_num: "0".to_string(),
            icv_int_tbl_ver_num: "0".to_string(),
            int_tbl_ver_num: "0".to_string(),
            min_int_pcnt: "".to_string(),
            max_int_pcnt: "".to_string(),
            cust_pref_pcnt: "0".to_string(),
            id_pref_pcnt: "0".to_string(),
            nrml_int_pcnt: "0".to_string(),
            base_int_tbl_code: "0".to_string(),
            base_pcnt: "0".to_string(),
            acct_crncy_code: "0".to_string(),
            datachanged: "0".to_string(),
            end_date: "0".to_string(),
            pegged_flg: "Y".to_string(),
            npa_classification: "0".to_string(),
            npa_amount: "0".to_string(),
            foracid: "0".to_string(),
            cust_id: "0".to_string(),
            gam_last_tran_date: "0".to_string(),
            gam_clr_bal_amt: "0".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Benchmark {
    pub acid: String,
    pub foracid: String,
    pub repricing_plan: String,
    pub floating_type: String,
    pub next_repricing_date: String,
}

impl Benchmark {
    pub fn new() -> Benchmark {
        Benchmark {
            acid: "0".to_string(),
            foracid: "0".to_string(),
            repricing_plan: "".to_string(),
            floating_type: "".to_string(),
            next_repricing_date: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TblCodes {
    pub int_tbl_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NPAData {
    pub npa_classification: String,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
    pub npa_amount: String,
}
impl NPAData {
    pub fn new() -> NPAData {
        NPAData {
            npa_classification: "0".to_string(),
            cust_hlth_code: "0".to_string(),
            cust_npa_class: "0".to_string(),
            final_npa_class: "0".to_string(),
            npa_amount: "0".to_string(),
        }
    }
}
