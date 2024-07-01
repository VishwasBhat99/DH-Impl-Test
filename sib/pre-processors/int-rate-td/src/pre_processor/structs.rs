use rbdate::NaiveDate;

#[derive(Debug)]
pub struct IntData {
    pub v_entity: String,
    pub v_schmtype: String,
    pub v_schmcode: String,
    pub v_crncy: String,
    pub v_opndate: NaiveDate,
    pub v_clsdate: NaiveDate,
    pub v_depamt: f64,
    pub v_perdmths: i64,
    pub v_perddays: i64,
    pub v_opneffdate: NaiveDate,
    pub v_matdate: NaiveDate,
    pub v_credt: NaiveDate,
    pub int_tbl_code_srl_num: i64,
    pub v_inttbl: String,
    pub v_crpref: f64,
    pub v_drpref: i64,
    pub v_passdt: NaiveDate,
    pub tam_deposit_type: String,
    pub tam_spl_catg_ind: String,
    pub tam_deposit_status: String,
    pub tam_auto_renewed_counter: i64,
    pub itc_min_int_pcnt_cr: f64,
    pub itc_max_int_pcnt_cr: f64,
    pub itc_nrml_int_pcnt: f64,
    pub itc_base_differential: String,
}

#[derive(Debug)]
pub struct RhtData {
    pub deposit_amount: f64,
    pub deposit_period_mths: i64,
    pub deposit_period_days: i64,
    pub open_effective_date: NaiveDate,
    pub ren_srl_num: i64,
    pub maturiy_date: NaiveDate,
    pub rcre_time: NaiveDate,
}

#[derive(Debug)]
pub struct ITCData {
    pub int_tbl_code: String,
    pub int_tbl_ver_num: i64,
    pub id_cr_pref_pcnt: f64,
    pub id_dr_pref_pcnt: f64,
    pub cust_dr_pref_pcnt: f64,
    pub int_tbl_code_srl_num: String,
    pub min_int_pcnt_dr: f64,
    pub max_int_pcnt_dr: f64,
}

impl Default for ITCData {
    fn default() -> ITCData {
        ITCData {
            int_tbl_code: "".to_string(),
            int_tbl_ver_num: 0,
            id_cr_pref_pcnt: 0.0,
            id_dr_pref_pcnt: 0.0,
            cust_dr_pref_pcnt: 0.0,
            int_tbl_code_srl_num: "".to_string(),
            min_int_pcnt_dr: 0.0,
            max_int_pcnt_dr: 0.0,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IcvKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
}

#[derive(Debug, Clone)]
pub struct IcvValue {
    pub lchg_time: NaiveDate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub int_tbl_ver_num: i64,
    pub int_version: i64,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TvsKey {
    pub int_tbl_code: String,
    pub crncy_code: String,
    pub int_tbl_ver_num: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TvsValue {
    pub max_period_run_days: i64,
    pub max_period_run_mths: i64,
    pub max_slab_amt: f64,
    pub int_slab_srl_num: i64,
    pub nrml_int_pcnt: f64,
}
