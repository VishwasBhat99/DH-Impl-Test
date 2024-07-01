use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    cf_type: String,
) -> AccountWithCashflows {
    let mut op_acc = AccountWithCashflows::new();

    op_acc.cf_sub_type = account.cf_sub_type;
    op_acc.c_party = account.c_party;
    op_acc.ccy = account.ccy;
    op_acc.typ = account.typ;
    op_acc.sanc_amt = account.sanc_amt;
    op_acc.st_dt = if let Some(dt) = account.st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op_acc.ed_dt = if let Some(dt) = account.ed_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    op_acc.country = account.country;
    op_acc.util_amt = account.util_amt;
    op_acc.cf_type = cf_type;
    op_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    op_acc
}
