use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;
    out_acc.reval_dt = if let Some(dt) = acc.reval_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_type = acc.deal_type;
    out_acc.deal_ref = acc.deal_ref;
    out_acc.leg_id = acc.leg_id;
    out_acc.portfolio = acc.portfolio;
    out_acc.product = acc.product;
    out_acc.counter_party = acc.counter_party;
    out_acc.buy_sell = acc.buy_sell;
    out_acc.deal_dt = if let Some(dt) = acc.deal_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.val_dt = if let Some(dt) = acc.val_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ccy_1 = acc.ccy_1;
    out_acc.ccy_2 = acc.ccy_2;
    out_acc.deal_rt = acc.deal_rt;
    out_acc.ccy1_amt = acc.ccy1_amt;
    out_acc.crnct2_amt = acc.crnct2_amt;
    out_acc.reval_rt = acc.reval_rt;
    out_acc.reval_eqv = acc.reval_eqv;
    out_acc.actual_pl = acc.actual_pl;
    out_acc.pnl_conversion_rt = acc.pnl_conversion_rt;
    out_acc.int_rt = acc.int_rt;
    out_acc.discounted_factor = acc.discounted_factor;
    out_acc.present_val_pl = acc.present_val_pl;
    out_acc.ccy1_spot_rt = acc.ccy1_spot_rt;
    out_acc.ccy2_spot_rt = acc.ccy2_spot_rt;
    out_acc.inr_eq_ccy1_spot_rt = acc.inr_eq_ccy1_spot_rt;
    out_acc.inr_eq_ccy2_spot_rt = acc.inr_eq_ccy2_spot_rt;
    out_acc.branch_code = acc.branch_code;
    out_acc.trsy_gl = acc.trsy_gl;
    out_acc.cf_dt = if let Some(dt) = acc.cf_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cf_ccy = acc.cf_ccy;
    out_acc.cf_amt = acc.cf_amt;
    out_acc.cf_typ = acc.cf_typ;
    out_acc.prin_amt = acc.prin_amt;
    out_acc.int_amt = acc.int_amt;
    out_acc.flow_typ = acc.flow_typ;
    out_acc.abs_cf_amt = acc.abs_cf_amt;
    out_acc.cbs_gl_cd = acc.cbs_gl_cd;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
