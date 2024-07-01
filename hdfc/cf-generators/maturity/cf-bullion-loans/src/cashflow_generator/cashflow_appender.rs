use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;

use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.acc_no = account.acc_no;
    out_acc.counter_party = account.cntr_party;
    out_acc.ccy = account.ccy;
    out_acc.gl_no = account.gl_no;
    out_acc.amt = if let Some(amt) = account.amt {
        amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.int_rt = if let Some(rt) = account.int_rt {
        rt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.st_dt = if let Some(dt) = account.st_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.alm_line = account.alm_line;
    out_acc.div = account.div;
    out_acc.cust_typ = account.cust_typ;
    out_acc.compmis1 = account.compmis1;
    out_acc.compmis2 = account.compmis2;
    out_acc.compmis3 = account.compmis3;
    out_acc.prod_category = account.prod_category;
    out_acc.ia_line = account.ia_line;
    out_acc.nxt_rep_dt = DEFAULT_INT;
    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.sma_flag = account.sma_flag;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
