use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use protobuf;
use rbdate::timestamp;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    as_on_dt: NaiveDate,
) -> AccountWithCashflows {
    let mut account_with_cashflows = AccountWithCashflows::new();
    account_with_cashflows.gl_number = account.gl_number;
    account_with_cashflows.gl_desc = account.gl_desc;
    account_with_cashflows.pd = account.pd;
    account_with_cashflows.prod_type = account.prod_type;
    account_with_cashflows.i = account.i;
    account_with_cashflows.total_pd_sum = account.total_pd_sum;
    account_with_cashflows.total_pd_count = account.total_pd_count;
    account_with_cashflows.as_on_dt = timestamp(as_on_dt);
    account_with_cashflows.nxt_rep_dt = DEFAULT_INT;
    account_with_cashflows.ccy = "INR".to_string();
    account_with_cashflows.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    account_with_cashflows
}
