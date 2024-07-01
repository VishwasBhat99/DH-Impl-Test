use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use macros;
use protobuf;
use rbdate::timestamp;
use slog::Logger;
use statics::DEFAULT_INT;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
    log: &Logger,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.acc_no = acc.acc_no;
    out_acc.br_cd = acc.br_cd;
    out_acc.cust_id = acc.cust_id;
    out_acc.ucic_id = acc.ucic_id;
    if (acc.ccy == "") {
        log_warn!(log, "Currency not found for account: {}.", out_acc.acc_no);
        out_acc.ccy = "NA".to_string();
    } else {
        out_acc.ccy = acc.ccy;
    }
    out_acc.prod_cd = acc.prod_cd;
    out_acc.gl_cd = acc.gl_cd;
    out_acc.gl_comp_portion = acc.gl_comp_portion;
    out_acc.acc_open_dt = timestamp(acc.acc_open_dt);
    out_acc.effc_dt = if let Some(dt) = acc.effc_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.bal_os = acc.bal_os;
    out_acc.bal_os_cly = acc.bal_os_cly;
    out_acc.int_comp_type = acc.int_comp_type;
    out_acc.compo_int_amt = acc.compo_int_amt;
    out_acc.int_rt = acc.int_rt;
    out_acc.mat_dt = timestamp(acc.mat_dt);
    out_acc.dep_amt = acc.dep_amt;
    out_acc.dep_amt_lcy = acc.dep_amt_lcy;
    out_acc.int_amt = acc.int_amt;
    out_acc.int_acc_amt = acc.int_acc_amt;
    out_acc.non_with_flag = acc.non_with_flag;
    out_acc.notice_day = acc.notice_day;
    out_acc.cust_const_code = acc.cust_const_code;
    out_acc.cntrct_num = acc.cntrct_num;
    out_acc.as_on = if let Some(dt) = acc.as_on {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.comp_freq = acc.comp_freq;
    out_acc.pay_freq = acc.pay_freq;
    out_acc.resid_days = acc.resid_days;
    out_acc.cntrct_days = acc.cntrct_days;
    out_acc.dumy = acc.dumy;
    out_acc.clients_code = acc.clients_code;
    out_acc.client_type = acc.client_type;
    out_acc.clients_name = acc.clients_name;
    out_acc.clients_bsr_type_flg = acc.clients_bsr_type_flg;
    out_acc.clients_busdivn_code = acc.clients_busdivn_code;
    out_acc.clients_const_code = acc.clients_const_code;
    out_acc.clients_cust_sub_catg = acc.clients_cust_sub_catg;
    out_acc.clients_group_code = acc.clients_group_code;
    out_acc.clients_pan_gir_num = acc.clients_pan_gir_num;
    out_acc.clients_risk_categorization = acc.clients_risk_categorization;
    out_acc.clients_risk_cntry = acc.clients_risk_cntry;
    out_acc.clients_segment_code = acc.clients_segment_code;
    out_acc.corpcl_client_name = acc.corpcl_client_name;
    out_acc.corpcl_orgn_qualifier = acc.corpcl_orgn_qualifier;
    out_acc.corpcl_indus_code = acc.corpcl_indus_code;
    out_acc.corpcl_sub_indus_code = acc.corpcl_sub_indus_code;
    out_acc.corpcl_nature_of_bus1 = acc.corpcl_nature_of_bus1;
    out_acc.corpcl_nature_of_bus2 = acc.corpcl_nature_of_bus2;
    out_acc.corpcl_nature_of_bus3 = acc.corpcl_nature_of_bus3;
    out_acc.corpcl_scheduled_bank = acc.corpcl_scheduled_bank;
    out_acc.corpcl_sovereign_flg = acc.corpcl_sovereign_flg;
    out_acc.corpcl_type_of_sovereign = acc.corpcl_type_of_sovereign;
    out_acc.corpcl_cntry_code = acc.corpcl_cntry_code;
    out_acc.corpcl_central_state_flg = acc.corpcl_central_state_flg;
    out_acc.corpcl_public_sector_flg = acc.corpcl_public_sector_flg;
    out_acc.corpcl_primary_dlr_flg = acc.corpcl_primary_dlr_flg;
    out_acc.corpcl_multilateral_bank = acc.corpcl_multilateral_bank;
    out_acc.corpcl_connp_inv_num = acc.corpcl_connp_inv_num;
    out_acc.corpcl_bc_gross_turnover = acc.corpcl_bc_gross_turnover;
    out_acc.t1 = acc.t1;
    out_acc.t2 = acc.t2;
    out_acc.t3 = acc.t3;
    out_acc.t4 = acc.t4;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;
    out_acc.res_tenor = acc.res_tenor;
    out_acc.cont_tenor = acc.cont_tenor;
    out_acc.rep_tenor = acc.rep_tenor;
    out_acc.cust_cons_code = acc.cust_cons_code;
    out_acc.industry = acc.industry;
    out_acc.division = acc.division;
    out_acc.cust_initial_dep_total_amount = acc.cust_initial_dep_total_amount;
    out_acc.cust_total_deposit_amount = acc.cust_total_deposit_amount;
    out_acc.is_with_drawable = acc.is_with_drawable;
    out_acc.is_custody_ac = acc.is_custody_ac;
    out_acc.is_clearing_ac = acc.is_clearing_ac;
    out_acc.is_cash_managment = acc.is_cash_managment;
    out_acc.is_tax_saving = acc.is_tax_saving;
    out_acc.is_under_lien = acc.is_under_lien;
    out_acc.is_wealth_mang = acc.is_wealth_mang;
    out_acc.pta_1 = acc.pta_1;
    out_acc.pta_2 = acc.pta_2;
    out_acc.pta_3 = acc.pta_3;
    out_acc.pta_4 = acc.pta_4;
    out_acc.pta_5 = acc.pta_5;
    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
