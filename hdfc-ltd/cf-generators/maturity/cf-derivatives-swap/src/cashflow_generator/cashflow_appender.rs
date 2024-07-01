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
    out_acc.instrument_type = account.instrument_type;
    out_acc.trading_banking = account.trading_banking;
    out_acc.counter_party_id = account.counter_party_id;
    out_acc.counterparty_name = account.counterparty_name;
    out_acc.internal_external = account.internal_external;
    out_acc.trade_date = if let Some(dt) = account.trade_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.start_date = if let Some(dt) = account.start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.end_date = if let Some(dt) = account.end_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.currency = account.currency;
    out_acc.original_notional_rec_leg = account.original_notional_rec_leg;
    out_acc.original_notional_rec_leg_lcy = account.original_notional_rec_leg_lcy;
    out_acc.outstanding_notional_rec_leg = account.outstanding_notional_rec_leg;
    out_acc.outstanding_notional_rec_leg_lcy =
        if let Some(outstanding_notional_rec_leg_lcy) = account.outstanding_notional_rec_leg_lcy {
            outstanding_notional_rec_leg_lcy
        } else {
            DEFAULT_FLOAT
        };
    out_acc.app1 = account.app1;
    out_acc.original_notional_pay_leg = account.original_notional_pay_leg;
    out_acc.original_notional_pay_leg_lcy = account.original_notional_pay_leg_lcy;
    out_acc.outstanding_notional_pay_leg = account.outstanding_notional_pay_leg;
    out_acc.outstanding_notional_pay_leg_lcy =
        if let Some(outstanding_notional_pay_leg_lcy) = account.outstanding_notional_pay_leg_lcy {
            outstanding_notional_pay_leg_lcy
        } else {
            DEFAULT_FLOAT
        };
    out_acc.contingent_notional = account.contingent_notional;
    out_acc.pay_leg_index = account.pay_leg_index;
    out_acc.pay_int_rate = account.pay_int_rate;
    out_acc.spread_pay_leg = account.spread_pay_leg;
    out_acc.rec_leg_index = account.rec_leg_index;
    out_acc.rec_int_rate = account.rec_int_rate;
    out_acc.spread_rec_leg = account.spread_rec_leg;
    out_acc.modified_duration_deal = account.modified_duration_deal;
    out_acc.exchange_rate = account.exchange_rate;
    out_acc.app5 = account.app5;
    out_acc.pay_reset_date = if let Some(dt) = account.pay_reset_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rec_reset_date = if let Some(dt) = account.rec_reset_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.pay_payment_date = if let Some(dt) = account.pay_payment_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rec_payment_date = if let Some(dt) = account.rec_payment_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.day_count_convention_rec = account.day_count_convention_rec;
    out_acc.day_count_convention_pay = account.day_count_convention_pay;
    out_acc.pay_reset_frequency = account.pay_reset_frequency;
    out_acc.rec_reset_frequency = account.rec_reset_frequency;
    out_acc.pay_payment_frequency = account.pay_payment_frequency;
    out_acc.rec_payment_frequency = account.rec_payment_frequency;
    out_acc.leg_type = account.leg_type;
    out_acc.underlying_pp = account.underlying_pp;
    out_acc.net_pl_amount = account.net_pl_amount;
    out_acc.counterpartycategory1 = account.counterpartycategory1;
    out_acc.counterpartycategory2 = account.counterpartycategory2;
    out_acc.counterpartycategory3 = account.counterpartycategory3;
    out_acc.cashflow_type = account.cashflow_type;
    out_acc.treasury_gL_code = account.treasury_gL_code;
    out_acc.app2 = account.app2;
    out_acc.app3 = account.app3;
    out_acc.app4 = account.app4;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
