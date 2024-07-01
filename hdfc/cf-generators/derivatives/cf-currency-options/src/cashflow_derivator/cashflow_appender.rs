use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.trade_id = account.trade_id;
    out_acc.book = account.book;
    out_acc.st_dt = if let Some(dt) = account.st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.opt_sell_buy = account.opt_sell_buy;
    out_acc.put_call = account.put_call;
    out_acc.call_ccy = account.call_ccy;
    out_acc.call_amt = account.call_amt;
    out_acc.put_ccy = account.put_ccy;
    out_acc.put_amt = account.put_amt;
    out_acc.strike_rt = account.strike_rt;
    out_acc.delivery_dt = if let Some(dt) = account.delivery_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.current_spot = account.crnt_spot;
    out_acc.mtm_gain_loss_inr = account.mtm_gain_loss_inr;
    out_acc.frwrd_delta_ccy = account.frwrd_delta_ccy;
    out_acc.frwrd_delta = if let Some(amt) = account.frwrd_delta {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.country_name = account.cntry_name;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
