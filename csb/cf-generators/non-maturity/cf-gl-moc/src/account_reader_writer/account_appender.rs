use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.gl_cd = acc.gl_cd;
    out_acc.dr_bal = acc.dr_bal;
    out_acc.cr_bal = acc.cr_bal;
    out_acc.amt = acc.amt;
    out_acc.ccy = acc.ccy;
    out_acc.br_cd = acc.br_cd;
    out_acc.typ = acc.typ;
    out_acc.gl_desc = acc.gl_desc;
    out_acc.w4b_cd = acc.w4b_cd;
    out_acc.w4b_desc = acc.w4b_desc;
    out_acc.balm_llg = acc.balm_llg;
    out_acc.care_llg = acc.care_llg;
    out_acc.ba_llg = acc.ba_llg;

    out_acc
}
