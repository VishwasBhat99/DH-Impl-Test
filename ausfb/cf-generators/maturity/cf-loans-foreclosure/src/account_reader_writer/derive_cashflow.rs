use chrono::{Datelike, NaiveDate};
use macros;
use rbdate::timestamp;
use slog::Logger;
use std::collections::HashMap;

use super::{
    account_reader::input_account::InputAccount, account_with_cashflows::Cashflow,
    structs::CashflowData,
};

pub fn get_cashflows(
    cashflow_vec: &Vec<CashflowData>,
    inp_acc: &InputAccount,
    as_on_date:&NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut int_freq_vec = Vec::new();
    let mut prev_date = cashflow_vec[0].flow_date;
    let prin_flow = cashflow_vec[0].principal_flow;
    let int_flow = cashflow_vec[0].interest_flow;
    let mut prin_check = true;
    let mut int_check = true;
    let mut check_index = 0;
    for cashflows in cashflow_vec.into_iter().skip(1) {
        check_index += 1;
        if check_index < cashflow_vec.len() - 1 {
            if prin_flow != cashflows.principal_flow {
                prin_check = false;
            }
            if int_flow != cashflows.interest_flow {
                int_check = false;
            }
        }
        let months = months_between_dates(prev_date, cashflows.flow_date);
        int_freq_vec.push(months);
        prev_date = cashflows.flow_date;
    }
    let mut frequency_map = HashMap::new();
    for value in int_freq_vec.iter() {
        *frequency_map.entry(value).or_insert(0) += 1;
    }
    let max_int_freq = frequency_map.keys().max().cloned().unwrap_or(&0);

    let int_month = match max_int_freq {
        1 => 12,
        3 => 4,
        6 => 2,
        12 => 1,
        _ => 12,
    };
    let mut old_sch_pymt = 0.0;
    let mut prev_old_sch_pymt = 0.0;
    let mut old_prin_flow = 0.0;
    let mut prev_old_prin_flow = 0.0;
    let mut revised_cls_bal = 0.0;
    let mut prev_revised_cls_bal = 0.0;
    let mut new_cls_bal = 0.0;
    let mut prev_new_cls_bal = 0.0;
    let mut prev_new_sch_pymt = 0.0;
    let mut new_prin_flow = 0.0;
    let mut prev_new_prin_flow = 0.0;
    let mut new_int_flow = 0.0;
    let mut prev_new_int_flow = 0.0;
    let mut new_sch_pymt = 0.0;
    let mut new_prin_out_bal = 0.0;
    let mut prev_new_prin_out_bal = 0.0;
    let mut prev_old_sch_pymt = 0.0;
    let mut index = 0;
    let mut final_cashflow_vec: Vec<Cashflow> = Vec::new();

    for cashflows in cashflow_vec.into_iter() {
        let sch_pymt = (cashflows.principal_flow + cashflows.interest_flow);
        old_sch_pymt = sch_pymt;

        if index == 0 {
            new_prin_out_bal = inp_acc.eop_balance;
            prev_new_prin_out_bal = inp_acc.eop_balance;
            new_int_flow = cashflows.interest_flow;
        } else {
            new_prin_out_bal = prev_revised_cls_bal;
            prev_new_prin_out_bal = prev_new_cls_bal;
            new_int_flow = (prev_new_prin_out_bal * inp_acc.net_rate) / (100.00 * int_month as f64);
        }

        if prev_new_prin_out_bal == 0.0 {
            new_int_flow = 0.0;
            prev_new_int_flow = 0.0;
        } else if inp_acc.foreclosure_rate_1 == 1.0 {
            new_int_flow = cashflows.interest_flow;
            prev_new_int_flow = cashflows.interest_flow;
        } else if index == 0 {
            new_int_flow = cashflows.interest_flow;
            prev_new_int_flow = cashflows.interest_flow;
        } else {
            new_int_flow = (prev_revised_cls_bal * inp_acc.net_rate) / (100.00 * int_month as f64);
            prev_new_int_flow =
                (prev_revised_cls_bal * inp_acc.net_rate) / (100.00 * int_month as f64);
        }
        if cashflows.principal_flow == 0.0 {
            new_sch_pymt = prev_new_int_flow;
            prev_new_sch_pymt = prev_new_int_flow;
            prev_new_int_flow = prev_new_int_flow;
            new_prin_flow = 0.0;
            prev_new_prin_flow = 0.0;
        } else if cashflows.principal_flow == 0.0 {
            new_sch_pymt = old_sch_pymt;
            prev_new_sch_pymt = old_sch_pymt;
        } else if prev_new_prin_out_bal == 0.0 {
            new_sch_pymt = 0.0;
            prev_new_sch_pymt = 0.0;
            new_prin_flow = 0.0;
            prev_new_prin_flow = 0.0;
        } else if inp_acc.foreclosure_rate_1 == 1.0 {
            new_prin_flow = cashflows.principal_flow;
            prev_new_prin_flow = cashflows.principal_flow;
            new_sch_pymt = old_sch_pymt;
            prev_new_sch_pymt = old_sch_pymt;
        } else if index == 0 {
            new_sch_pymt = old_sch_pymt;
            prev_new_sch_pymt = old_sch_pymt;
        } else if old_sch_pymt == prev_old_sch_pymt
            || index == cashflow_vec.len() - 1
            || prin_check
            || int_check
        {
            new_sch_pymt = prev_new_sch_pymt * inp_acc.foreclosure_rate_1;
            prev_new_sch_pymt = prev_new_sch_pymt * inp_acc.foreclosure_rate_1;
        } else {
            new_sch_pymt = old_sch_pymt;
            prev_new_sch_pymt = old_sch_pymt;
        }
        if (prev_new_sch_pymt - prev_new_int_flow) >= prev_new_prin_out_bal {
            new_prin_flow = new_prin_out_bal;
            prev_new_prin_flow = new_prin_out_bal;
        } else if index < cashflow_vec.len() - 1 && prev_new_prin_out_bal != 0.0 {
            new_prin_flow = new_sch_pymt - new_int_flow;
            prev_new_prin_flow = new_sch_pymt - new_int_flow;
        } else {
            new_prin_flow = new_prin_out_bal;
            prev_new_prin_flow = new_prin_out_bal;
        }

        if new_prin_flow <= 0.0 {
            new_prin_flow = 0.0;
            prev_new_prin_out_bal = 0.0;
        }
        new_cls_bal = new_prin_out_bal - new_prin_flow;
        prev_new_cls_bal = new_prin_out_bal - new_prin_flow;
        let foreclosure_amt = (prev_new_cls_bal * inp_acc.foreclosure) / 100.00;
        let revised_cls_bal = prev_new_cls_bal - foreclosure_amt;
        prev_revised_cls_bal = prev_new_cls_bal - foreclosure_amt;
        let revised_prin_flow = new_prin_flow + foreclosure_amt;
        prev_old_sch_pymt = sch_pymt;
        let cf_date = timestamp(cashflows.flow_date);
        

        log_debug!(
            log,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            inp_acc.cod_acct_no,
            cashflows.principal_flow,
            cashflows.interest_flow,
            old_sch_pymt,
            new_prin_out_bal,
            inp_acc.foreclosure,
            new_sch_pymt,
            new_int_flow,
            new_prin_flow,
            new_cls_bal,
            foreclosure_amt,
            revised_cls_bal,
            revised_prin_flow,
            cashflows.flow_date 
        );
        if timestamp(inp_acc.derived_reset_date.unwrap_or(*as_on_date))<cf_date{
            final_cashflow_vec.push(new_cashflow(0.0, revised_prin_flow, cf_date));
        }
        else{
        final_cashflow_vec.push(new_cashflow(new_int_flow, revised_prin_flow, cf_date));
        }
        index += 1;
    }

    final_cashflow_vec
}

pub fn months_between_dates(start: NaiveDate, end: NaiveDate) -> i64 {
    let start_year = start.year() as f64;
    let start_month = start.month() as f64;
    let end_year = end.year() as f64;
    let end_month = end.month() as f64;
    let months = (end_year - start_year) * 12.0 + (end_month - start_month);
    months.round() as i64
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
