use crate::configuration_parameters::ConfigurationParameters;

use super::account_field_names::AccFieldNames;
use super::structs::CRMData;
use process::get_haircut_prnct::get_haircut_prnct;
use rbdate::{NaiveDate, NaiveDateTime};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_crm_data(
    account: &AccountWithCFs,
    acc_keys: &AccFieldNames,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> CRMData {
    // get haircut percentage
    let haircut_prnct: f64 = get_haircut_prnct(&account, &rules)
        .to_string()
        .parse()
        .unwrap_or(0.0);
    // read fields
    let col_id = match account.get_i64_for_key(&acc_keys.col_id) {
        Ok(val) => val,
        Err(_) => 0,
    };
    let exp_acc_no = match account.get_string_for_key(&acc_keys.exp_acc_no) {
        Ok(val) => val.to_string(),
        Err(_) => format!(""),
    };
    let cust_no = match account.get_i64_for_key(&acc_keys.cust_no) {
        Ok(val) => val,
        Err(_) => 0,
    };
    let col_type = match account.get_string_for_key(&acc_keys.col_type) {
        Ok(val) => val.to_string(),
        Err(_) => format!(""),
    };
    let tot_col_val_lcy = match account.get_f64_for_key(&acc_keys.tot_col_val_lcy) {
        Ok(val) => val,
        Err(_) => 0.0,
    };
    let col_ccy = match account.get_string_for_key(&acc_keys.col_ccy) {
        Ok(val) => val.to_string(),
        Err(_) => format!(""),
    };
    let current_col_val_lcy = match account.get_f64_for_key(&acc_keys.current_col_val_lcy) {
        Ok(val) => val,
        Err(_) => 0.0,
    };
    let mat_date_col = match account.get_i64_for_key(&acc_keys.mat_date_col) {
        Ok(val) => val,
        Err(_) => 0,
    };
    let is_eligible_col = "Y".to_string();
    let haircut_amt = (current_col_val_lcy * haircut_prnct) / 100.0;
    let col_val_aftr_haircut = current_col_val_lcy - haircut_amt;

    let mut out_acc = CRMData::new();

    let rev_col_val_aftr_haircut = if config_params.rev_col_haircut_flag() {
        if tot_col_val_lcy - col_val_aftr_haircut > 0.0 {
            tot_col_val_lcy - col_val_aftr_haircut
        } else {
            0.0
        }
    } else {
        col_val_aftr_haircut
    };

    out_acc.col_id = col_id;
    out_acc.exp_acc_no = exp_acc_no;
    out_acc.cust_no = cust_no;
    out_acc.col_type = col_type;
    out_acc.tot_col_val_lcy = tot_col_val_lcy;
    out_acc.col_ccy = col_ccy;
    out_acc.current_val_col_lcy = current_col_val_lcy;
    out_acc.mat_date_col = naivedate_from_timestamp(mat_date_col)
        .format("%d-%m-%Y")
        .to_string();
    out_acc.is_eligible = is_eligible_col;
    out_acc.haircut_prcnt = haircut_prnct;
    out_acc.haircut_amt = haircut_amt;
    out_acc.col_val_aftr_haircut = rev_col_val_aftr_haircut;

    out_acc
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
