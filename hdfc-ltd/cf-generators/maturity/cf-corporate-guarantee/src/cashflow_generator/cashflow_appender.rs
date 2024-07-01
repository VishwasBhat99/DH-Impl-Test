use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.account_number = account.account_number;
    out_acc.branch_code = account.branch_code;
    out_acc.customer_id = account.customer_id;
    out_acc.customer_name = account.customer_name;
    out_acc.currency = account.currency;
    out_acc.gl_code = account.gl_code;
    out_acc.currentoutstandingbal =
        if let Some(currentoutstandingbal) = account.currentoutstandingbal {
            currentoutstandingbal
        } else {
            DEFAULT_FLOAT
        };
    out_acc.issue_date = if let Some(dt) = account.issue_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.guarantee_type = account.guarantee_type;
    out_acc.app1 = account.app1;
    out_acc.app2 = account.app2;
    out_acc.app3 = account.app3;
    out_acc.app4 = account.app4;
    out_acc.app5 = account.app5;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
