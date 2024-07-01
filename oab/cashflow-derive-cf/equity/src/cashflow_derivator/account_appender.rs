use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::account_with_cashflows::OutputAccount;
use protobuf;
use rbdate::*;
use statics::*;
use std::collections::HashMap;

pub fn create_account_without_cashflows(
    account: InputAccount,
    rule_dates: &HashMap<i64, NaiveDate>,
    rule_rates: &HashMap<i64, f64>,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    // Standard Fields
    out_acc.account_id = account.account_id;
    out_acc.book_value = account.book_value;
    out_acc.cf_amount = account.cf_amount;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.listing_status = account.listing_status;
    out_acc.listed_exchange = account.listed_exchange;
    out_acc.equity_id = account.equity_id;
    out_acc.equity_name = account.equity_name;
    out_acc.equity_issuer_type = account.equity_issuer_type;
    out_acc.issuer_country = account.issuer_country;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_name = account.customer_name;
    out_acc.customer_type = account.customer_type;
    out_acc.isin = account.isin;
    out_acc.ifrs9cat = account.ifrs9cat;
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.branch = account.branch;
    out_acc.rm = account.rm;
    out_acc.department = account.department;
    out_acc.gl = account.gl;
    out_acc.product_code = account.product_code;
    out_acc.inv_type = account.inv_type;

    let lst_cf = get_cashflows(&rule_dates, &rule_rates, account.cf_amount);
    out_acc.cashflows = protobuf::RepeatedField::from_vec(lst_cf);

    out_acc
}

pub fn get_cashflows(
    rule_dates: &HashMap<i64, NaiveDate>,
    rule_rates: &HashMap<i64, f64>,
    cf_amount: f64,
) -> Vec<Cashflow> {
    let mut lst_cf: Vec<Cashflow> = Vec::new();

    for (key, value) in rule_dates {
        let rate = match rule_rates.get(key) {
            Some(x) => *x,
            None => 0.0,
        };
        let dist_amount = cf_amount * rate / 100.00;
        lst_cf.push(new_cashflow(dist_amount, *value));
    }

    lst_cf
}

fn new_cashflow(dist_amount: f64, d: NaiveDate) -> Cashflow {
    let mut cashflow = Cashflow::new();
    cashflow.prin_amt = dist_amount;
    cashflow.date = rbdate::timestamp(d);

    cashflow
}
