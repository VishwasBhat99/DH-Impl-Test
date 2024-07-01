use super::tenor_calculations::get_months;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    as_on_date: NaiveDate,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;

    out_acc.fc_ubs_acc = account.fc_ubs_acc;
    out_acc.cust_name = account.cust_name;
    out_acc.pout_bal = if let Some(bal) = account.pout_bal {
        bal
    } else {
        DEFAULT_FLOAT
    };
    out_acc.acc_int = account.acc_int;
    out_acc.st_dt = if let Some(dt) = account.st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.c_dt = if let Some(dt) = account.c_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl_cd = account.gl_cd;
    out_acc.int_rt = if let Some(val) = account.int_rt {
        val
    } else {
        DEFAULT_FLOAT
    };
    out_acc.int_typ = account.int_typ;
    out_acc.int_bmark = account.int_bmark;
    out_acc.rt_flag = account.rt_flag;
    out_acc.prod_cd = account.prod_cd;
    out_acc.nxt_pay_dt = if let Some(dt) = account.nxt_pay_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mis1 = account.mis1;
    out_acc.mis2 = account.mis2;
    out_acc.mis3 = account.mis3;
    out_acc.ccy = account.ccy;
    out_acc.ratings = account.ratings;
    out_acc.rating_agency = account.rating_agency;
    out_acc.asset_class = account.asset_class;
    out_acc.div = account.div;
    out_acc.typ = account.typ;
    out_acc.originator = account.originator;
    out_acc.rep_freq = account.rep_freq;
    out_acc.nxt_rep_dt = if let Some(dt) = account.nxt_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.portfolio = account.portfolio;
    out_acc.alm_line = account.alm_line;
    out_acc.txn_mis2 = account.txn_mis2;
    out_acc.old_fc_ubs_acc = account.old_fc_ubs_acc;
    out_acc.deal_name = account.deal_name;
    out_acc.cf_start_date = if let Some(dt) = account.cf_start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.org_tenor = get_months(account.st_dt, account.c_dt);
    out_acc.resid_tenor = get_months(Option::Some(as_on_date), account.c_dt);
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.ubs_acct_number = account.ubs_acct_number;
    out_acc.sma_flag = account.sma_flag;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
