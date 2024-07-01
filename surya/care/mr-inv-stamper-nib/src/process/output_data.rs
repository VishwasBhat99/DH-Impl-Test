use super::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputData {
    pub as_on_date: String,
    pub deal_id: String,
    pub src_sys_code: String,
    pub llg_id: String,
    pub isin: String,
    pub inv_type: String,
    pub inv_desc: String,
    pub category: String,
    pub face_val_ccy: f64,
    pub face_val_hcy: f64,
    pub book_val_ccy: f64,
    pub book_val_hcy: f64,
    pub market_val_hcy: f64,
    pub ccy_id: String,
    pub exch_rate: String,
    pub gl_cd_1: String,
    pub gl_cd_2: String,
    pub gl_cd_3: String,
    pub issuer_id: String,
    pub issuer_name: String,
    pub issuer_type: String,
    pub sub_date: i64,
    pub val_or_settle_date: i64,
    pub mat_date: i64,
    pub res_days: String,
    pub coup_rate: String,
    pub coup_freq_code: String,
    pub coup_basis: String,
    pub guranteed_by: String,
    pub applied_rating_id: String,
    pub internal_rating: String,
    pub external_rating_agenecy: String,
    pub external_rating: String,
    pub inst_purpose: String,
    pub spec_risk_cap_rule_id: String,
    pub spec_risk_cap_prcnt: String,
    pub gen_mr_rule_id: String,
    pub gen_mr_rule_prcnt: String,
    pub spec_charge_amt_hcy: String,
    pub gen_market_risk_amt_hcy: String,
    pub tot_mr_amt_hcy: String,
    pub add_prcnt_1: String,
    pub add_prcnt_2: String,
    pub add_prcnt_3: String,
    pub add_prcnt_4: String,
    pub add_prcnt_5: String,
    pub add_date_1: String,
    pub add_date_2: String,
    pub add_date_3: String,
    pub add_date_4: String,
    pub add_date_5: String,
    pub add_str_1: String,
    pub add_str_2: String,
    pub add_str_3: String,
    pub add_str_4: String,
    pub add_str_5: String,
}

impl Display for OutputData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.as_on_date,
            self.deal_id,
            self.src_sys_code,
            self.llg_id,
            self.isin,
            self.inv_type,
            self.inv_desc,
            self.category,
            self.face_val_ccy,
            self.face_val_hcy,
            self.book_val_ccy,
            self.book_val_hcy,
            self.market_val_hcy,
            self.ccy_id,
            self.exch_rate,
            self.gl_cd_1,
            self.gl_cd_2,
            self.gl_cd_3,
            self.issuer_id,
            self.issuer_name,
            self.issuer_type,
            naivedate_from_timestamp(self.sub_date).format("%d-%m-%Y").to_string(),
            naivedate_from_timestamp(self.val_or_settle_date).format("%d-%m-%Y").to_string(),
            naivedate_from_timestamp(self.mat_date).format("%d-%m-%Y").to_string(),
            self.res_days,
            self.coup_rate,
            self.coup_freq_code,
            self.coup_basis,
            self.guranteed_by,
            self.applied_rating_id,
            self.internal_rating,
            self.external_rating_agenecy,
            self.external_rating,
            self.inst_purpose,
            self.spec_risk_cap_rule_id,
            self.spec_risk_cap_prcnt,
            self.gen_mr_rule_id,
            self.gen_mr_rule_prcnt,
            self.spec_charge_amt_hcy,
            self.gen_market_risk_amt_hcy,
            self.tot_mr_amt_hcy,
            self.add_prcnt_1,
            self.add_prcnt_2,
            self.add_prcnt_3,
            self.add_prcnt_4,
            self.add_prcnt_5,
            self.add_date_1,
            self.add_date_2,
            self.add_date_3,
            self.add_date_4,
            self.add_date_5,
            self.add_str_1,
            self.add_str_2,
            self.add_str_3,
            self.add_str_4,
            self.add_str_5,
        )
    }
}

