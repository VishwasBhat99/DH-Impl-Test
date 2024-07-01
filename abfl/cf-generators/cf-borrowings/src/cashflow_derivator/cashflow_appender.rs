use crate::cashflow_derivator::account_reader::input_account::InputAccount;
use crate::cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use crate::cashflow_derivator::account_with_cashflows::Cashflow;
use crate::statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.cust_no = account.cust_no;
    out_acc.reference = account.reference;
    out_acc.cust_name = account.cust_name;
    out_acc.branch_cd = account.branch_cd;
    out_acc.norm_int_rt = if let Some(val) = account.norm_int_rt {
        val
    } else {
        DEFAULT_FLOAT
    };
    out_acc.acurl_freq = account.acurl_freq;
    out_acc.book_dt = if let Some(dt) = account.book_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.val_dt = if let Some(dt) = account.val_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.due_dt = if let Some(dt) = account.due_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.user_def_stats = account.user_def_stats;
    out_acc.prod_cd = account.prod_cd;
    out_acc.gl = account.gl;
    out_acc.curr = account.curr;
    out_acc.prin_ost_bal = account.prin_ost_bal;
    out_acc.component = account.component;
    out_acc.amt_due = account.amt_due;
    out_acc.amt_setld = account.amt_setld;
    out_acc.cf_amt = account.cf_amt.unwrap_or(DEFAULT_FLOAT);
    out_acc.spread = account.spread;
    out_acc.bucket_category = account.bucket_category;
    out_acc.is_secured = account.is_secured;
    out_acc.product_type = account.product_type;
    out_acc.comp_perc = account.comp_perc;
    out_acc.old_rt_typ = account.old_rt_typ;
    out_acc.old_benchmark = account.old_benchmark;
    out_acc.nxt_call_dt = if let Some(dt) = account.nxt_call_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nxt_put_dt = if let Some(dt) = account.nxt_put_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rt_flag_new = account.rt_flag_new;
    out_acc.rt_cd_new = account.rt_cd_new;
    out_acc.ucid = account.ucid;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_llg = account.ia_llg;
    out_acc.balm_llg = account.balm_llg;
    out_acc.coupon_freq = account.coupon_freq;
    out_acc.nxt_repricing_dt = if let Some(dt) = account.nxt_repricing_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_repricing_dt = if let Some(dt) = account.lst_repricing_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.as_on_dt = rbdate::timestamp(account.as_on_dt);
    out_acc.int_basis = account.int_basis;
    out_acc.int_calc_typ = account.int_calc_typ;
    out_acc.cust_typ = account.cust_typ;
    out_acc.npa_typ = account.npa_typ;
    out_acc.bmid = account.bmid;
    out_acc.division = account.division;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
