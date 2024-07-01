use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use macros;
use protobuf;
use rbdate::timestamp;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
    log: &Logger,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();

    out_acc.account_id = acc.account_id;
    out_acc.customername = acc.customername;
    out_acc.branch_name = acc.branch_name;
    out_acc.customer_no = acc.customer_no;
    out_acc.commonclientcode = acc.commonclientcode;
    out_acc.productcode = acc.productcode;
    out_acc.loan_start_date = if let Some(dt) = acc.loan_start_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = timestamp(acc.maturity_date);
    out_acc.balance_term = acc.balance_term;
    out_acc.sanctionamount = acc.sanctionamount;
    out_acc.disbursed_amount = acc.disbursed_amount;
    out_acc.currency = acc.currency;
    out_acc.principal_ouststanding_amount = acc.principal_ouststanding_amount;
    out_acc.overdue_interest = acc.overdue_interest;
    out_acc.overdue_principal = acc.overdue_principal;
    out_acc.pre_emi_outstanding_amount = acc.pre_emi_outstanding_amount;
    out_acc.pre_emi_remaining = acc.pre_emi_remaining;
    out_acc.interest_type = acc.interest_type;
    out_acc.interest_calulation_method = acc.interest_calulation_method;
    out_acc.number_of_total_emi = acc.number_of_total_emi;
    out_acc.emi_frequency = acc.emi_frequency;
    out_acc.installment_type = acc.installment_type;
    out_acc.revised_lob = acc.revised_lob;
    out_acc.revised_vertical = acc.revised_vertical;
    out_acc.accountstatus = acc.accountstatus;
    out_acc.weightedtenor = acc.weightedtenor;
    out_acc.sumprinout = acc.sumprinout;
    out_acc.avgtenor = acc.avgtenor;
    out_acc.factor = acc.factor;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
