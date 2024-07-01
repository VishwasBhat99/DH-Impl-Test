use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use slog::Logger;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.v_src_sys_id= account.v_src_sys_id;
    out_acc.v_exp_id= account.v_exp_id;
    out_acc.v_d_cust_ref_code= account.v_d_cust_ref_code;
    out_acc.v_line_code = account.v_line_code ; 
    out_acc.v_prod_code = account.v_prod_code ;
    out_acc.v_pp_table = account.v_pp_table ;
    out_acc.n_ccf_prcnt = account.n_ccf_prcnt ;
    out_acc.d_exp_strt_dt = if let Some(dt) = account.d_exp_strt_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.d_exp_end_dt = if let Some(dt) = account.d_exp_end_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.n_exp_amt = account.n_exp_amt ;
    out_acc.n_undrawn_amt = account.n_undrawn_amt ;
    out_acc.v_basel_prod_typ_desc_lv1 = account.v_basel_prod_typ_desc_lv1 ;
    out_acc.v_basel_prod_typ_desc = account.v_basel_prod_typ_desc ;
    out_acc.v_basel_asst_class_desc = account.v_basel_asst_class_desc ;
    out_acc.v_party_typ_desc = account.v_party_typ_desc ;
    out_acc.gl_code = account.gl_code ;
    out_acc.v_party_name = account.v_party_name ;
    out_acc.v_ram_id = account.v_ram_id ;
    out_acc.v_ccy_code = account.v_ccy_code ;
    out_acc.v_fclty_desc = account.v_fclty_desc ;
    out_acc.v_ret_corp_ind = account.v_ret_corp_ind ;  
    out_acc.fb_nfb = account.fb_nfb ;
    out_acc.ccod_flag = account.ccod_flag ;
    out_acc.lcr_cat = account.lcr_cat ;
    out_acc.asst_class_desc = account.asst_class_desc ;
    out_acc.final_map_lcr = account.final_map_lcr ;
    out_acc.flag_uncond_cancelled_exp_ind=account.flag_uncond_cancelled_exp_ind;
    out_acc.slr_amt = account.slr_amt ;
    out_acc.lcr_amt = account.lcr_amt ;
    out_acc
}
