use std::{collections::HashMap};

use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{timestamp, NaiveDate};
use slog::Logger;
use statics::*;

use super::DealData;


pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
    deal_list_map: &HashMap<String, Vec<DealData>>
) -> Vec<Cashflow> {
    let mut cashflows: Vec<Cashflow> = Vec::new();
    let mut cf_date = account.maturity_date.unwrap_or(as_on_dt);
    let final_buy_quantity = account.face_value as f64;
    let final_buy_amount = account.book_value;
 

    let def_vec: Vec<DealData> = Vec::new();
    let mut deal_data_vec = match deal_list_map.get(&account.concat_deal_id) {
        Some(val) => {
            val.clone()},
        None => {
            let mut cf: Cashflow = Cashflow::new();
            cf.date = timestamp(account.issue_date.unwrap_or(as_on_dt));
            cf.int_amt = 0.0;
            cf.prin_amt = final_buy_amount;
            cashflows.push(cf);
            def_vec.clone()
        }
    };
    deal_data_vec.sort_by(|a, b| b.deal_dt.cmp(&a.deal_dt));
    let mut cumu_buy_qty = 0.0;
    let mut cumu_buy_amt = 0.0;

   for deal_data in deal_data_vec {
        if cumu_buy_amt < final_buy_amount {
            cumu_buy_qty += deal_data.buy_quantity;
            cumu_buy_amt += deal_data.buy_amount;
            let cf = new_cashflow(0.0, deal_data.buy_amount, timestamp(deal_data.deal_dt));
            cashflows.push(cf);
            cf_date = deal_data.deal_dt;
        }else{
            break;
        }
    }
    let amt_diff = final_buy_amount - cumu_buy_amt;
    if final_buy_amount != cumu_buy_amt {
        balance_cashflows(&mut cashflows,amt_diff);
    }
    cashflows
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}

fn balance_cashflows (cashflows:&mut Vec<Cashflow>, amt_diff:f64) {
    let cf_len = cashflows.len();
    if amt_diff != 0.0 {
        let last_cf = &cashflows[cf_len-1];
        let new_prin = last_cf.prin_amt + amt_diff;
        let cf = new_cashflow(0.0, new_prin, last_cf.date);
        cashflows[cf_len-1]=cf;
    }
}