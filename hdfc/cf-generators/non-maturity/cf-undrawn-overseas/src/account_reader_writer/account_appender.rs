use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use macros;
use rbdate::timestamp;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.v_d_cust_ref_code = account.v_d_cust_ref_code;
    out_acc.v_line_code = account.v_line_code;
    out_acc.n_ccf_prcnt = account.n_ccf_prcnt;
    out_acc.n_undrawn_amt = account.n_undrawn_amt;
    out_acc.v_basel_asset_class_desc = account.v_basel_asset_class_desc;
    out_acc.v_party_type_desc = account.v_party_type_desc;
    out_acc.gl_code = account.gl_code;
    out_acc.v_ccy_code = account.v_ccy_code;
    out_acc.branch_code = account.branch_code;
    out_acc.country_code = account.country_code;
    out_acc.lcr_category = account.lcr_category;
    out_acc.asset_class_desc = account.asset_class_desc;
    out_acc.final_mapping_lcr=account.final_mapping_lcr;
    out_acc.f_uncond_cancelled_exp_ind=account.f_uncond_cancelled_exp_ind;
    out_acc.ccod_flag = account.ccod_flag;
    out_acc.fb_nfb = account.fb_nfb;
    out_acc
}
