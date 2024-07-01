use super::tenor_calculations::get_months;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    input_account: InputAccount,
    mut cashflows: Vec<Cashflow>,
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
        if let Some(dt) = input_account.next_repricing_date {
            timestamp(dt)
        } else {
            DEFAULT_INT
        }
    };

    if cashflows.len() > 0 {
        let last_cf = cashflows.remove(cashflows.len()-1);
        out_acc.mat_date = last_cf.date;
        cashflows.push(last_cf);
    }
    let rate_flag = input_account.rate_flag.to_ascii_uppercase();
    out_acc.rate_flag = rate_flag.clone();
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
    
    if rate_flag.to_uppercase() == "TIERED FIXED" || rate_flag.to_uppercase() == "FIXED" {
        out_acc.next_repricing_date = out_acc.mat_date;
    }
    else {
        out_acc.next_repricing_date = {
            if let Some(dt) = input_account.next_repricing_date {
                timestamp(dt)
            } else {
                DEFAULT_INT
            }
        };
    }

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
    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
