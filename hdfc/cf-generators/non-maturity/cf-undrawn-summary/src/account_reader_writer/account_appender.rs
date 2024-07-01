use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use slog::Logger;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.v_src_sys_id = account.v_src_sys_id;
    out_acc.n_ccf_prcnt = account.n_ccf_prcnt;
    out_acc.n_undrawn_amt = account.n_undrawn_amt;
    out_acc.v_basel_asset_class_desc = account.v_basel_asset_class_desc;
    out_acc.v_party_type_desc = account.v_party_type_desc;
    out_acc.v_ccy_code = account.v_ccy_code;
    out_acc.v_exp_amt = account.v_exp_amt;
    out_acc.n_undrawn_amt = account.n_undrawn_amt;
    out_acc.lcr_category = account.lcr_category;
    out_acc.asset_class_desc = account.asset_class_desc;
    out_acc.final_mapping = account.final_mapping;
    out_acc.f_uncond_cancelled_exp_ind=account.f_uncond_cancelled_exp_ind;
    out_acc.slr = account.slr;
    out_acc.lcr = account.lcr;
    out_acc.v_prod_code=account.v_prod_code;
    out_acc
}
