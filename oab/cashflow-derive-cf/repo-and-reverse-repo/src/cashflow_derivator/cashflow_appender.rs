use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    _nxt_rep_dt: Option<NaiveDate>,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.account_id = account.account_id;
    out_acc.sec_id = account.sec_id;
    out_acc.product = account.product;
    out_acc.product_type = account.product_type;
    out_acc.face_amt = account.face_amt;
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.cmne = account.cmne;
    out_acc.sn = account.sn;
    out_acc.c_type = account.c_type;
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repo_rate = account.repo_rate;
    out_acc.int_calc_type = account.int_calc_type;
    out_acc.spread = account.spread;
    out_acc.benchmark = account.benchmark;
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
