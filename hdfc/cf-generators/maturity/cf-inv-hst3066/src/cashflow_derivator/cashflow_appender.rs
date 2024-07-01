use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use rbdate::timestamp;

use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    as_on_dt: NaiveDate,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.account_number = account.account_number;
    out_acc.portfolio_name = account.portfolio_name;
    out_acc.maturity = if let Some(dt) = account.maturity {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.face_value = if let Some(fv) = account.face_value {
        fv
    } else {
        DEFAULT_INT
    };
    out_acc.book_value = if let Some(bv) = account.book_value {
        bv
    } else {
        DEFAULT_INT
    };
    out_acc.accr_amt = account.accr_amt;
    out_acc.inv_type = account.inv_type;
    out_acc.as_on_dt = timestamp(as_on_dt);
    out_acc.ccy = account.ccy;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
