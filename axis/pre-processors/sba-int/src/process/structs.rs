use rbdate::date_from_timestamp;
use rbdate::NaiveDate;

#[derive(Clone, Debug)]
pub struct SBAInt {
    pub acid: String,
    pub del_flg: String,
    pub int_slab_dr_cr_flg: String,
    pub itc_lchg_time: NaiveDate,
    pub schm_type: String,
    pub int_tbl_code: String,
    pub int_version: String,
    pub int_tbl_ver_num: String,
    pub min_int_pcnt_cr: f64,
    pub max_int_pcnt_cr: f64,
    pub cust_cr_pref_pcnt: f64,
    pub id_cr_pref_pnt: f64,
    pub nrml_int_pcnt: f64,
    pub id_dr_pref_pcnt: f64,
    pub base_int_tbl_code: String,
    pub base_pcnt_dr: f64,
    pub base_pcnt_cr: f64,
    pub base_pcnt: f64,
    pub acct_crncy_code: String,
    pub datachanged: bool,
    pub out_bal_amt: f64,
}

impl SBAInt {
    pub fn new() -> SBAInt {
        SBAInt {
            acid: "".to_string(),
            del_flg: "".to_string(),
            int_slab_dr_cr_flg: "".to_string(),
            itc_lchg_time: date_from_timestamp(0),
            schm_type: "".to_string(),
            int_tbl_code: "".to_string(),
            int_version: "".to_string(),
            int_tbl_ver_num: "".to_string(),
            min_int_pcnt_cr: 0.0,
            max_int_pcnt_cr: 0.0,
            cust_cr_pref_pcnt: 0.0,
            id_cr_pref_pnt: 0.0,
            nrml_int_pcnt: 0.0,
            id_dr_pref_pcnt: 0.0,
            base_int_tbl_code: "".to_string(),
            base_pcnt_dr: 0.0,
            base_pcnt_cr: 0.0,
            base_pcnt: 0.0,
            acct_crncy_code: "".to_string(),
            datachanged: false,
            out_bal_amt: 0.0,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct BalmIcvIvsKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
}

#[derive(Clone, Copy)]
pub struct BalmIcvVal {
    pub int_version: i64,
    pub int_tbl_ver_num: i64,
    pub base_pcnt_cr: f64,
}

#[derive(Clone, Debug, Copy)]
pub struct IvsIntVal {
    pub nrml_int_pcnt: f64,
    pub begin_slab_amt: f64,
    pub end_slab_amt: f64,
}

#[derive(Clone, Debug)]
pub struct BalmIvsVal {
    pub int_tbl_ver_num: i64,
    pub int_val: Vec<IvsIntVal>,
}
