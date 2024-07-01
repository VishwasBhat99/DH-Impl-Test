use super::derive_default_ftp_flag::get_default_ftp_flag;
use super::tenor_calculations::get_months;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.reference = account.reference;
    out_acc.cust = account.cust;
    out_acc.curr = account.curr;
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
    out_acc.npa_stats = account.npa_stats;
    out_acc.gl = account.gl;
    out_acc.int_rt = if let Some(int_rt) = account.int_rt {
        int_rt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.cust_name = account.cust_name;
    out_acc.comp_mis1 = account.comp_mis1;
    out_acc.comp_mis2 = account.comp_mis2;
    out_acc.loan_type = account.loan_type;
    out_acc.acurl_basis = account.acurl_basis;
    out_acc.div = account.div;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_llg = account.ia_llg;
    out_acc.balm_llg = account.balm_llg;
    out_acc.as_on_dt = if let Some(dt) = account.as_on_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.asset_class = account.asset_class;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.exchange_rt = account.exchange_rt;
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.bal_os_amt_lcy = account.bal_os_amt_lcy;
    out_acc.org_tenor = get_months(account.int_st_dt, account.mat_dt);
    out_acc.int_st_dt = if let Some(dt) = account.int_st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.def_ftp_flag = get_default_ftp_flag(
        account.int_st_dt,
        account.mat_dt,
        None::<NaiveDate>,
        None::<NaiveDate>,
    );
    out_acc.txn_dt = if let Some(dt) = account.txn_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.bill_amt = account.bill_amt;
    out_acc.concat = account.concat;
    out_acc.rate_flag = account.rate_flag;
    out_acc.comp_mis3 = account.comp_mis3;
    out_acc.is_acc_weaker = account.is_acc_weaker;
    out_acc.ews_weaker_value = account.ews_weaker_value;
    out_acc.sma_flag = account.sma_flag;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
