use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    nxt_rep_dt: Option<NaiveDate>,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.account_id = account.account_id.to_string();
    out_acc.sec_id = account.sec_id;
    out_acc.face_amt = account.face_amt;
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.prod_type = account.prod_type;
    out_acc.coup_rate = account.coup_rate;
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_repr_date = if let Some(dt) = nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repricing_frequency = account.repricing_frequency;
    out_acc.last_repr_date = if let Some(dt) = account.last_repr_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.benchmark = account.benchmark;
    out_acc.spread = account.spread;
    out_acc.ctype = account.ctype;
    out_acc.acct_ng_type = account.acct_ng_type;
    out_acc.guarantor = account.guarantor;
    out_acc.ccode = account.ccode;
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.call_date = if let Some(dt) = account.call_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.inv_type = account.inv_type;
    out_acc.int_rate = account.int_rate;
    out_acc.rate_flag = account.rate_flag;
    out_acc.product_code = account.product_code;
    out_acc.customer_id = account.customer_id;
    out_acc.branch = account.branch;
    out_acc.rm = account.rm;
    out_acc.department = account.department;
    out_acc.gl = account.gl;
    out_acc.customer_name = account.customer_name;
    out_acc.monthly_avg_bal = account.monthly_avg_bal;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }

    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());

    out_acc
}
