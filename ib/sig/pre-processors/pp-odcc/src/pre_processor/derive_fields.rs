use super::{input_account::InputAccount, RepData};
use crate::pre_processor::MasterData;
use chrono::Datelike;
use rbdate::NaiveDate;
use slog::Logger;
use std::{collections::HashMap, str::FromStr};

use macros;

pub fn get_repricing_freq(rep_freq: &str, account_open_date: NaiveDate) -> NaiveDate {
    let next_rep_date = match rep_freq {
        "A" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 12)
            .expect("Cannot derive `next repricing date`."),
        "M" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 1)
            .expect("Cannot derive `next repricing date`."),
        "B" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 2)
            .expect("Cannot derive `next repricing date`."),
        "Q" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 3)
            .expect("Cannot derive `next repricing date`."),
        "H" => rbdate::incr_dt_by_mon_presrv_eom(account_open_date, 6)
            .expect("Cannot derive `next repricing date`."),
        _ => panic!("cannot get repricing frequency for `account open date: {}`",account_open_date),
    };
    return next_rep_date;
}

pub fn get_op_line(
    out_acc: &InputAccount,
    as_on_date: NaiveDate,
    master_data: &mut HashMap<String, MasterData>,
    repricing_map: &HashMap<String, RepData>,
    log: &Logger,
) -> String {
    let asondate = as_on_date.format("%d-%m-%Y").to_string();
    let rpt_date =
        NaiveDate::parse_from_str(out_acc.rpt_dt.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let acc_rpt_dt = rpt_date.format("%d-%m-%Y").to_string();
    let open_date =
        NaiveDate::parse_from_str(out_acc.open_dt.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let acc_open_dt = open_date.format("%d-%m-%Y").to_string();
    let lim_exp_date =
        NaiveDate::parse_from_str(out_acc.lim_exp_dt.as_str(), "%Y-%m-%d").unwrap_or(as_on_date);
    let acc_lim_exp_dt = lim_exp_date.format("%d-%m-%Y").to_string();

    let acc_int_frm_dt = NaiveDate::parse_from_str(&out_acc.int_from_dt.as_str(), "%Y-%m-%d")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let acc_int_to_dt = NaiveDate::parse_from_str(out_acc.int_to_dt.as_str(), "%Y-%m-%d")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let acc_lst_cust_txn_dt =
        NaiveDate::parse_from_str(out_acc.lst_cust_txn_dt.as_str(), "%Y-%m-%d")
            .unwrap_or(as_on_date)
            .format("%d-%m-%Y")
            .to_string();
    let acc_npa_dt = NaiveDate::parse_from_str(out_acc.npa_dt.as_str(), "%Y-%m-%d")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let acc_subv_end_dt = NaiveDate::parse_from_str(out_acc.subv_end_dt.as_str(), "%Y-%m-%d")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let acc_sae_end_dt = NaiveDate::parse_from_str(out_acc.sae_end_dt.as_str(), "%Y-%m-%d")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let acc_subsidy_apprn_dt =
        NaiveDate::parse_from_str(out_acc.subsidy_apprn_dt.as_str(), "%Y-%m-%d")
            .unwrap_or(as_on_date)
            .format("%d-%m-%Y")
            .to_string();
    let acc_lst_int_dr_dt = NaiveDate::parse_from_str(out_acc.last_int_dr_dt.as_str(), "%Y-%m-%d")
        .unwrap_or(as_on_date)
        .format("%d-%m-%Y")
        .to_string();
    let concat = format!(
        "{}{}{}{}",
        out_acc.acct_suffix, out_acc.cust_typ, out_acc.gl_code, out_acc.src_sys_cd
    );
    let binding = MasterData::def();
    let vs_code = &master_data.get(&concat).unwrap_or(&binding).vs_param;
    let mut acc_mclr_int_strt_dt= NaiveDate::parse_from_str(out_acc.mclr_int_strt_dt.as_str(), "%Y-%m-%d")
    .unwrap_or(as_on_date)
    .format("%d-%m-%Y")
    .to_string();
    if repricing_map.contains_key(vs_code) {
        let rep_data = match repricing_map.get(vs_code) {
            Some(rep_data) => rep_data,
            _ => panic!("vsa codes is not present in repricing master file"),
        };
        let mut repricing_date=open_date;
        if !rep_data.repricing_day.is_empty() {
            let month = open_date.month();
            let year = open_date.year();
            let day=&rep_data.repricing_day;
            let day_int = day.parse::<u32>().unwrap();
            repricing_date = NaiveDate::from_ymd(year, month, day_int);
        } 
        while repricing_date <= as_on_date {
            repricing_date = get_repricing_freq(&rep_data.repricing_freq, repricing_date);
        }
        acc_mclr_int_strt_dt =
            NaiveDate::parse_from_str(&repricing_date.to_string(), "%Y-%m-%d")
                .unwrap_or(as_on_date)
                .format("%d-%m-%Y")
                .to_string();
    }
    format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||||{}|{}|{}||",
    acc_rpt_dt,
    out_acc.core_acc_typ_cd,
    out_acc.core_acc_status_cd,
    out_acc.int_period_cnt,
    out_acc.overdraft_priviledges_flg,
    out_acc.overdraft_flg,
    out_acc.overdraft_limit_amt,
    out_acc.acc_id,
    out_acc.src_sys_cd,
    out_acc.min_acc_open_amt,
    out_acc.req_min_bal_amt,
    out_acc.bureau_class_cd,
    out_acc.late_payment_days_late_no,
    out_acc.late_payment_status_cd,
    out_acc.sector_cd,
    out_acc.base_rate_cat,
    out_acc.npa_provision,
    out_acc.special_int_rt,
    out_acc.rt_of_int,
    out_acc.day_basis_cd,
    out_acc.prd_cd,
    out_acc.acc_typ,
    out_acc.deposit_id,
    out_acc.branch_code,
    out_acc.cust_id,
    out_acc.cust_typ,
    out_acc.ccy_code,
    out_acc.gl_code,
    out_acc.out_bal,
    out_acc.accrued_int_amt,
    acc_int_frm_dt,
    acc_int_to_dt,
    acc_open_dt,
    acc_lst_cust_txn_dt,
    acc_npa_dt,
    out_acc.npa_flg,
    out_acc.rating_code,
    out_acc.ib_code,
    out_acc.mclr_freq,
    acc_mclr_int_strt_dt,
    out_acc.ccy,
    out_acc.prin_bal_amt,
    out_acc.sub_amt_rec,
    acc_subv_end_dt,
    acc_sae_end_dt,
    acc_lim_exp_dt,
    acc_subsidy_apprn_dt,
    acc_lst_int_dr_dt,
    out_acc.acct_suffix,
    asondate,
    concat,
    master_data.get(&concat).unwrap_or(&MasterData::def()).vs_param,
    master_data.get(&concat).unwrap_or(&MasterData::def()).vg_param,
    )
}
