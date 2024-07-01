use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;
    out_acc.acc_no = account.acc_no;
    out_acc.disbursed_amt = account.disbursed_amt;
    out_acc.os_loan_bal_lcy = account.os_loan_bal_lcy;
    out_acc.int_rate = account.int_rate;
    out_acc.ei_amt_crnt = account.ei_amt_crnt;
    out_acc.int_type = account.int_type;
    out_acc.os_p_bal_due_local_ccy = account.os_p_bal_due_local_ccy;
    out_acc.os_i_bal_due_local_ccy = account.os_i_bal_due_local_ccy;
    out_acc.ei_amt_paid_adv_lcy = account.ei_amt_paid_adv_lcy;
    out_acc.pre_ei_bal_lcy = account.pre_ei_bal_lcy;
    out_acc.acc_open_value_date = if let Some(dt) = account.acc_open_value_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ei_start_date_crnt = if let Some(dt) = account.ei_start_date_crnt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ei_end_date_crnt = if let Some(dt) = account.ei_end_date_crnt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ei_pay_freq_crnt = account.ei_pay_freq_crnt;
    out_acc.emi_last_paid_date_crnt = if let Some(dt) = account.emi_last_paid_date_crnt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ei_pay_day = account.ei_pay_day;
    out_acc.ei_orginal_term = account.ei_orginal_term;
    out_acc.ei_bal_term = account.ei_bal_term;
    out_acc.rep_bm = account.rep_bm;
    out_acc.spread = account.spread;
    out_acc.last_rep_date = if let Some(dt) = account.last_rep_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.next_rep_date = if let Some(dt) = account.next_rep_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rep_freq = account.rep_freq;
    out_acc.no_ei_structures = account.no_ei_structures;
    out_acc.npa_class = account.npa_class;
    out_acc.remark = account.remark;
    out_acc.months_os_comb = account.months_os_comb;
    out_acc.mor_type = account.mor_type;
    out_acc.from_mor_date = if let Some(dt) = account.from_mor_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.to_mor_date = if let Some(dt) = account.to_mor_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.recalc_ei_amt_flag = account.recalc_ei_amt_flag;
    out_acc.mor_int_calc = account.mor_int_calc;
    out_acc.bullet_pay_flag = account.bullet_pay_flag;
    out_acc.restrct_flag = account.restrct_flag;
    out_acc.residential_mortgage = account.residential_mortgage;
    out_acc.risk_weight = account.risk_weight;
    out_acc.internal_rating = account.internal_rating;
    out_acc.external_rating = account.external_rating;
    out_acc.contractual_tenor = account.contractual_tenor;
    out_acc.residual_tenor = account.residual_tenor;
    out_acc.cust_constitution_code = account.cust_constitution_code;
    out_acc.prod_code = account.prod_code;
    out_acc.p_gl_code = account.p_gl_code;
    out_acc.m_npaclass = account.m_npaclass;
    out_acc.acrd_int = account.acrd_int;
    out_acc.cust_id = account.cust_id;
    out_acc.cust_name = account.cust_name;
    out_acc.group_id = account.group_id;
    out_acc.group_name = account.group_name;
    out_acc.branch_code = account.branch_code;
    out_acc.sector = account.sector;
    out_acc.industry = account.industry;
    out_acc.ltv = account.ltv;
    out_acc.overdue_acc = account.overdue_acc;
    out_acc.excess_acc = account.excess_acc;
    out_acc.loan_type = account.loan_type;
    out_acc.resid_int = account.resid_int;
    out_acc.ccy = account.ccy;
    out_acc.hdfc_ltd_percent = account.hdfc_ltd_percent;
    out_acc.sec_percent = account.sec_percent;
    out_acc.overdue_type = account.overdue_type;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.tot_int_amt = tot_int_amt;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
