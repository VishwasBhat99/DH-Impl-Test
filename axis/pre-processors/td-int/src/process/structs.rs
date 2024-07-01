use rbdate::date_from_timestamp;
use rbdate::NaiveDate;

#[derive(Clone, Debug)]
pub struct TdInt {
    pub del_flg: String,
    pub open_effective_date: NaiveDate,
    pub schm_type: String,
    pub int_tbl_code: String,
    pub int_version: i64,
    pub int_tbl_ver_num: i64,
    pub min_int_pcnt_cr: f64,
    pub max_int_pcnt_cr: f64,
    pub cust_cr_pref_pcnt: f64,
    pub id_cr_pref_pcnt: f64,
    pub nrml_int_pcnt: f64,
    pub id_dr_pref_pcnt: f64,
    pub base_int_tbl_code: String,
    pub base_pcnt_dr: f64,
    pub base_pcnt_cr: f64,
    pub base_pcnt: f64,
    pub deposit_period_mths: f64,
    pub deposit_period_days: f64,
    pub deposit_amount: f64,
    pub acct_crncy_code: String,
    pub deposit_type: String,
    pub spl_catg_ind: String,
    pub nrml_int_pcnt_cr: f64,
    pub base_differential_exists: String,
    pub lchg_time: NaiveDate,
    pub deposit_status: String,
    pub maturity_amount: f64,
    pub maturity_date: NaiveDate,
    pub rcre_time: NaiveDate,
    pub auto_renewed_counter: i64,
}

impl TdInt {
    pub fn new() -> TdInt {
        TdInt {
            del_flg: "".to_string(),
            open_effective_date: date_from_timestamp(0),
            schm_type: "".to_string(),
            int_tbl_code: "".to_string(),
            int_version: 0,
            int_tbl_ver_num: 0,
            min_int_pcnt_cr: 0.0,
            max_int_pcnt_cr: 0.0,
            cust_cr_pref_pcnt: 0.0,
            id_cr_pref_pcnt: 0.0,
            nrml_int_pcnt: 0.0,
            id_dr_pref_pcnt: 0.0,
            base_int_tbl_code: "".to_string(),
            base_pcnt_dr: 0.0,
            base_pcnt_cr: 0.0,
            base_pcnt: 0.0,
            deposit_period_mths: 0.0,
            deposit_period_days: 0.0,
            deposit_amount: 0.0,
            acct_crncy_code: "".to_string(),
            deposit_type: "".to_string(),
            spl_catg_ind: "".to_string(),
            nrml_int_pcnt_cr: 0.0,
            base_differential_exists: "".to_string(),
            lchg_time: date_from_timestamp(0),
            deposit_status: "0".to_string(),
            maturity_amount: 0.0,
            maturity_date: date_from_timestamp(0),
            rcre_time: date_from_timestamp(0),
            auto_renewed_counter: 0,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub struct BalmIcv {
    pub int_tbl_code: String,
    pub crncy_code: String,
}

#[derive(Clone, Copy)]
pub struct BalmIcvVal {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub int_version: i64,
    pub int_tbl_ver_num: i64,
    pub base_pcnt_cr: f64,
}
