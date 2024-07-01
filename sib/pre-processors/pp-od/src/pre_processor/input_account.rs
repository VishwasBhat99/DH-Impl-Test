extern crate serde;
extern crate serde_derive;
use chrono::NaiveDate;

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
pub struct RateCode {
    pub intrate_code: String,
    pub base_rate: String,
    pub fixed_floating: String,
    pub benchmark: String,
}

impl RateCode {
    pub fn new() -> RateCode {
        RateCode {
            intrate_code: "NA".to_string(),
            base_rate: "NA".to_string(),
            fixed_floating: "NA".to_string(),
            benchmark: "NA".to_string(),
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
#[derive(Debug, Clone, Default)]
pub struct AdditionLoanFile {
    pub gnt_type: String,
    pub status_code: String,
    pub occupation_code: String,
    pub sector: String,
    pub sector_code: String,
    pub subsector_code: String,
    pub staffflag: String,
    pub cre_free_text_1: String,
    pub pres_val_sec:f64,
    pub paripassu_perc:f64,
    pub prov_percent: f64,
    pub dumm2: f64,
    pub dumm3: f64,
    pub dumm4: f64,
    pub dumm5: String,
    pub dumm6: String,
    pub dumm7: String,
    pub dumm8: String,
    pub dumm9: NaiveDate,
    pub dumm10: NaiveDate,
    pub const_catgory_code: String,
    pub rating_agc: String,
    pub rating: String,
    pub super_annuation_flag: String,
    pub turn_amt1: f64,
    pub turn_amt2: f64,
    pub turn_amt_3: f64,
    pub ftp_char1: String,
    pub ftp_char2: String,
    pub ftp_amt1: f64,
    pub ftp_amt2: f64,
    pub ftp_date1: NaiveDate,
    pub ftp_date2: NaiveDate,
}

impl AdditionLoanFile {
    pub fn new() -> AdditionLoanFile {
     AdditionLoanFile {
       gnt_type: "NA".to_string(),
      status_code: "NA".to_string(),
      occupation_code: "NA".to_string(),
      sector: "NA".to_string(),
      sector_code: "NA".to_string(),
      subsector_code: "NA".to_string(),
      staffflag: "NA".to_string(),
      cre_free_text_1: "NA".to_string(),
      pres_val_sec:0.0,
      paripassu_perc:0.0,
      prov_percent: 0.0,
      dumm2: 0.0,
      dumm3: 0.0,
      dumm4: 0.0,
      dumm5: "NA".to_string(),
      dumm6: "NA".to_string(),
      dumm7: "NA".to_string(),
      dumm8: "NA".to_string(),
      dumm9: NaiveDate::from_ymd(1970, 1, 1),
      dumm10: NaiveDate::from_ymd(1970, 1, 1),
      const_catgory_code: "NA".to_string(),
      rating_agc: "NA".to_string(),
      rating: "NA".to_string(),
      super_annuation_flag: "NA".to_string(),
      turn_amt1: 0.0,
      turn_amt2: 0.0,
      turn_amt_3: 0.0,
      ftp_char1: "NA".to_string(),
      ftp_char2: "NA".to_string(),
      ftp_amt1: 0.0,
      ftp_amt2: 0.0,
      ftp_date1: NaiveDate::from_ymd(1970, 1, 1),
      ftp_date2: NaiveDate::from_ymd(1970, 1, 1),
       }
    }
}