use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    cf_type: &str,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.entity = account.entity;
    out_acc.trade_id = account.trade_id.to_string();
    out_acc.struct_id = account.struct_id;
    out_acc.comp_typology = account.comp_typology;
    out_acc.cntrct_typology = account.cntrct_typology;
    out_acc.desk = account.desk;
    out_acc.book = account.book;
    out_acc.folder = account.folder;
    out_acc.trading_banking = account.trading_banking;
    out_acc.cntr_prty_grp_cd = account.cntr_prty_grp_cd;
    out_acc.cntr_prty_prnt_cd = account.cntr_prty_prnt_cd;
    out_acc.intrnl_extrnl = account.intrnl_extrnl;
    out_acc.trade_dt = if let Some(dt) = account.trade_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.del_dt = if let Some(dt) = account.del_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.buy_sell = account.buy_sell;
    out_acc.put_call = account.put_call;
    out_acc.call_ccy = account.call_ccy;
    out_acc.put_ccy = account.put_ccy;
    out_acc.put_amt = account.put_amt;
    out_acc.mtm_excld_prem_inr = account.mtm_excld_prem_inr;
    out_acc.position_ccy = account.position_ccy;
    out_acc.forward_delta_ccy_1_amt = account.forward_delta_ccy_1_amt;
    out_acc.pl_ccy = account.pl_ccy;
    out_acc.forward_delta_ccy_2_amt = account.forward_delta_ccy_2_amt;
    out_acc.inst = account.ccy;
    out_acc.cf_type = cf_type.to_string();
    out_acc.cntrct_usage = account.cntrct_usage;
    out_acc.settle_ccy = account.settle_ccy;
    out_acc.fwdmtm_settle_ccy = account.fwdmtm_settle_ccy;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.settlement_type = account.settlement_type;
    out_acc.settlement_Amount = account.settlement_amount;
    out_acc.Delivery_Date = if let Some(dt) = account.del_dt {
        dt.format("%d-%m-%Y").to_string()
    } else {
        "".to_string()
    };
    out_acc.digital_payout_currency = account.digital_payout_currency;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