pub fn get_op_data(
    account: &AccountWithCFs,
    acc_keys: &AccFieldNames,
    llg_id: i32,
    spec_risk_prcnt: String,
    gen_mr_prcnt: String,
    face_val_hcy: f64,
    face_val_ccy: f64,
    book_val_hcy: f64,
    book_val_ccy: f64,
    market_val_hcy: f64,
    exchange_rate: f64,
    config_params: &ConfigurationParameters,
) -> OutputData {
    let spec_risk_cap_rule_id = spec_risk_prcnt[0..4].to_string();
    let spec_risk_cap_val = spec_risk_prcnt[4..].to_string();
    let spec_risk_cap_prcnt = spec_risk_cap_val.parse::<f64>().unwrap_or(0.0) / 100.0;
    let gen_mr_prcnt_rule_id = gen_mr_prcnt[0..4].to_string();
    let gen_mr_prcnt_val = gen_mr_prcnt[4..].to_string();
    let gen_mr_prcnt = gen_mr_prcnt_val.parse::<f64>().unwrap_or(0.0) / 100.0;
    let spec_charge_amt_hcy = (market_val_hcy * spec_risk_cap_prcnt) / 100.0;
    let gen_market_risk_amt_hcy = (market_val_hcy * gen_mr_prcnt) / 100.0;
    let tot_mr_amt_hcy = spec_charge_amt_hcy + gen_market_risk_amt_hcy;
    OutputData {
        as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
        deal_id: match account.get_string_for_key(&acc_keys.deal_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        src_sys_code: match account.get_string_for_key(&acc_keys.src_sys_code) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        llg_id: llg_id.to_string(),
        isin: match account.get_string_for_key(&acc_keys.isin) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        inv_type: match account.get_string_for_key(&acc_keys.inv_type) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        inv_desc: match account.get_string_for_key(&acc_keys.inv_desc) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        category: match account.get_string_for_key(&acc_keys.category) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        face_val_ccy: face_val_ccy,
        face_val_hcy: face_val_hcy,
        book_val_ccy: book_val_ccy,
        book_val_hcy: book_val_hcy,
        market_val_hcy: market_val_hcy,
        ccy_id: match account.get_string_for_key(&acc_keys.ccy_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        exch_rate: exchange_rate.to_string(),
        gl_cd_1: match account.get_string_for_key(&acc_keys.gl_cd_1) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        gl_cd_2: match account.get_string_for_key(&acc_keys.gl_cd_2) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        gl_cd_3: match account.get_string_for_key(&acc_keys.gl_cd_3) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        issuer_id: match account.get_string_for_key(&acc_keys.issuer_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        issuer_name: match account.get_string_for_key(&acc_keys.issuer_name) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        issuer_type: match account.get_string_for_key(&acc_keys.issuer_type) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        sub_date: match account.get_i64_for_key(&acc_keys.sub_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        val_or_settle_date: match account.get_i64_for_key(&acc_keys.val_or_settle_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        mat_date: match account.get_i64_for_key(&acc_keys.mat_date) {
            Ok(val) => val,
            Err(_) => 0,
        },
        res_days: match account.get_string_for_key(&acc_keys.res_days) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        coup_rate: match account.get_string_for_key(&acc_keys.coup_rate) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        coup_freq_code: match account.get_string_for_key(&acc_keys.coup_freq_code) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        coup_basis: match account.get_string_for_key(&acc_keys.coup_basis) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        guranteed_by: match account.get_string_for_key(&acc_keys.guranteed_by) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        applied_rating_id: match account.get_string_for_key(&acc_keys.applied_rating_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        internal_rating: match account.get_string_for_key(&acc_keys.internal_rating) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        external_rating_agenecy: match account.get_string_for_key(&acc_keys.external_rating_agenecy)
        {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        external_rating: match account.get_string_for_key(&acc_keys.external_rating) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        inst_purpose: match account.get_string_for_key(&acc_keys.inst_purpose) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        },
        spec_risk_cap_rule_id: spec_risk_cap_rule_id,
        spec_risk_cap_prcnt: spec_risk_cap_prcnt.to_string(),
        gen_mr_rule_id: gen_mr_prcnt_rule_id,
        gen_mr_rule_prcnt: gen_mr_prcnt.to_string(),
        spec_charge_amt_hcy: spec_charge_amt_hcy.to_string(),
        gen_market_risk_amt_hcy: gen_market_risk_amt_hcy.to_string(),
        tot_mr_amt_hcy: tot_mr_amt_hcy.to_string(),
        add_prcnt_1: "".to_string(),
        add_prcnt_2: "".to_string(),
        add_prcnt_3: "".to_string(),
        add_prcnt_4: "".to_string(),
        add_prcnt_5: "".to_string(),
        add_date_1: "".to_string(),
        add_date_2: "".to_string(),
        add_date_3: "".to_string(),
        add_date_4: "".to_string(),
        add_date_5: "".to_string(),
        add_str_1: "".to_string(),
        add_str_2: "".to_string(),
        add_str_3: "".to_string(),
        add_str_4: "".to_string(),
        add_str_5: "".to_string(),
    }
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
