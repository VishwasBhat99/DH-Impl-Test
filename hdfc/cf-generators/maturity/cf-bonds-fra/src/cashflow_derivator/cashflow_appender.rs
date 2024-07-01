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

    out_acc.entity = account.entity;
    out_acc.trade_id = account.trade_id;
    out_acc.cntrct_id = account.cntrct_id;
    out_acc.struct_id = account.struct_id;
    out_acc.comp_typology = account.comp_typology;
    out_acc.pkg_typology = account.pkg_typology;
    out_acc.cntrct_typology = account.cntrct_typology;
    out_acc.cntrct_usage = account.cntrct_usage;
    out_acc.desk = account.desk;
    out_acc.trading_banking = account.trading_banking;
    out_acc.cntr_prty_child_cd = account.cntr_prty_child_cd;
    out_acc.cntr_prty_name = account.cntr_prty_name;
    out_acc.intrnl_extrnl = account.intrnl_extrnl;
    out_acc.trade_dt = if let Some(dt) = account.trade_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.fixing_dt = if let Some(dt) = account.fixing_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.settlement_dt = if let Some(dt) = account.settlement_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_dt = if let Some(dt) = account.maturity_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_ccy = account.deal_ccy;
    out_acc.sec_cd = account.sec_cd;
    out_acc.undrlying_sec = account.undrlying_sec;
    out_acc.undrlying_sec_maturity = if let Some(dt) = account.undrlying_sec_maturity {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.notional_amt = account.notional_amt;
    out_acc.org_notional_in_inr = account.org_notional_in_inr;
    out_acc.ost_notional_in_inr = account.ost_notional_in_inr.abs();
    out_acc.cont_notional = account.cont_notional;
    out_acc.buy_sell = account.buy_sell;
    out_acc.fut_cash_proceeds_ccy = account.fut_cash_proceeds_ccy;
    out_acc.fut_cash_proceeds = account.fut_cash_proceeds;
    out_acc.fut_cash_proceeds_in_inr = account.fut_cash_proceeds_in_inr;
    out_acc.mtm = account.mtm;
    out_acc.mtm_in_inr = account.mtm_in_inr;
    out_acc.fwdmtm_in_inr = account.fwdmtm_in_inr;
    out_acc.net_bcva_adj_gmtm_in_inr = account.net_bcva_adj_gmtm_in_inr;
    out_acc.cva = account.cva;
    out_acc.dva = account.dva;
    out_acc.bcva = account.bcva;
    out_acc.netpv01 = account.netpv01;
    out_acc.bank_or_nonbank = account.bank_or_nonbank;
    out_acc.org_tenor = account.org_tenor;
    out_acc.res_tenor = account.res_tenor;
    out_acc.udrlying = account.udrlying;
    out_acc.deal_status = account.deal_status;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
