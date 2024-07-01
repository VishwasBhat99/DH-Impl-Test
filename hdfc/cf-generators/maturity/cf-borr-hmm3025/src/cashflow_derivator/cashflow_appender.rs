use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.deal_id = account.deal_id;
    out_acc.inst_name = account.inst_name;
    out_acc.oper_type = account.oper_type;
    out_acc.counter_party = account.counter_party;
    out_acc.deal_dt = if let Some(dt) = account.deal_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.val_dt = if let Some(dt) = account.val_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ccy = account.ccy;
    out_acc.lcy_amt = account.lcy_amt;
    out_acc.roi = account.roi;
    out_acc.tenor_days = account.tenor_days;
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_amt = account.int_amt;
    out_acc.mat_amt = if let Some(amt) = account.mat_amt {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.dealer_name = account.dealer_name;
    out_acc.ndsref = account.ndsref;
    out_acc.deal_status = account.deal_status;
    out_acc.nds_time = account.nds_time;
    out_acc.aip = account.aip;
    out_acc.air = account.air;
    out_acc.code = account.code;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
