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
    out_acc.ed_dt = if let Some(dt) = account.ed_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_ccy = account.deal_ccy;
    out_acc.org_notional = account.org_notional;
    out_acc.org_notional_inr = account.org_notional_inr;
    out_acc.rec_crnt_notional = account.rec_crnt_notional;
    out_acc.rec_crnt_notional_inr = account.rec_crnt_notional_inr;
    out_acc.pay_crnt_notional = account.pay_crnt_notional;
    out_acc.pay_crnt_notional_inr = account.pay_crnt_notional_inr;
    out_acc.contignent_notional = account.contignent_notional;
    out_acc.deal_side = account.deal_side;
    out_acc.pay_leg_idx = account.pay_leg_idx;
    out_acc.pay_int_rt = account.pay_int_rt;
    out_acc.spread_pay_leg = account.spread_pay_leg;
    out_acc.rec_leg_idx = account.rec_leg_idx;
    out_acc.rec_int_rt = account.rec_int_rt;
    out_acc.spread_rec_leg = account.spread_rec_leg;
    out_acc.pay_side_acrl = account.pay_side_acrl;
    out_acc.pay_side_mtm = account.pay_side_mtm;
    out_acc.pay_side_gmtm = account.pay_side_gmtm;
    out_acc.rec_side_acrl = account.rec_side_acrl;
    out_acc.net_acrl_inr = account.net_acrl_inr;
    out_acc.net_acrl_usd = account.net_acrl_usd;
    out_acc.future_cash_proceeds_ccy = account.future_cash_proceeds_ccy;
    out_acc.future_cash_proceeds = account.future_cash_proceeds;
    out_acc.future_cash_proceeds_inr = account.future_cash_proceeds_inr;
    out_acc.net_mtm = account.net_mtm;
    out_acc.net_mtm_inr = account.net_mtm_inr;
    out_acc.net_mtm_usd = account.net_mtm_usd;
    out_acc.net_gmtm_inr = account.net_gmtm_inr;
    out_acc.net_gmtm_usd = account.net_gmtm_usd;
    out_acc.net_bcva_adjstd_gmtm_inr = account.net_bcva_adjstd_gmtm_inr;
    out_acc.pay_side_pv01_inr = account.pay_side_pv01_inr;
    out_acc.rec_side_pv01_inr = account.rec_side_pv01_inr;
    out_acc.net_pv01_inr = account.net_pv01_inr;
    out_acc.pay_side_modified_duration = account.pay_side_modified_duration;
    out_acc.exchange_rt = account.exchange_rt;
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
    out_acc.org_tenor = account.org_tenor;
    out_acc.residual_tenor = account.residual_tenor;
    out_acc.pay_reset_freq = account.pay_reset_freq;
    out_acc.rec_reset_freq = account.rec_reset_freq;
    out_acc.pay_payment_freq = account.pay_payment_freq;
    out_acc.rec_payment_freq = account.rec_payment_freq;
    out_acc.deal_status = account.deal_status;
    out_acc.inp_id = account.inp_id;
    out_acc.auth_id = account.auth_id;
    out_acc.trad_bank = account.trad_bank;
    out_acc.m_bank_b = account.m_bank_b;
    out_acc.flowtype = account.flowtype;
    out_acc.flowtype1 = account.flowtype1;
    out_acc.flowtype2 = account.flowtype2;
    out_acc.flowtype3 = account.flowtype3;
    out_acc.flowtype4 = account.flowtype4;
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
    out_acc.cntrct_usage = account.cntrct_usage;
    out_acc.non_idx_flw_curr = account.non_idx_flw_curr;
    out_acc.non_idx_cf_amt = account.non_idx_cf_amt;
    out_acc.indexation_dt = if let Some(dt) = account.indexation_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
