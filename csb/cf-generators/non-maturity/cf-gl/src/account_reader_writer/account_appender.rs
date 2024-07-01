use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.gl_cd = acc.gl_cd;
    out_acc.os_bal = acc.os_bal;
    out_acc.ccy = acc.ccy;
    out_acc.os_bal_lcy = acc.os_bal_lcy;
    out_acc.br_cd = acc.br_cd;
    out_acc.dr = acc.dr;
    out_acc.cr = acc.cr;
    out_acc.gl_desc = acc.gl_desc;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.w4b_desc = acc.w4b_desc;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;
    out_acc.cf_type = acc.cf_type;
    out_acc.is_acc_gl = acc.is_acc_gl;

    out_acc
}
