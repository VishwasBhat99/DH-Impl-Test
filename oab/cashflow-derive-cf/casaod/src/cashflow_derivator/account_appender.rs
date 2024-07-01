use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::account_with_cashflows::OutputAccount;
use protobuf;
use rbdate::{timestamp, NaiveDate};
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
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.int_rate = account.int_rate;
    out_acc.cf_amount = account.cf_amount;
    out_acc.gl = account.gl;
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rate_flag = account.rate_flag;
    out_acc.branch = account.branch;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_type = account.customer_type;
    out_acc.product_code = account.product_code;

    // Standard Passthrough
    out_acc.group = account.group;
    out_acc.acc_branch = account.acc_branch;
    out_acc.acc_number = account.acc_number;
    out_acc.acc_suffix = account.acc_suffix;
    out_acc.acc_type = account.acc_type;
    out_acc.deal_type = account.deal_type;
    out_acc.repricing_frequency = account.repricing_frequency;
    out_acc.last_repr_date = if let Some(dt) = account.last_repr_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_repr_date = if let Some(dt) = account.next_repr_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_compounding_frequency = account.int_compounding_frequency;
    out_acc.int_repayment_frequency = account.int_repayment_frequency;
    out_acc.margin_rate = account.margin_rate;
    out_acc.cpas = account.cpas;
    out_acc.cust_constitution_code = account.cust_constitution_code;
    out_acc.customer_rating = account.customer_rating;
    out_acc.p2 = account.p2;
    out_acc.analysis_code = account.analysis_code;
    out_acc.sundry_analysis_code = account.sundry_analysis_code;
    out_acc.numeric_analysis_code = account.numeric_analysis_code;
    out_acc.base_rate_code = account.base_rate_code;
    out_acc.differential_rate_code = account.differential_rate_code;
    out_acc.accrued_int_amt = account.accrued_int_amt;
    out_acc.rm = account.rm;
    out_acc.customer_name = account.customer_name;
    out_acc.monthly_avg_bal = account.monthly_avg_bal;
    out_acc.npa_flag = account.npa_flag;
    out_acc.npa_type = account.npa_type;
    out_acc.pension_acc_flag = account.pension_acc_flag;
    out_acc.waiver_flag = account.waiver_flag;

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
