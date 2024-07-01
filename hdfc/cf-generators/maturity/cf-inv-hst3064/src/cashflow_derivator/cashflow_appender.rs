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

    out_acc.security_desc = account.security_desc;
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
    out_acc.int_rate = account.int_rate;
    out_acc.amt = account.amt;
    out_acc.int_freq = account.int_freq;
    out_acc.last_cpn_dt = if let Some(dt) = account.last_cpn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_cpn_dt = if let Some(dt) = account.nxt_cpn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.no_of_days = if let Some(days) = account.no_of_days {
        days
    } else {
        DEFAULT_INT
    };
    out_acc.as_on_dt = timestamp(as_on_dt);
    out_acc.ccy = account.ccy;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
