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

    out_acc.accno = account.accno;

    out_acc.branchcode = account.branchcode;
    out_acc.custno = account.custno;
    out_acc.uccid = account.uccid;
    out_acc.ccy = account.ccy;
    out_acc.product = account.product;
    out_acc.acc_date = if let Some(dt) = account.acc_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl_code = account.gl_code;
    out_acc.glcode_compounded_portion = account.glcode_compounded_portion;
    out_acc.glcode_int_accrued = account.glcode_int_accrued;
    out_acc.deposit_date = if let Some(dt) = account.deposit_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.initial_deposit_amount = account.initial_deposit_amount;
    out_acc.initial_dep_amtlcy = account.initial_dep_amtlcy;
    out_acc.current_outstanding_bal =
        if let Some(current_outstanding_bal) = account.current_outstanding_bal {
            current_outstanding_bal
        } else {
            DEFAULT_FLOAT
        };
    out_acc.current_outstandingbal_lcy = account.current_outstandingbal_lcy;
    out_acc.cum_interest = account.cum_interest;
    out_acc.cum_interest_amt_lcy = account.cum_interest_amt_lcy;
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc.interest_type = account.interest_type;
    out_acc.interst_acrrual_basis = account.interst_acrrual_basis;
    out_acc.interest_accured_amount = account.interest_accured_amount;
    out_acc.interest_compution_type = account.interest_compution_type;
    out_acc.interest_rate = if let Some(interest_rate) = account.interest_rate {
        interest_rate
    } else {
        DEFAULT_FLOAT
    };
    out_acc.interest_payment_freq = account.interest_payment_freq;
    out_acc.next_int_payment_dt = account.next_int_payment_dt;
    out_acc.compounding_freq = account.compounding_freq;
    out_acc.next_compounding_dt = if let Some(dt) = account.next_compounding_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.floating_rate_benchmark = account.floating_rate_benchmark;
    out_acc.spread = account.spread;
    out_acc.next_repricing_dt = if let Some(dt) = account.next_repricing_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.repricing_frequency = account.repricing_frequency;
    out_acc.non_withdrawable_flag = account.non_withdrawable_flag;
    out_acc.noticedays = account.noticedays;
    out_acc.lockin_till_dt = if let Some(dt) = account.lockin_till_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.dep_pledged_against_loan_yn = account.dep_pledged_against_loan_yn;
    out_acc.customerconstitutioncode_1 = account.customerconstitutioncode_1;
    out_acc.customerconstitutioncode_2 = account.customerconstitutioncode_2;
    out_acc.period_months = account.period_months;
    out_acc.period_days = account.period_days;
    out_acc.intrest_craeted_upto = if let Some(dt) = account.intrest_craeted_upto {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.interest_accrued_upto = if let Some(dt) = account.interest_accrued_upto {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.f_15hyear = account.f_15hyear;
    out_acc.customer_name = account.customer_name;
    out_acc.total_principal_amount = account.total_principal_amount;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
