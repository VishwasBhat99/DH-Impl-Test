use process::account_reader::input_account::InputAccount;
use process::account_with_cashflows::Account;
use rbdate::{timestamp, timestamp_to_naivedate, NaiveDate};

pub fn append_cf_data(out_acc: &mut Account, account_data: &InputAccount, ason: NaiveDate) {
    out_acc.account_number = account_data.acc_no.to_string();
    out_acc.disbursed_amt = account_data.disbursed_amt;
    out_acc.os_loan_bal_local_currency = account_data.os_loan_bal_local_currency;
    out_acc.curr_applicable_interest_rate = account_data.curr_applicable_interest_rate;
    out_acc.ei_amount_current = account_data.ei_amount_current;
    out_acc.interest_type = account_data.interest_type.to_string();
    out_acc.os_p_bal_due_local_currency = account_data.os_p_bal_due_local_currency;
    out_acc.os_i_bal_due_local_currency = account_data.os_i_bal_due_local_currency;
    out_acc.ei_amt_paid_advance_local_curr = account_data.ei_amt_paid_advance_local_curr;
    out_acc.pre_ei_bal_local_curr = account_data.pre_ei_bal_local_curr;
    out_acc.account_open_value_date = match account_data.account_open_value_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.maturity_date = match account_data.maturity_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.ei_start_date_current = match account_data.ei_start_date_current {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.ei_end_date_current = match account_data.ei_end_date_current {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.ei_payment_frequency_current = account_data.ei_payment_frequency_current.to_string();
    out_acc.emi_last_paid_date_current = match account_data.emi_last_paid_date_current {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.ei_payment_day = account_data.ei_payment_day.to_string();
    out_acc.ei_orginal_term = account_data.ei_orginal_term.to_string();
    out_acc.ei_balance_term = account_data.ei_balance_term.to_string();
    out_acc.repricing_benchmark = account_data.repricing_benchmark.to_string();
    out_acc.spread = account_data.spread.to_string();
    out_acc.last_repricing_date = match account_data.last_repricing_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.next_repricing_date = match account_data.next_repricing_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.repricing_frequency = account_data.repricing_frequency.to_string();
    out_acc.number_ei_structures = account_data.number_ei_structures;
    out_acc.npa_classification = account_data.npa_classification.to_string();
    out_acc.remark = account_data.remark.to_string();
    out_acc.months_os_comb = account_data.months_os_comb.to_string();
    out_acc.moratorium_type = account_data.moratorium_type.to_string();
    out_acc.from_moratorium_date = match account_data.from_moratorium_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.to_moratorium_date = match account_data.to_moratorium_date {
        Some(date) => date.and_hms(0, 0, 0).timestamp(),
        None => 0,
    };
    out_acc.recalculate_ei_amount_flag = account_data.recalculate_ei_amount_flag.to_string();
    out_acc.moratorium_interest_calculation =
        account_data.moratorium_interest_calculation.to_string();
    out_acc.bullet_payment_flag = account_data.bullet_payment_flag.to_string();
    out_acc.restructured_flag = account_data.restructured_flag.to_string();
    out_acc.residential_mortgage = account_data.residential_mortgage.to_string();
    out_acc.risk_weight = account_data.risk_weight.to_string();
    out_acc.internal_rating = account_data.internal_rating.to_string();
    out_acc.external_rating = account_data.external_rating.to_string();
    out_acc.contractual_tenor = account_data.contractual_tenor.to_string();
    out_acc.residual_tenor = account_data.residual_tenor.to_string();
    out_acc.customer_constitution_code = account_data.customer_constitution_code.to_string();
    out_acc.product_code = account_data.product_code.to_string();
    out_acc.p_gl_code = account_data.p_gl_code.to_string();
    out_acc.m_npa_classification = account_data.m_npa_classification.to_string();
    out_acc.accrued_interest = account_data.accrued_interest.to_string();
    out_acc.customer_id = account_data.customer_id.to_string();
    out_acc.customer_name = account_data.customer_name.to_string();
    out_acc.group_id = account_data.group_id.to_string();
    out_acc.group_name = account_data.group_name.to_string();
    out_acc.branch_code = account_data.branch_code.to_string();
    out_acc.sector = account_data.sector.to_string();
    out_acc.industry = account_data.industry.to_string();
    out_acc.ltv = account_data.ltv.to_string();
    out_acc.overdue_account = account_data.overdue_account.to_string();
    out_acc.excess_account = account_data.excess_account.to_string();
    out_acc.loan_type = account_data.loan_type.to_string();
    out_acc.residual_interest = account_data.residual_interest.to_string();
    out_acc.currency = account_data.currency.to_string().to_string();
    out_acc.hdfc_ltd_percentage = account_data.hdfc_ltd_percentage;
    out_acc.securitization_percentage = account_data.securitization_percentage;
    out_acc.overdue_type = account_data.overdue_type.to_string();
    out_acc.alm_line = account_data.alm_line.to_string();
    out_acc.asondate = timestamp(ason);
    out_acc.emi_overdue_gl_cd = account_data.emi_overdue_gl_cd;
    out_acc.pre_emi_overdue_gl_cd = account_data.pre_emi_overdue_gl_cd;
    out_acc.excess_emi_gl_cd = account_data.excess_emi_gl_cd;
    out_acc.excess_pre_emi_gl_cd = account_data.excess_pre_emi_gl_cd;
    out_acc.lcr_fin_non_fin_flag = account_data.lcr_fin_non_fin_flag.to_string();
    out_acc.undrawn_loans = account_data.undrawn_loans;
    out_acc.undrawn_ccod = account_data.undrawn_ccod;
    out_acc.purpose = account_data.purpose.to_string();
    out_acc.drawing_power = account_data.drawing_power;
    out_acc.tenor = account_data.tenor.to_string();
    out_acc.turn_over = account_data.turn_over.to_string();
    out_acc.line_of_activity = account_data.line_of_activity.to_string();
    out_acc.rating = account_data.rating.to_string();
    out_acc.net_weight = account_data.net_wt.to_string();
    out_acc.curr_os_bal = account_data.curr_os_bal;
}
