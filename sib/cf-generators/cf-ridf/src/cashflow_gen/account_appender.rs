use cashflow_gen::account_reader::input_account::InputAccount;
use cashflow_gen::account_with_cashflows::AccountWithCashflows;
use rbdate::{date_from_timestamp, timestamp};
use statics::*;

use super::account_with_cashflows::Cashflow;
// use super::gen_cashflows::Derive_additional_fields;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    total_inst_amt: f64,
    cashflows: Vec<Cashflow>,
) -> (AccountWithCashflows, f64, f64, usize) {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.tranche_desc = acc.transche_desc;
    out_acc.date_of_modification = if let Some(dt) = acc.date_of_mobilization {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.deposit = acc.deposit;
    out_acc.rate_of_interest = acc.rate_of_interest;
    out_acc.due_date = if let Some(dt) = acc.due_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repaid_on = acc.repaid_on;
    out_acc.repay_amt = acc.repay_amt;
    out_acc.os_bal = acc.os_bal;
    out_acc.tr_dt = if let Some(dt) = acc.tr_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.tr_type = acc.tr_type;
    out_acc.cmuser = acc.cmuser;
    out_acc.cmdate = if let Some(dt) = acc.cmdate {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.vuser = acc.vuser;
    out_acc.vdate = if let Some(dt) = acc.vdate {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.curr = acc.curr;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());
    let (mut prin_amt, mut int_amt) = (0.0, 0.0);
    for cf in out_acc.cashflows.iter() {
        int_amt += cf.get_int_amt();
        prin_amt += cf.get_prin_amt();
    }
    (out_acc, int_amt, prin_amt, cashflows.len())
}
