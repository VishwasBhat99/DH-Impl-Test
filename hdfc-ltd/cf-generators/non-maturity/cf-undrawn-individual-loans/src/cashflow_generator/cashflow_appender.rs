use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.lac_no = account.lac_no;
    out_acc.sanc_amt = if let Some(sanc_amt) = account.sanc_amt {
        sanc_amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.currency = account.currency;
    out_acc.amt_14per = if let Some(amt_14per) = account.amt_14per {
        amt_14per
    } else {
        DEFAULT_FLOAT
    };
    out_acc.amt_2per = if let Some(amt_2per) = account.amt_2per {
        amt_2per
    } else {
        DEFAULT_FLOAT
    };
    out_acc.approval_dt = if let Some(dt) = account.approval_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.first_cf_dt = if let Some(dt) = account.first_cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
