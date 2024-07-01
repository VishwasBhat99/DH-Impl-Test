use chrono::NaiveDate;
use rbdate::date_from_timestamp;

#[derive(Clone, Debug)]
pub struct OdInt {
    pub del_flg: String,
    pub int_slab_dr_cr_flg: String,
    pub itc_lchg_time: NaiveDate,
    pub schm_type: String,
    pub int_tbl_code: String,
    pub int_tbl_code_srl_num: i64,
    pub icv_int_tbl_ver_num: String,
    pub int_tbl_ver_num: i64,
    pub min_int_pcnt: f64,
    pub max_int_pcnt: f64,
    pub cust_pref_pcnt: f64,
    pub id_pref_pcnt: f64,
    pub nrml_int_pcnt: f64,
    pub base_int_tbl_code: i64,
    pub base_pcnt: f64,
    pub acct_crncy_code: String,
    pub datachanged: String,
    pub end_date: String,
    pub pegged_flg: String,
    pub npa_classification: String,
    pub npa_amount: String,
    pub foracid: String,
    pub cust_id: String,
    pub gam_last_tran_date: NaiveDate,
    pub gam_clr_bal_amt: f64,
}

impl OdInt {
    pub fn new() -> OdInt {
        OdInt {
            del_flg: "".to_string(),
            int_slab_dr_cr_flg: "".to_string(),
            itc_lchg_time: date_from_timestamp(0),
            schm_type: "".to_string(),
            int_tbl_code: "".to_string(),
            int_tbl_code_srl_num: 0,
            icv_int_tbl_ver_num: "0".to_string(),
            int_tbl_ver_num: 0,
            min_int_pcnt: 0.0,
            max_int_pcnt: 0.0,
            cust_pref_pcnt: 0.0,
            id_pref_pcnt: 0.0,
            nrml_int_pcnt: 0.0,
            base_int_tbl_code: 0,
            base_pcnt: 0.0,
            acct_crncy_code: "".to_string(),
            datachanged: "".to_string(),
            end_date: date_from_timestamp(0).to_string(),
            pegged_flg: "".to_string(),
            npa_classification: "".to_string(),
            npa_amount: "".to_string(),
            foracid: "".to_string(),
            cust_id: "".to_string(),
            gam_last_tran_date: date_from_timestamp(0),
            gam_clr_bal_amt: 0.0,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub struct BalmIcvKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
}

#[derive(Clone, Debug)]
pub struct BalmIcvVal {
    pub icv_int_tbl_ver_num: String,
    pub int_tbl_ver_num: i64,
    pub base_pcnt: f64,
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub struct IvsKey {
    pub int_tbl_code: String,
    pub int_tbl_ver_num: i64,
    pub crncy_code: String,
}
#[derive(Clone, Copy)]
pub struct IvsVal {
    pub nrml_int_pcnt: f64,
    pub int_slab_srl_no: i64,
    pub end_slab_amt: f64,
}
