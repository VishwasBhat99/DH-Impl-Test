use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: &InputAccount,
    cashflows: Vec<Cashflow>,
    as_on: NaiveDate,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;
    out_acc.acc_no = account.acc_no.to_string();
    out_acc.disbursed_amt = account.disbursed_amt;
    out_acc.os_loan_bal_lcy = account.os_loan_bal_lcy;
    out_acc.int_rate = account.int_rate;
    out_acc.ei_amt_crnt = account.ei_amt_crnt;
    out_acc.int_type = account.int_type.to_string();
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
    out_acc.ei_pay_freq_crnt = account.ei_pay_freq_crnt.to_string();
    out_acc.emi_last_paid_date_crnt = if let Some(dt) = account.emi_last_paid_date_crnt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ei_pay_day = account.ei_pay_day;
    out_acc.ei_orginal_term = account.ei_orginal_term;
    out_acc.ei_bal_term = account.ei_bal_term;
    out_acc.rep_bm = account.rep_bm.to_string();
    out_acc.spread = account.spread.to_string();
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
    out_acc.rep_freq = account.rep_freq.to_string();
    out_acc.no_ei_structures = account.no_ei_structures;
    out_acc.npa_class = account.npa_class.to_string();
    out_acc.remark = account.remark.to_string();
    out_acc.months_os_comb = account.months_os_comb.to_string();
    out_acc.mor_type = account.mor_type.to_string();
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
    out_acc.recalc_ei_amt_flag = account.recalc_ei_amt_flag.to_string();
    out_acc.mor_int_calc = account.mor_int_calc.to_string();
    out_acc.bullet_pay_flag = account.bullet_pay_flag.to_string();
    out_acc.restrct_flag = account.restrct_flag.to_string();
    out_acc.residential_mortgage = account.residential_mortgage.to_string();
    out_acc.risk_weight = account.risk_weight.to_string();
    out_acc.internal_rating = account.internal_rating.to_string();
    out_acc.external_rating = account.external_rating.to_string();
    out_acc.contractual_tenor = account.contractual_tenor;
    out_acc.residual_tenor = account.residual_tenor;
    out_acc.cust_constitution_code = account.cust_constitution_code.to_string();
    out_acc.prod_code = account.prod_code.to_string();
    out_acc.p_gl_code = account.p_gl_code.to_string();
    out_acc.m_npaclass = account.m_npaclass.to_string();
    out_acc.acrd_int = account.acrd_int;
    out_acc.cust_id = account.cust_id.to_string();
    out_acc.cust_name = account.cust_name.to_string();
    out_acc.group_id = account.group_id.to_string();
    out_acc.group_name = account.group_name.to_string();
    out_acc.branch_code = account.branch_code.to_string();
    out_acc.sector = account.sector.to_string();
    out_acc.industry = account.industry.to_string();
    out_acc.ltv = account.ltv.to_string();
    out_acc.overdue_acc = account.overdue_acc.to_string();
    out_acc.excess_acc = account.excess_acc.to_string();
    out_acc.loan_type = account.loan_type.to_string();
    out_acc.resid_int = account.resid_int;
    out_acc.ccy = account.ccy.to_string();
    out_acc.hdfc_ltd_percent = account.hdfc_ltd_percent;
    out_acc.sec_percent = account.sec_percent;
    out_acc.overdue_type = account.overdue_type.to_string();
    out_acc.alm_line = account.alm_line.to_string();
    out_acc.emi_overdue_gl_cd = account.emi_overdue_gl_cd;
    out_acc.pre_emi_overdue_gl_cd = account.pre_emi_overdue_gl_cd;
    out_acc.excess_emi_gl_cd = account.excess_emi_gl_cd;
    out_acc.excess_pre_emi_gl_cd = account.excess_pre_emi_gl_cd;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.asondate = timestamp(as_on);
    out_acc.structure_number = "0".to_string();
    out_acc.memi = 0.0;
    out_acc.roi = 0.0;
    out_acc.ost_bal = 0.0;
    out_acc.sma_flag = account.sma_flag.to_string();
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
