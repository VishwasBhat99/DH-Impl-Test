use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::timestamp;
use statics::*;

pub fn create_account_with_cashflows(
    acc: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.acc_no = acc.acc_no;
    out_acc.br_cd = acc.br_cd;
    out_acc.cust_no = acc.cust_no;
    out_acc.ucc_id = acc.ucc_id;
    out_acc.ccy = acc.ccy;
    out_acc.produ = acc.produ;
    out_acc.gl = acc.gl;
    out_acc.gl_comp_portn = acc.gl_comp_portn;
    out_acc.open_dt = if let Some(dt) = acc.open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.effect_dt = if let Some(dt) = acc.effect_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.os_bal = acc.os_bal;
    out_acc.os_bal_cry = acc.os_bal_cry;
    out_acc.int_comp_type = acc.int_comp_type;
    out_acc.comp_int_amt = acc.comp_int_amt;
    out_acc.int_rt = acc.int_rt;
    out_acc.mat_dt = if let Some(dt) = acc.mat_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.dep_amt = acc.dep_amt;
    out_acc.dep_amt_cry = acc.dep_amt_cry;
    out_acc.int_amt = acc.int_amt;
    out_acc.int_acrd = acc.int_acrd;
    out_acc.non_with_flag = acc.non_with_flag;
    out_acc.notice_day = acc.notice_day;
    out_acc.const_cd = acc.const_cd;
    out_acc.const_desc = acc.const_desc;
    out_acc.resid_days = acc.resid_days;
    out_acc.cntrct_days = acc.cntrct_days;
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
    out_acc.corpcl_central_state_flg = acc.corpcl_central_state_flg;
    out_acc.corpcl_public_sector_flg = acc.corpcl_public_sector_flg;
    out_acc.corpcl_primary_dlr_flg = acc.corpcl_primary_dlr_flg;
    out_acc.corpcl_multilateral_bank = acc.corpcl_multilateral_bank;
    out_acc.corpcl_connp_inv_num = acc.corpcl_connp_inv_num;
    out_acc.corpcl_bc_gross_turnover = acc.corpcl_bc_gross_turnover;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }
    out_acc.tot_int_amt = tot_int_amt;
    out_acc.tot_prin_amt = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
