use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    int_rt: f64,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.entity = account.entity;
    out_acc.trade_id = account.trade_id;
    out_acc.contract_id = account.contract_id;
    out_acc.folder = account.folder;
    out_acc.trading_banking = account.trading_banking;
    out_acc.intrnl_extrnl = account.intrnl_extrnl;
    out_acc.cntr_prty_name = account.cntr_prty_name;
    out_acc.trade_dt = if let Some(dt) = account.trade_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.st_dt = if let Some(dt) = account.st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_ccy = account.deal_ccy;
    out_acc.org_notional = account.org_notional;
    out_acc.pay_int_rt = account.pay_int_rt;
    out_acc.rec_int_rt = account.rec_int_rt;
    out_acc.exchng_rt = account.exchng_rt;
    out_acc.pay_reset_dt = if let Some(dt) = account.pay_reset_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rec_reset_dt = if let Some(dt) = account.rec_reset_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.pay_payment_dt = if let Some(dt) = account.pay_payment_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rec_payment_dt = if let Some(dt) = account.rec_payment_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.pay_payment_freq = account.pay_payment_freq;
    out_acc.rec_payment_freq = account.rec_payment_freq;
    out_acc.deal_stats = account.deal_stats;
    out_acc.inp_id = account.inp_id;
    out_acc.trade_bank = account.trade_bank;
    out_acc.m_bank = account.m_bank;
    out_acc.flow_typ = account.flow_typ;
    out_acc.flow_typ1 = account.flow_typ1;
    out_acc.flow_typ2 = account.flow_typ2;
    out_acc.flow_typ3 = account.flow_typ3;
    out_acc.flow_typ4 = account.flow_typ4;
    out_acc.flow_amt = account.flow_amt;
    out_acc.cf_dt = if let Some(dt) = account.cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.flow_ccy = account.flow_ccy;
    out_acc.hkd_rt = account.hkd_rt;
    out_acc.hkd_amt = account.hkd_amt;
    out_acc.m_h_rep_dt2 = if let Some(dt) = account.m_h_rep_dt2 {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.inr_amt = account.inr_amt;
    out_acc.inr_rt = account.inr_rt;
    out_acc.int_rt = int_rt;
    out_acc.cont_usage = account.cont_usage;
    out_acc.non_idx_fwcurcy = account.non_idx_fwcurcy;
    out_acc.non_idx_fwamt = account.non_idx_fwamt;
    out_acc.indxn_dt = if let Some(dt) = account.indxn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
