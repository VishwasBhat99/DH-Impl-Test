use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use macros;
use rbdate;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.casa_acc_no = account.casa_acc_no;
    out_acc.casa_prod_cd = account.casa_prod_cd;
    out_acc.acc_br_cd = account.acc_br_cd;
    out_acc.book_bal = account.book_bal;
    out_acc.flex_cube_cust_id = account.flex_cube_cust_id;
    out_acc.acc_open_dt = if let Some(dt) = account.acc_open_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.cust_shrt_name = account.cust_shrt_name;
    out_acc.cust_id = account.asset_bal_gl;
    out_acc.cbr_num_1 = account.cbr_num_1;
    out_acc.cbr_num_2 = account.cbr_num_2;
    out_acc.cbr_num_3 = account.cbr_num_3;
    out_acc.cr_rt = if let Some(rt) = account.cr_rt {
        rt
    } else {
        log_error!(
            log,
            "`cr_rate` is not well formatted for account : `{}`.",
            out_acc.casa_acc_no
        );
        DEFAULT_FLOAT
    };
    out_acc.dr_rt = if let Some(rt) = account.dr_rt {
        rt
    } else {
        log_error!(
            log,
            "`dr_rate` is not well formatted for account : `{}`.",
            out_acc.casa_acc_no
        );
        DEFAULT_FLOAT
    };
    out_acc.prod_name = account.prod_name;
    out_acc.component = account.component;
    out_acc.rt_flg = account.rt_flg;
    out_acc.inst = account.inst;
    out_acc.crnt_book_bal = if let Some(bal) = account.crnt_book_bal {
        bal
    } else {
        log_error!(
            log,
            "`current_book_balance` not well formatted for account : `{}`.",
            out_acc.casa_acc_no
        );
        DEFAULT_FLOAT
    };
    out_acc.div = account.div;
    out_acc.alm_line = account.alm_line;
    out_acc.ia_llg = account.ia_llg;
    out_acc.balm_llg = account.balm_llg;
    out_acc.int_index_name = account.int_index_name;
    out_acc.npa_flg = account.npa_flg;
    out_acc.int_rt = if out_acc.component == "UBSDD" {
        out_acc.cr_rt
    } else if out_acc.component == "UBSOD" {
        out_acc.dr_rt
    } else {
        log_error!(
            log,
            "`component` is not well-formatted for account : `{}`.",
            out_acc.casa_acc_no
        );
        DEFAULT_FLOAT
    };

    log_debug!(
        log,
        "account: `{}`, current book balance: `{}`, interest rate: `{}`.",
        out_acc.casa_acc_no,
        out_acc.crnt_book_bal,
        out_acc.int_rt
    );

    out_acc
}
