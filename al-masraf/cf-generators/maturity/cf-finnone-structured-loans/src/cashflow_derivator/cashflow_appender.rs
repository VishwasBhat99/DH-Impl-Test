use super::tenor_calculations::get_months;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut tot_int_amt = DEFAULT_FLOAT;
    let mut tot_prin_amt = DEFAULT_FLOAT;
    let mut out_acc = AccountWithCashflows::new();

    out_acc.account_number = input_account.account_number;
    out_acc.org_date = {
        if let Some(dt) = input_account.org_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.branch = input_account.branch;
    out_acc.curr_code = input_account.curr_code;
    out_acc.intt_rate = input_account.intt_rate;
    out_acc.product_code = input_account.product_code;
    out_acc.mat_date = {
        if let Some(dt) = input_account.mat_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.rate_flag = input_account.rate_flag;
    out_acc.repricing_index = {
        if input_account.repricing_index == "" {
            "NA".to_string()
        } else {
            input_account.repricing_index
        }
    };
    out_acc.psl = input_account.psl;
    out_acc.npa = input_account.npa;
    out_acc.indv_corp_flag = input_account.indv_corp_flag;
    out_acc.customer_type = input_account.customer_type;
    out_acc.asset_class_id = input_account.asset_class_id;
    out_acc.customer_id = input_account.customer_id;
    out_acc.prod_type = input_account.prod_type;
    out_acc.final_int_rate = input_account.final_int_rate;
    out_acc.cost_centre = input_account.cost_centre;
    out_acc.alm_line = input_account.alm_line;
    out_acc.coa = input_account.coa;
    out_acc.division = input_account.division;
    out_acc.rep_freq = input_account.rep_freq;
    out_acc.next_repricing_date = {
        if let Some(dt) = input_account.next_repricing_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.last_repricing_date = {
        if let Some(dt) = input_account.last_repricing_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.asset_class = input_account.asset_class;
    out_acc.al_line = input_account.al_line;
    out_acc.balm_l2 = input_account.balm_l2;
    out_acc.bmid = input_account.bmid;
    out_acc.ia_line = input_account.ia_line;
    out_acc.scheme_id = input_account.scheme_id;
    out_acc.weaker_code = input_account.weaker_code;
    out_acc.der_int_rate = input_account.der_int_rate;
    out_acc.bnchmrk_rate = input_account.bnchmrk_rate;
    out_acc.spread = input_account.spread;
    out_acc.fully_floating_flg = input_account.fully_floating_flg;
    out_acc.gr_ofs_gl = input_account.gr_ofs_gl;
    out_acc.org_tenor = get_months(input_account.org_date, input_account.mat_date);
    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.accrual_basis = input_account.accrual_basis;
    out_acc.accrued_interest = input_account.accrued_interest;
    out_acc.current_bal = input_account.current_bal;
    out_acc.due_date = {
        if let Some(dt) = input_account.due_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.interest_pay_freq = input_account.interest_pay_freq;
    out_acc.original_balance = input_account.original_balance;
    out_acc.orig_term = input_account.orig_term;
    out_acc.emi = input_account.emi;
    out_acc.payment_freq = input_account.payment_freq;
    out_acc.payment_type = input_account.payment_type;
    out_acc.dpd = input_account.dpd;
    out_acc.customer_name = input_account.customer_name;
    out_acc.inst_st_dt = {
        if let Some(dt) = input_account.inst_st_dt {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.weaker = input_account.weaker;
    out_acc.current_book_balance = input_account.current_book_balance;
    out_acc.first_inst_date = {
        if let Some(dt) = input_account.first_inst_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.inst_num = input_account.inst_num;
    out_acc.num_inst_paid = input_account.num_inst_paid;
    out_acc.last_inst_date = {
        if let Some(dt) = input_account.last_inst_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.gr_dr = input_account.gr_dr;
    out_acc.gr_cr = input_account.gr_cr;
    out_acc.re_dr = input_account.re_cr;
    out_acc.is_dr = input_account.is_dr;
    out_acc.is_cr = input_account.is_cr;
    out_acc.ui_dr = input_account.ui_dr;
    out_acc.ui_cr = input_account.ui_cr;
    out_acc.is_ofs_gl = input_account.is_ofs_gl;
    out_acc.re_ofs_gl = input_account.re_ofs_gl;
    out_acc.ui_ofs_gl = input_account.ui_ofs_gl;
    out_acc.as_on_date = {
        if let Some(dt) = input_account.as_on_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };
    out_acc.A1 = input_account.A1;
    out_acc.A2 = input_account.A2;
    out_acc.A3 = input_account.A3;
    out_acc.A4 = input_account.A4;
    out_acc.A5 = input_account.A5;
    out_acc.A6 = input_account.A6;
    out_acc.A7 = input_account.A7;
    out_acc.A8 = input_account.A8;
    out_acc.A9 = input_account.A9;
    out_acc.A10 = input_account.A10;
    out_acc.A11 = input_account.A11;
    out_acc.A12 = input_account.A12;
    out_acc.A13 = input_account.A13;
    out_acc.A14 = input_account.A14;
    out_acc.A15 = input_account.A15;
    out_acc.A16 = input_account.A16;
    out_acc.A17 = input_account.A17;
    out_acc.A18 = input_account.A18;
    out_acc.A19 = input_account.A19;
    out_acc.A20 = input_account.A20;
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
