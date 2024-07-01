use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.acc_no = acc.acc_no;
    out_acc.branch_cd = acc.branch_cd;
    out_acc.cust_no = acc.cust_no;
    out_acc.ucc_id = acc.ucc_id;
    out_acc.ccy = acc.ccy;
    out_acc.produ = acc.produ;
    out_acc.gl = acc.gl;
    out_acc.open_dt = if let Some(dt) = acc.open_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    }; 
    out_acc.os_bal = acc.os_bal;
    out_acc.os_bal_cry = acc.os_bal_cry;
    out_acc.int_rt = acc.int_rt;
    out_acc.int_type = acc.int_type;
    out_acc.int_bm = acc.int_bm;
    out_acc.spread = acc.spread;
    out_acc.inoperative = acc.inoperative;
    out_acc.int_accrd = acc.int_accrd;
    out_acc.const_cd = acc.const_cd;
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
    out_acc.acc_mt_dt = acc.acc_mt_dt;
    out_acc.orginal_dep_amt_lcy = acc.orginal_dep_amt_lcy;
    out_acc.current_outstanding_amt = acc.current_outstanding_amt;
    out_acc.current_outstanding_amt_lcy = acc.current_outstanding_amt_lcy;
    out_acc.res_tenor = acc.res_tenor;
    out_acc.cont_tenor = acc.cont_tenor;
    out_acc.rep_tenor = acc.rep_tenor;
    out_acc.comp_freq = acc.comp_freq;
    out_acc.cust_cons_code = acc.cust_cons_code;
    out_acc.industry = acc.industry;
    out_acc.division = acc.division;
    out_acc.cust_initial_dep_total_amount = acc.cust_initial_dep_total_amount;
    out_acc.cust_total_deposit_amount = acc.cust_total_deposit_amount;
    out_acc.is_withdrawable = acc.is_withdrawable;
    out_acc.is_custody_ac = acc.is_custody_ac;
    out_acc.is_clearing_ac = acc.is_clearing_ac;
    out_acc.is_cash_managment = acc.is_cash_managment;
    out_acc.is_tax_saving = acc.is_tax_saving;
    out_acc.is_under_lien = acc.is_under_lien;
    out_acc.is_wealth_mang = acc.is_wealth_mang;
    out_acc.pta1 = acc.pta1;
    out_acc.pta2 = acc.pta2;
    out_acc.pta3 = acc.pta3;
    out_acc.pta4 = acc.pta4;
    out_acc.pta5 = acc.pta5;

    out_acc
}
