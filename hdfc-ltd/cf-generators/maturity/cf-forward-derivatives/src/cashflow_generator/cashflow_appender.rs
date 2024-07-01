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

    out_acc.deal_id = account.deal_id;
    out_acc.deal_type = account.deal_type;
    out_acc.deal_ref = account.deal_ref;
    out_acc.leg_type = account.leg_type;
    out_acc.leg_number = account.leg_number;
    out_acc.trading_banking = account.trading_banking;
    out_acc.counter_party_id = account.counter_party_id;
    out_acc.counter_party_name = account.counter_party_name;
    out_acc.deal_date = if let Some(dt) = account.deal_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.currency = account.currency;
    out_acc.exchange_rate = account.exchange_rate;
    out_acc.currency_amt_inr = if let Some(currency_amt_inr) = account.currency_amt_inr {
        currency_amt_inr
    } else {
        DEFAULT_FLOAT
    };
    out_acc.currency_amt = account.currency_amt;
    out_acc.reval_rate = account.reval_rate;
    out_acc.netplamount = account.netplamount;
    out_acc.treasury_gl_code = account.treasury_gl_code;
    out_acc.app1 = account.app1;
    out_acc.app2 = account.app2;
    out_acc.app3 = account.app3;
    out_acc.app4 = account.app4;
    out_acc.app5 = account.app5;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
