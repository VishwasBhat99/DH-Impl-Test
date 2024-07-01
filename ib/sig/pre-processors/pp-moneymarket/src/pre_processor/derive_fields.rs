use super::input_account::InputAccount;
use crate::pre_processor::MasterData;
use chrono::Datelike;
use rbdate::NaiveDate;
use std::collections::HashMap;

pub fn get_op_line(
    acc: &InputAccount,
    as_on_date: NaiveDate,
    master_data: &mut HashMap<String, MasterData>,
) -> String {
    let concat = format!(
        "{}{}{}{}",
        acc.acct_suffix, acc.cust_no, acc.gl_code, acc.cust_type
    );
    let ason_date = as_on_date.format("%d-%m-%Y").to_string();
    let rpt_date = NaiveDate::parse_from_str(acc.rpt_dt.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let acc_rpt_dt = rpt_date.format("%d-%m-%Y").to_string();
    let v_date = NaiveDate::parse_from_str(acc.vdate.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let v_dt = v_date.format("%d-%m-%Y").to_string();
    let m_date = NaiveDate::parse_from_str(acc.mdate.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let m_dt = m_date.format("%d-%m-%Y").to_string();
    let r_date = NaiveDate::parse_from_str(acc.rdate.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let r_dt = r_date.format("%d-%m-%Y").to_string();
    let deal_dt = acc.deal_dt.to_owned();
    let dealdt = NaiveDate::parse_from_str(&deal_dt, "%d/%m/%Y %r").unwrap_or(as_on_date);
    let dealdate = dealdt.format("%d-%m-%Y").to_string();
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||||{}|{}|{}||",
        acc_rpt_dt,
        acc.br,
        acc.dealno,
        acc.asset_liability,
        acc.cust_no,
        acc.cust,
        acc.prodtype,
        dealdate,
        v_dt,
        m_dt,
        r_dt,
        acc.ccy,
        acc.principal,
        acc.rate,
        acc.amt_gd,
        acc.int_rt,
        acc.accr_int,
        acc.gl_code,
        acc.cust_type,
        acc.brca,
        acc.acct_suffix,
        ason_date,
        concat,
        master_data
            .get(&concat)
            .unwrap_or(&MasterData::def())
            .vs_param,
        master_data
            .get(&concat)
            .unwrap_or(&MasterData::def())
            .vg_param,
    )
}
