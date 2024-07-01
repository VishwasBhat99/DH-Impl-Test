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
    out_acc.br_no = account.br_no;
    out_acc.origin_branch = account.origin_branch;
    out_acc.lac_no = account.lac_no;
    out_acc.laf_no = account.laf_no;
    out_acc.borr_name = account.borr_name;
    out_acc.disb_amt = account.disb_amt;
    out_acc.sanc_amt = account.sanc_amt;
    out_acc.emi = account.emi;
    out_acc.roi = account.roi;
    out_acc.clps_loan_type = account.clps_loan_type;
    out_acc.first_disb_date = if let Some(dt) = account.first_disb_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.last_disb_date = if let Some(dt) = account.last_disb_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.commitment_amt = if let Some(commitment_amt) = account.commitment_amt {
        commitment_amt
    } else {
        DEFAULT_FLOAT
    };
    out_acc.amt_of_disb = account.amt_of_disb;
    out_acc.approval_date = if let Some(dt) = account.approval_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.currency = account.currency;
    out_acc.treasury_glcode = account.treasury_glcode;
    out_acc.app1 = account.app1;
    out_acc.app2 = account.app2;
    out_acc.app3 = account.app3;
    out_acc.app4 = account.app4;
    out_acc.app5 = account.app5;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
