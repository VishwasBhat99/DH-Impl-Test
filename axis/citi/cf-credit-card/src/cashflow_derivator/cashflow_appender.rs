use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.product_id = account.product_id;
    out_acc.account_number = account.account_number;
    out_acc.bill_due_day = timestamp(account.bill_due_day);
    out_acc.outstanding_balance_inr = account.outstanding_balance_inr;
    out_acc.currency_loan = account.currency_loan;
    out_acc.rate_of_int = account.rate_of_int;
    out_acc.total_tenure = account.total_tenure;
    out_acc.completed_tenure = account.completed_tenure;
    out_acc.customer_id = account.customer_id;
    out_acc.emi_amount = account.emi_amount;
    out_acc.int_day_count = account.int_day_count;
    out_acc.data_process_date = timestamp(account.data_process_date);
    out_acc.report_date = timestamp(account.report_date);
    out_acc.gl_code = account.gl_code;
    out_acc.installment_frequency = account.installment_frequency;
    out_acc.npa_classification = account.npa_classification;
    out_acc.npa_amount = account.npa_amount;
    out_acc.cust_hlth_code = account.cust_hlth_code;
    out_acc.cust_npa_class = account.cust_npa_class;
    out_acc.final_npa_class = account.final_npa_class;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
