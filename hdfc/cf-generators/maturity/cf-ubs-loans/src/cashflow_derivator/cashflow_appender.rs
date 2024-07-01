use super::derive_default_ftp_flag::get_default_ftp_flag;
use super::tenor_calculations::get_months;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    nxt_rep_dt: Option<NaiveDate>,
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
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.val_dt = if let Some(dt) = account.val_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.user_def_stats = account.user_def_stats;
    out_acc.prod_cd = account.prod_cd;
    out_acc.gl = account.gl;
    out_acc.curr = account.curr;
    out_acc.prin_ost_bal = account.prin_ost_bal;
    out_acc.spread = account.spread;
    out_acc.compmis1 = account.compmis1;
    out_acc.compmis2 = account.compmis2;
    out_acc.compmis3 = account.compmis3;
    out_acc.rt_flag_new = account.rt_flag_new;
    out_acc.rt_cd_new = account.rt_cd_new;
    out_acc.division = account.division;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_llg = account.ia_llg;
    out_acc.balm_llg = account.balm_llg;
    out_acc.repricing_freq = account.repricing_freq;
    out_acc.nxt_repricing_dt = if let Some(dt) = nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lst_repricing_dt = if let Some(dt) = account.lst_repricing_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_basis = account.int_basis;
    out_acc.cust_typ = account.cust_typ;
    out_acc.npa_typ = account.npa_typ;
    out_acc.bmid = account.bmid;
    out_acc.concat = account.concat;
    out_acc.cntr_party = account.cntr_party;
    out_acc.lcy_amount = account.lcy_amount;
    out_acc.raw_benchmark = account.raw_benchmark;
    out_acc.der_int_rate = account.der_int_rate;
    out_acc.bnchmrk_rate = account.bnchmrk_rate;
    out_acc.spread_val = account.spread_val;
    out_acc.fully_floating_flg = account.fully_floating_flg;
    out_acc.call_option_date = if let Some(dt) = account.call_option_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.put_option_date = if let Some(dt) = account.put_option_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.is_acc_weaker = account.is_acc_weaker;
    out_acc.ews_weaker_value = account.ews_weaker_value;
    out_acc.frequency = account.frequency;
    out_acc.gl_desc = account.gl_description;
    out_acc.ratecode = account.ratecode;
    out_acc.ratespread = account.ratespread;
    out_acc.bdp_div = account.bdp_division;
    out_acc.bdp_coa = account.bdp_coa;
    out_acc.retail = account.retail;
    out_acc.prod_desc = account.prod_desc;
    out_acc.yldgrp_al = account.yldgrp_al;
    out_acc.concat2_point = account.concat2_point;
    out_acc.psl_category = account.psl_category;
    out_acc.lrd_udf = if let Some(dt) = account.lrd_udf {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.nrd_udf = if let Some(dt) = account.nrd_udf {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.old_rt_typ = account.old_rt_typ;

    // TODO: Remove such calculations from cf
    out_acc.def_ftp_flag = get_default_ftp_flag(
        account.val_dt,
        account.mat_dt,
        account.lst_repricing_dt,
        account.nxt_repricing_dt,
        &out_acc.rt_flag_new,
    );
    out_acc.org_tenor = get_months(account.val_dt, account.mat_dt);
    out_acc.resid_tenor = get_months(account.lst_repricing_dt, account.nxt_repricing_dt);
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.sma_flag = account.sma_flag;
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
