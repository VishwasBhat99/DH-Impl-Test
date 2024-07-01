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
    let mut total_interest_amount = DEFAULT_FLOAT;
    let mut total_principal_amount = DEFAULT_FLOAT;

    out_acc.deal_id = account.deal_id;
    out_acc.branch = account.branch;
    out_acc.inst_name = account.inst_name;
    out_acc.lend_borr_typ = account.lend_borr_typ;
    out_acc.typology = account.typology;
    out_acc.usage = account.usage;
    out_acc.sub_typ_borr_lend = account.sub_typ_borr_lend;
    out_acc.cntrprty = account.cntrprty;
    out_acc.crtn_dt = if let Some(dt) = account.crtn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.val_date = if let Some(dt) = account.val_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deal_date = if let Some(dt) = account.deal_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ccy = account.ccy;
    out_acc.crnt_deal_amt = account.crnt_deal_amt;
    out_acc.crnt_conv_rt_lcy = account.crnt_conv_rt_lcy;
    out_acc.crnt_deal_amt_lcy = account.crnt_deal_amt_lcy;
    out_acc.roi = account.roi;
    out_acc.tenor_days = account.tenor_days;
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.prin_amt = account.prin_amt;
    out_acc.int_amt = account.int_amt;
    out_acc.cf_typ = account.cf_typ;
    out_acc.flow_typ = account.flow_typ;
    out_acc.mat_amt = account.mat_amt;
    out_acc.dealer_name = account.dealer_name;
    out_acc.nds_ref_no = account.nds_ref_no;
    out_acc.nxt_fix_dt = if let Some(dt) = account.nxt_fix_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.residual_tenor = account.residual_tenor;
    out_acc.nxt_put_dt = if let Some(dt) = account.nxt_put_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_call_dt = if let Some(dt) = account.nxt_call_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_int_pay_dt = if let Some(dt) = account.nxt_int_pay_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_pay_tenor = account.int_pay_tenor;
    out_acc.aip_air = account.aip_air;
    out_acc.downgrade_clause = account.downgrade_clause;
    out_acc.avg_monthly_bal = account.avg_monthly_bal;
    out_acc.glcode = account.glcode;
    out_acc.cntrprty_ctgry_1 = account.cntrprty_ctgry_1;
    out_acc.cntrprty_ctgry_2 = account.cntrprty_ctgry_2;
    out_acc.cntrprty_ctgry_3 = account.cntrprty_ctgry_3;
    out_acc.cntrprty_ctgry_4 = account.cntrprty_ctgry_4;
    out_acc.int_pay_rec = account.int_pay_rec;
    out_acc.bckt_days = account.bckt_days;
    out_acc.system_gl = account.system_gl;
    out_acc.alm_concat = account.alm_concat;
    out_acc.div = account.div;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_line = account.ia_line;
    for cf in &cashflows {
        total_interest_amount += cf.interest_amount;
        total_principal_amount += cf.principal_amount;
    }
    out_acc.tot_int_amt = total_interest_amount;
    out_acc.tot_prin_amt = total_principal_amount;
    out_acc.sma_flag = account.sma_flag;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
