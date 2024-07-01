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
    out_acc.def_ftp_flag = get_default_ftp_flag(
        account.val_dt,
        account.mat_dt,
        account.lst_repricing_dt,
        account.nxt_repricing_dt,
    );
    out_acc.org_tenor = get_months(account.val_dt, account.mat_dt);
    out_acc.resid_tenor = get_months(account.lst_repricing_dt, account.nxt_repricing_dt);
    out_acc.lcy_out_amt_usd = account.lcy_out_amt_usd;
    out_acc.lcy_int_amt_usd = account.lcy_int_amt_usd;
    out_acc.penalty_amt_usd = account.penalty_amt_usd;
    out_acc.int_rate = account.int_rate;
    out_acc.int_payout_dt = if let Some(dt) = account.int_payout_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
