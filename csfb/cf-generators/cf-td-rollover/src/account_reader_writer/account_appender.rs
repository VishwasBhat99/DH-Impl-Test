use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::{Cashflow, OutputAccount};
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.account_number = account.account_number;
    out_acc.branch_code = account.branch_code;
    out_acc.cust_id = account.cust_id;
    out_acc.group_id = account.group_id;
    out_acc.cust_name = account.cust_name;
    out_acc.currency = account.currency;
    out_acc.product_code = account.product_code;
    out_acc.gl_code = account.gl_code;
    out_acc.acc_open_date = if let Some(date) = account.acc_open_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.curr_outstanding_bal = account.curr_outstanding_bal;
    out_acc.curr_outstanding_bal_lcy = account.curr_outstanding_bal_lcy;
    out_acc.interest_rate = account.interest_rate;
    out_acc.accr_int_amt = account.accr_int_amt;
    out_acc.accr_int_gl_code = account.accr_int_gl_code;
    out_acc.init_dep_amount = account.init_dep_amount;
    out_acc.init_dep_amount_lcy = account.init_dep_amount_lcy;
    out_acc.mat_date = if let Some(date) = account.mat_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.int_accrual_basis = account.int_accrual_basis;
    out_acc.int_comp_type = account.int_comp_type;
    out_acc.int_pay_freq = account.int_pay_freq;

    out_acc.next_int_pay_date = if let Some(date) = account.next_int_pay_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.comp_freq = account.comp_freq;
    out_acc.next_comp_date = if let Some(date) = account.next_comp_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.pledge_against_loan = account.pledge_against_loan;
    out_acc.loan_acc_no = account.loan_acc_no;
    out_acc.loan_acc_mat_date = if let Some(date) = account.loan_acc_mat_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.constitution = account.constitution;
    out_acc.roi_category = account.roi_category;
    out_acc.contract_no = account.contract_no;
    out_acc.stable_deposit = account.stable_deposit;

    out_acc.effective_mat_date = if let Some(date) = account.effective_mat_date {
        timestamp(date)
    } else {
        DEFAULT_INT
    };
    out_acc.days_till_report = account.days_till_report;
    out_acc.volatility = account.volatility;
    out_acc.period_of_deposits = account.period_of_deposits;
    out_acc.premature_ratio = account.premature_ratio;
    out_acc.overall_rollover_ratio = account.overall_rollover_ratio;
    out_acc.rollover_ratio_non_volatile = account.rollover_ratio_non_volatile;
    out_acc.non_rollover_ratio_non_volatile = account.non_rollover_ratio_non_volatile;
    out_acc.non_rollover_ratio_volatile = account.non_rollover_ratio_volatile;
    out_acc.td_overdue_flag = account.td_overdue_flag;
    out_acc.financial_client = account.financial_client;
    out_acc.lcr_category = account.lcr_category;
    out_acc.line_of_credit = account.line_of_credit;
    out_acc.turnover = account.turnover;
    out_acc.add_field1 = account.add_field1;
    out_acc.add_field2 = account.add_field2;
    out_acc.add_field3 = account.add_field3;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
