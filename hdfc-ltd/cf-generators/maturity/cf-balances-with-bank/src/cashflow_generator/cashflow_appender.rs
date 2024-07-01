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

    out_acc.bank_code = account.bank_code;
    out_acc.acc_open_date = if let Some(dt) = account.acc_open_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.initial_deposit_amount =
        if let Some(initial_deposit_amount) = account.initial_deposit_amount {
            initial_deposit_amount
        } else {
            DEFAULT_FLOAT
        };
    out_acc.initial_deposit_amountlcy = account.initial_deposit_amountlcy;
    out_acc.int_rate = if let Some(int_rate) = account.int_rate {
        int_rate
    } else {
        DEFAULT_FLOAT
    };
    out_acc.int_payment_fq = account.int_payment_fq;
    out_acc.acc_no = account.acc_no;
    out_acc.gl_code = account.gl_code;
    out_acc.slr_nonslr = account.slr_nonslr;
    out_acc.ccy = account.ccy;
    out_acc.product_code = account.product_code;
    out_acc.code_gl = account.code_gl;
    out_acc.holding_period = account.holding_period;
    out_acc.interest_accrued = account.interest_accrued;
    out_acc.broken_quat_int =
    if let Some(broken_quat_int) = account.broken_quat_int {
        broken_quat_int
    } else {
        DEFAULT_FLOAT
    };
    out_acc.app1 = account.app1;
    out_acc.app2 = account.app2;
    out_acc.app3 = account.app3;
    out_acc.app4 = account.app4;
    out_acc.app5 = account.app5;
    out_acc.app6 = account.app6;
    out_acc.app7 = account.app7;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
