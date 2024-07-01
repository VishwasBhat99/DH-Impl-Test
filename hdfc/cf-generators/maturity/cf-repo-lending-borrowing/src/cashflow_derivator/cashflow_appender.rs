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

    out_acc.deal_no = account.deal_no;
    out_acc.bank_oper_typ = account.bank_oper_typ;
    out_acc.deal_dt = if let Some(dt) = account.deal_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.value_dt = if let Some(dt) = account.value_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_type = account.deal_type;
    out_acc.slr_type = account.slr_type;
    out_acc.category = account.category;
    out_acc.portfolio = account.portfolio;
    out_acc.counter_party = account.counter_party;
    out_acc.settle_amt = account.settle_amt;
    out_acc.accr_int = account.accr_int;
    out_acc.sec_setdate = if let Some(dt) = account.sec_setdate {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.as_on_dt = timestamp(as_on_dt);
    out_acc.ccy = account.ccy;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
