use super::super::account::AccountData;
use super::super::op_gen::op_account::Account;
use rbdate::timestamp;

pub fn create_account(acc: AccountData) -> Account {
    let mut out_acc = Account::new();
    out_acc.acc_id = acc.acc_id;
    out_acc.acc_open_date = timestamp(acc.acc_open_date);
    out_acc.os_amount = acc.os_amount;
    out_acc.currency = acc.currency;

    out_acc
}
