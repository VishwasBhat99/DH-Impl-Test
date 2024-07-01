extern crate chrono;

use super::input_account::InputAccount;
use super::input_account::MasterSheetAccount;
use rbdate::datevalue_to_naive_date;
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &InputAccount,
    as_on_date: NaiveDate,
    master_map: &HashMap<String, MasterSheetAccount>,
    date_format: String,
) -> String {
    let asondate_timestamp = as_on_date.and_hms(0, 0, 0).timestamp();
    let asondate_datevalue = (asondate_timestamp / 86400) + 25568;
    let acc_open_datevalue = acc.acc_open_dt.parse::<i64>().unwrap_or(asondate_datevalue) + 1;
    let acc_open_date = datevalue_to_naive_date(&acc_open_datevalue.to_string())
        .unwrap()
        .format("%d-%m-%Y");
    let mat_dt = NaiveDate::parse_from_str(acc.mat_dt.as_str(), &date_format).unwrap_or(as_on_date);
    let mat_date = mat_dt.format("%d-%m-%Y").to_string();
    let nego_dt =
        NaiveDate::parse_from_str(acc.nego_strt_dt.as_str(), &date_format).unwrap_or(as_on_date);
    let nego_strt_date = nego_dt.format("%d-%m-%Y").to_string();
    let ason_date = as_on_date.format("%d-%m-%Y").to_string();
    let mut default_master_acc = MasterSheetAccount::new();
    default_master_acc.gl_acc_no = "NONE".to_string();
    default_master_acc.classification = "NONE".to_string();
    default_master_acc.logic = "NONE".to_string();
    default_master_acc.description = "NONE".to_string();
    default_master_acc.other_llg_classification = "NONE".to_string();
    default_master_acc.group = "NONE".to_string();
    default_master_acc.llg = "NONE".to_string();

    let input_master_acc = master_map.get(&acc.gl_cd).unwrap_or(&default_master_acc);
    let master_grp = &input_master_acc.group;
    let master_llg = &input_master_acc.llg;
    let master_class = &input_master_acc.classification;
    let mut master_classification = String::new();
    if master_map.contains_key(&acc.gl_cd) {
        master_classification = master_class[0..1].to_string();
    } else {
        master_classification = "NONE".to_string();
    }
    format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||0.0|0.0|0.0|{}|{}|{}|{}",
    &acc.acc_no,
    &acc.bill_id,
    &acc.branch_cd,
    &acc.cust_no,
    &acc.ucc_id,
    &acc.ccy,
    &acc.gl_cd,
    &acc.prod_cd,
    &acc_open_date,
    &acc.curr_out_bal,
    &acc.curr_out_bal_lcy,
    &acc.original_bill_amt,
    &acc.mat_amt,
    &acc.int_rt,
    &mat_date,
    &nego_strt_date,
    &acc.int_accured,
    &acc.int_realised,
    &acc.ext_rt_agency_id,
    &acc.cust_rt_ext,
    &acc.cust_rt_int,
    &acc.npa_class,
    &acc.prov_amt,
    &acc.prov_dt,
    &acc.cust_const_cd,
    &master_grp,
    &master_llg,
    master_classification,
    ason_date,
    ason_date,
    ason_date,
    ason_date,
    )
}
