use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
    ccy: &str,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.acc_no = acc.acc_no;
    out_acc.cust_name = acc.cust_name;
    out_acc.client_id = acc.client_id;
    out_acc.tl_limit = acc.tl_limit;
    out_acc.ccod_limit = acc.ccod_limit;
    out_acc.pbg_limit = acc.pbg_limit;
    out_acc.fbg_limit = acc.fbg_limit;
    out_acc.loc_limit = acc.loc_limit;
    out_acc.bliab_bill_limit = acc.bliab_bill_limit;
    out_acc.tl_blnc = acc.tl_blnc;
    out_acc.ccod_blnc = acc.ccod_blnc;
    out_acc.pbg_blnc = acc.pbg_blnc;
    out_acc.fbg_blnc = acc.fbg_blnc;
    out_acc.loc_blnc = acc.loc_blnc;
    out_acc.bliab_bill_blnc = acc.bliab_bill_blnc;
    out_acc.tl_ualimit = acc.tl_ualimit;
    out_acc.ccod_ualimit = acc.ccod_ualimit;
    out_acc.pbg_ualimit = acc.pbg_ualimit;
    out_acc.fbg_ualimit = acc.fbg_ualimit;
    out_acc.loc_ualimit = acc.loc_ualimit;
    out_acc.bliab_bill_ualimit = acc.bliab_bill_ualimit;
    out_acc.tl_dep = acc.tl_dep;
    out_acc.ccod_dep = acc.ccod_dep;
    out_acc.pbg_dep = acc.pbg_dep;
    out_acc.fbg_dep = acc.fbg_dep;
    out_acc.loc_dep = acc.loc_dep;
    out_acc.bliab_bill_dep = acc.bliab_bill_dep;
    out_acc.tl_cr_eq = acc.tl_cr_eq;
    out_acc.ccod_cr_eq = acc.ccod_cr_eq;
    out_acc.pbg_cr_eq = acc.pbg_cr_eq;
    out_acc.fbg_cr_eq = acc.fbg_cr_eq;
    out_acc.loc_cr_eq = acc.loc_cr_eq;
    out_acc.bliab_bill_cr_eq = acc.bliab_bill_cr_eq;
    out_acc.client_type_ip = acc.client_type_ip;
    out_acc.ext_rating = acc.ext_rating;
    out_acc.asset_code = acc.asset_code;
    out_acc.bsr = acc.bsr;
    out_acc.ccy = String::from(ccy);
    out_acc.client_type = acc.client_type;
    out_acc.clients_name = acc.clients_name;
    out_acc.clients_bsr_type_flg = acc.clients_bsr_type_flg;
    out_acc.clients_busdivn_code = acc.clients_busdivn_code;
    out_acc.clients_const_code = acc.clients_const_code;
    out_acc.clients_pan_gir_num = acc.clients_pan_gir_num;
    out_acc.clients_risk_categorization = acc.clients_risk_categorization;
    out_acc.clients_risk_cntry = acc.clients_risk_cntry;
    out_acc.clients_segment_code = acc.clients_segment_code;
    out_acc.corpcl_orgn_qualifier = acc.corpcl_orgn_qualifier;
    out_acc.corpcl_indus_code = acc.corpcl_indus_code;
    out_acc.corpcl_nature_of_bus1 = acc.corpcl_nature_of_bus1;
    out_acc.corpcl_nature_of_bus2 = acc.corpcl_nature_of_bus2;
    out_acc.corpcl_nature_of_bus3 = acc.corpcl_nature_of_bus3;
    out_acc.corpcl_central_state_flg = acc.corpcl_central_state_flg;
    out_acc.corpcl_public_sector_flg = acc.corpcl_public_sector_flg;
    out_acc.corpcl_primary_dlr_flg = acc.corpcl_primary_dlr_flg;
    out_acc.corpcl_multilateral_bank = acc.corpcl_multilateral_bank;
    out_acc.corpcl_connp_inv_num = acc.corpcl_connp_inv_num;
    out_acc.corpcl_bc_gross_turnover = acc.corpcl_bc_gross_turnover;
    out_acc.ccod_undrawn_lcr = acc.ccod_undrawn_lcr;
    out_acc.ccod_und_nsfr = acc.ccod_und_nsfr;
    out_acc.care_funded = acc.care_funded;
    out_acc.care_lcbg = acc.care_lcbg;
    out_acc.sanc_dt = if let Some(dt) = acc.sanc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.occp_cd = acc.occp_cd;
    out_acc.sens_sec = acc.sens_sec;
    out_acc.prior_subtype = acc.prior_subtype;
    out_acc.restruct_flag = acc.restruct_flag;
    out_acc.restruct_dt = if let Some(dt) = acc.restruct_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.mor_prd = acc.mor_prd;
    out_acc.rating = acc.rating;
    out_acc.consitin = acc.consitin;
    out_acc.pan = acc.pan;
    out_acc.limit_amt = acc.limit_amt;
    out_acc.gross_adv = acc.gross_adv;
    out_acc.exp_amt = acc.exp_amt;
    out_acc.unvail_amt = acc.unvail_amt;
    out_acc.gold_gram = acc.gold_gram;
    out_acc.fund_flag = acc.fund_flag;
    out_acc.ltv_value = acc.ltv_value;
    out_acc.pt_i64_1 = acc.pt_i64_1;
    out_acc.pt_i64_2 = acc.pt_i64_2;
    out_acc.pt_i64_3 = acc.pt_i64_3;
    out_acc.pt_i64_4 = acc.pt_i64_4;
    out_acc.pt_i64_5 = acc.pt_i64_5;
    out_acc.pt_f64_1 = acc.pt_f64_1;
    out_acc.pt_f64_2 = acc.pt_f64_2;
    out_acc.pt_f64_3 = acc.pt_f64_3;
    out_acc.pt_f64_4 = acc.pt_f64_4;
    out_acc.pt_f64_5 = acc.pt_f64_5;
    out_acc.pt_str_1 = acc.pt_str_1;
    out_acc.pt_str_2 = acc.pt_str_2;
    out_acc.pt_str_3 = acc.pt_str_3;
    out_acc.pt_str_4 = acc.pt_str_4;
    out_acc.pt_str_5 = acc.pt_str_5;
    out_acc.maturity_dt = if let Some(dt) = acc.maturity_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.ccod_tl_limit = acc.tl_limit + acc.ccod_limit;

    for cf in &cashflows {
        tot_int_amt += cf.interest_amount;
        tot_prin_amt += cf.principal_amount;
    }
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.total_interest_amount = tot_int_amt;

    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
