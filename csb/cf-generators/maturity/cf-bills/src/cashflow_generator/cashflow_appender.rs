use super::tenor_calculations::get_months;
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
    out_acc.bill_id = account.bill_id;
    out_acc.client_id = account.client_id;
    out_acc.clients_name = account.clients_name;
    out_acc.lbm_gl = account.lbm_gl;
    out_acc.prod_cd = account.prod_cd;
    out_acc.bal = account.bal;
    out_acc.curr = account.curr;
    out_acc.bal_lcy = account.bal_lcy;
    out_acc.open_dt = if let Some(dt) = account.open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mat_dt = if let Some(dt) = account.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.int_type = account.int_type;
    out_acc.int_rt = account.int_rt;
    out_acc.next_rep_dt = if let Some(dt) = account.next_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.last_rep_dt = if let Some(dt) = account.last_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.rep_freq = account.rep_freq;
    out_acc.benchmark = account.benchmark;
    out_acc.cust_class = account.cust_class;
    out_acc.npa_class = account.npa_class;
    out_acc.ason = if let Some(dt) = account.last_rep_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.client_type = account.client_type;
    out_acc.clients_name1 = account.clients_name1;
    out_acc.clients_bsr_type_flg = account.clients_bsr_type_flg;
    out_acc.clients_busdivn_code = account.clients_busdivn_code;
    out_acc.clients_const_code = account.clients_const_code;
    out_acc.clients_pan_gir_num = account.clients_pan_gir_num;
    out_acc.clients_risk_categorization = account.clients_risk_categorization;
    out_acc.clients_risk_cntry = account.clients_risk_cntry;
    out_acc.clients_segment_code = account.clients_segment_code;
    out_acc.corpcl_orgn_qualifier = account.corpcl_orgn_qualifier;
    out_acc.corpcl_indus_code = account.corpcl_indus_code;
    out_acc.corpcl_nature_of_bus1 = account.corpcl_nature_of_bus1;
    out_acc.corpcl_nature_of_bus2 = account.corpcl_nature_of_bus2;
    out_acc.corpcl_nature_of_bus3 = account.corpcl_nature_of_bus3;
    out_acc.corpcl_central_state_flg = account.corpcl_central_state_flg;
    out_acc.corpcl_public_sector_flg = account.corpcl_public_sector_flg;
    out_acc.corpcl_primary_dlr_flg = account.corpcl_primary_dlr_flg;
    out_acc.corpcl_multilateral_bank = account.corpcl_multilateral_bank;
    out_acc.corpcl_connp_inv_num = account.corpcl_connp_inv_num;
    out_acc.corpcl_bc_gross_turnover = account.corpcl_bc_gross_turnover;
    out_acc.w4b_cd = account.w4b_cd;
    out_acc.balm_llg = account.balm_llg;
    out_acc.care_llg = account.care_llg;
    out_acc.ba_llg = account.ba_llg;
    out_acc.asset_code = account.asset_code;
    out_acc.npa_dt = if let Some(dt) = account.npa_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.account_balance = account.account_balance;
    out_acc.pwo = account.pwo;
    out_acc.written_off_dt = if let Some(dt) = account.written_off_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ho_balance = account.ho_balance;
    out_acc.npa_provision = account.npa_provision;
    out_acc.ho_provision = account.ho_provision;
    out_acc.suspencebalance = account.suspence_bal;
    out_acc.suspence_writeoff = account.suspence_writeoff;
    out_acc.ho_suspence = account.ho_suspence;
    out_acc.claim = account.claim;
    out_acc.primary = account.primary;
    out_acc.collateral = account.collateral;
    out_acc.total_security = account.total_security;
    out_acc.primary_valuation_dt = if let Some(dt) = account.primary_valuation_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.collateral_valuation_dt = if let Some(dt) = account.collateral_valuation_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gold_deficit = account.gold_deficit;
    out_acc.fraud = account.fraud;
    out_acc.wilful_default = account.wilful_default;
    out_acc.subsidy = account.subsidy;
    out_acc.priority = account.priority;
    out_acc.priority_type = account.priority_type;
    out_acc.main_sector = account.main_sector;
    out_acc.sub_sector = account.sub_sector;
    out_acc.activity = account.activity;
    out_acc.industry = account.industry;
    out_acc.categoryofborrower = account.category_of_borrower;
    out_acc.org_gl_head = account.org_gl_head;
    out_acc.npa_amt = account.npa_amt;
    out_acc.sanc_dt = if let Some(dt) = account.sanc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.occp_cd = account.occp_cd;
    out_acc.sens_sec = account.sens_sec;
    out_acc.prior_subtype = account.prior_subtype;
    out_acc.restruct_flag = account.restruct_flag;
    out_acc.restruct_dt = if let Some(dt) = account.restruct_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mor_prd = account.mor_prd;
    out_acc.rating = account.rating;
    out_acc.consitin = account.consitin;
    out_acc.pan = account.pan;
    out_acc.limit_amt = account.limit_amt;
    out_acc.gross_adv = account.gross_adv;
    out_acc.exp_amt = account.exp_amt;
    out_acc.unvail_amt = account.unvail_amt;
    out_acc.gold_gram = account.gold_gram;
    out_acc.fund_flag = account.fund_flag;
    out_acc.pt_i64_1 = account.pt_i64_1;
    out_acc.pt_i64_2 = account.pt_i64_2;
    out_acc.pt_i64_3 = account.pt_i64_3;
    out_acc.pt_i64_4 = account.pt_i64_4;
    out_acc.pt_i64_5 = account.pt_i64_5;
    out_acc.pt_f64_1 = account.pt_f64_1;
    out_acc.pt_f64_2 = account.pt_f64_2;
    out_acc.pt_f64_3 = account.pt_f64_3;
    out_acc.pt_f64_4 = account.pt_f64_4;
    out_acc.pt_f64_5 = account.pt_f64_5;
    out_acc.clients_group_code = account.clients_group_code;
    out_acc.class1 = account.class1;
    out_acc.class2 = account.class2;
    out_acc.class3 = account.class3;
    out_acc.actual_cust_care_value = account.actual_cust_care_value;
    out_acc.actual_acc_care_value = account.actual_acc_care_value;
    out_acc.int_benchmark = account.int_benchmark;
    out_acc.bank_name = account.bank_name;
    out_acc.cet = account.pt_f64_2.to_string();

    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.tot_int_amt = tot_int_amt;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
