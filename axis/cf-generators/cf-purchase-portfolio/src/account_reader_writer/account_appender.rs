use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Account;
use account_reader_writer::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use rbdate::NaiveDate;
pub fn create_account_with_cashflows(acc: InputAccount, cashflows: Vec<Cashflow>) -> Account {
    let mut out_acc = Account::new();
    //Calculate emi value before writing to output
    let mut sum = 0.0;
    out_acc.id = acc.id;
    out_acc.account_number = acc.account_number;
    out_acc.cust_id = acc.cust_id;
    out_acc.deal_name = acc.deal_name;
    out_acc.principal_os = acc.principal_os;
    out_acc.deal_start_date = timestamp(acc.deal_start_date);
    out_acc.cf_start_date = timestamp(acc.cf_start_date);
    out_acc.cf_end_date = timestamp(acc.cf_end_date);
    out_acc.accrued_interest = acc.accrued_interest;
    out_acc.deal_value = acc.deal_value;
    out_acc.gl_code = acc.gl_code;
    out_acc.system_value = acc.system_value;
    out_acc.current_nominal_interest_rate = acc.current_nominal_interest_rate;
    out_acc.product_type = acc.product_type;
    out_acc.originator_name = acc.originator_name;
    out_acc.contract_yield = acc.contract_yield;
    out_acc.payment_frequency = acc.payment_frequency;
    out_acc.loan_reset_frequency = acc.loan_reset_frequency;
    out_acc.interest_rate_type = acc.interest_rate_type;
    out_acc.next_reset_date = timestamp(acc.next_reset_date);
    out_acc.borrower_constitution = acc.borrower_constitution;
    out_acc.pan = acc.pan;
    out_acc.voter_id = acc.voter_id;
    out_acc.external_benchmark = acc.external_benchmark;
    out_acc.dpd_in_days = acc.dpd_in_days;
    out_acc.daily_dpd_reported_date = timestamp(acc.daily_dpd_reported_date);
    out_acc.due_from_customer = acc.due_from_customer;
    out_acc.cmonth_emi_due = acc.cmonth_emi_due;
    out_acc.actual_amount_paid = acc.actual_amount_paid;
    out_acc.principal_due_cmonth = acc.principal_due_cmonth;
    out_acc.principal_rcvd_cmonth = acc.principal_rcvd_cmonth;
    out_acc.interest_method_code = acc.interest_method_code;
    out_acc.bank_share = acc.bank_share;
    out_acc.originator_share = acc.originator_share;
    out_acc.customer_od_bank_share = acc.customer_od_bank_share;
    out_acc.customer_od_originator_share = acc.customer_od_originator_share;
    out_acc.maturity_date = timestamp(acc.maturity_date);
    out_acc.exposure_unique_id = acc.exposure_unique_id;
    out_acc.fic_mis_date = timestamp(acc.fic_mis_date);
    out_acc.system_date = timestamp(acc.system_date);
    out_acc.balm_control_status_id = acc.balm_control_status_id;
    out_acc.derived_principal = acc.derived_principal;
    out_acc.derived_cmonth_emi_due = acc.derived_cmonth_emi_due;
    out_acc.npa_classification = acc.npa_classification;
    out_acc.cust_hlth_code = acc.cust_hlth_code;
    out_acc.cust_npa_class = acc.cust_npa_class;
    out_acc.final_npa_class = acc.final_npa_class;
    out_acc.currency = acc.currency;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
