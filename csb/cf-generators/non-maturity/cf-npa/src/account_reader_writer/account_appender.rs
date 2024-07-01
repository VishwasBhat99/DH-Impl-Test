use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.acc_no = acc.acc_no;
    out_acc.asset_cd = acc.asset_cd;
    out_acc.acc_bal = acc.acc_bal;
    out_acc.ho_bal = acc.ho_bal;
    out_acc.ho_prov = acc.ho_prov;
    out_acc.npa_amt = acc.npa_amt;
    out_acc.ccy = acc.ccy;

    out_acc
}
