use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, _log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.key_1 = account.key_1;
    out_acc.gl_class_code = account.gl_class_code;
    out_acc.status = account.status;
    out_acc.balance = account.balance;
    out_acc.old_bad_debt_ind = account.old_bad_debt_ind;
    out_acc.i_or_b = account.i_or_b;
    out_acc.crm_flag = account.crm_flag;
    out_acc.app_amt = account.app_amt;
    out_acc.lmt1 = account.lmt1;
    out_acc.lmt2 = account.lmt2;
    out_acc.lmt3 = account.lmt3;
    out_acc.lmt4 = account.lmt4;
    out_acc.od_lmt = account.od_lmt;
    out_acc.adv_val = account.adv_val;
    out_acc.basel_class = account.basel_class;
    out_acc.limit_exp_date = if let Some(dt) = account.limit_exp_date {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.lending_status = account.lending_status;
    out_acc.dp = account.dp;
    out_acc.drawing_amt = account.drawing_amt;
    out_acc.od_multi_lim_allow = account.od_multi_lim_allow;
    out_acc.ccy = account.ccy;
    out_acc.group = account.group;
    out_acc.llg = account.llg;
    out_acc.limit_amt = account.limit_amt;
    out_acc.dp_amt = account.dp_amt;
    out_acc.undrawn_sls_amt = account.undrawn_sls_amt;
    out_acc.undrawn_lcr_amt = account.undrawn_lcr_amt;
    out_acc.undrawn_nsfr_amt = account.undrawn_nsfr_amt;

    out_acc
}
