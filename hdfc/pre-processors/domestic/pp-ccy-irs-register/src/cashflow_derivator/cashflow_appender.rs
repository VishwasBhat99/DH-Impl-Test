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
    out_acc.struct_id_link_id = account.struct_id_link_id;
    out_acc.comp_typology = account.comp_typology;
    out_acc.pkg_typology = account.pkg_typology;
    out_acc.cntrct_typology = account.cntrct_typology;
    out_acc.desk = account.desk;
    out_acc.book = account.book;
    out_acc.folder = account.folder;
    out_acc.trading_banking = account.trading_banking;
    out_acc.cntr_prty_grp_cd = account.cntr_prty_grp_cd;
    out_acc.cntr_prty_chld_cd = account.cntr_prty_chld_cd;
    out_acc.cntr_prty_name = account.cntr_prty_name;
    out_acc.intrnl_extrnl = account.intrnl_extrnl;
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
    out_acc.ccy_pair = account.ccy_pair;
    out_acc.rec_leg_ccy = account.rec_leg_ccy;
    out_acc.org_notional_rec_leg = account.org_notional_rec_leg;
    out_acc.org_notional_rec_leg_inr = account.org_notional_rec_leg_inr;
    out_acc.ost_notional_rec_leg = account.ost_notional_rec_leg;
    out_acc.ost_notional_rec_leg_inr = account.ost_notional_rec_leg_inr;
    out_acc.pay_leg_ccy = account.pay_leg_ccy;
    out_acc.org_notional_pay_leg = account.org_notional_pay_leg;
    out_acc.org_notional_pay_leg_inr = account.org_notional_pay_leg_inr;
    out_acc.ost_notional_pay_leg = account.ost_notional_pay_leg;
    out_acc.ost_notional_pay_leg_inr = account.ost_notional_pay_leg_inr;
    out_acc.deal_side = account.deal_side;
    out_acc.pay_leg_idx = account.pay_leg_idx;
    out_acc.pay_int_rt = account.pay_int_rt;
    out_acc.spread_pay_leg = account.spread_pay_leg;
    out_acc.rec_leg_idx = account.rec_leg_idx;
    out_acc.rec_int_rt = account.rec_int_rt;
    out_acc.spread_rec_leg = account.spread_rec_leg;
    out_acc.rec_side_acrl_inr = account.rec_side_acrl_inr;
    out_acc.rec_side_mtm_inr = account.rec_side_mtm_inr;
    out_acc.future_cash_proceeds_ccy = account.future_cash_proceeds_ccy;
    out_acc.future_cash_proceeds_inr = account.future_cash_proceeds_inr;
    out_acc.mrkt_val_financed = account.mrkt_val_financed;
    out_acc.net_mtm_usd = account.net_mtm_usd;
    out_acc.net_mtm_inr = account.net_mtm_inr;
    out_acc.pay_side_pv01_inr = account.pay_side_pv01_inr;
    out_acc.rec_side_pv01_inr = account.rec_side_pv01_inr;
    out_acc.net_pv01_inr = account.net_pv01_inr;
    out_acc.pay_side_modified_duration = account.pay_side_modified_duration;
    out_acc.receive_side_modified_duration = account.receive_side_modified_duration;
    out_acc.modified_duration_deal = account.modified_duration_deal;
    out_acc.pay_leg_exchange_rt = account.pay_leg_exchange_rt;
    out_acc.rec_leg_exchange_rt = account.rec_leg_exchange_rt;
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
    out_acc.index_rec_leg = account.index_rec_leg;
    out_acc.index_pay_leg = account.index_pay_leg;
    out_acc.day_count_cnvntn_rec_leg = account.day_count_cnvntn_rec_leg;
    out_acc.day_count_cnvntn_pay_leg = account.day_count_cnvntn_pay_leg;
    out_acc.pay_reset_freq = account.pay_reset_freq;
    out_acc.rec_reset_freq = account.rec_reset_freq;
    out_acc.pay_payment_freq = account.pay_payment_freq;
    out_acc.rec_payment_freq = account.rec_payment_freq;
    out_acc.deal_status = account.deal_status;
    out_acc.flowtype = account.flowtype;
    out_acc.flowtype1 = account.flowtype1;
    out_acc.flowtype2 = account.flowtype2;
    out_acc.flowtype3 = account.flowtype3;
    out_acc.flowtype4 = account.flowtype4;
    out_acc.flowamount = account.flowamount;
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
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
