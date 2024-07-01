use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::timestamp;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.dev_dt = timestamp(acc.dev_dt);
    out_acc.ccy = acc.ccy;
    out_acc.amt = acc.amt;

    out_acc
}
