use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    //  out_acc.date = acc.date;
    out_acc.date = if let Some(dt) = acc.date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.segment = acc.segment;
    out_acc.sub_segment = acc.sub_segment;
    out_acc.member_id = acc.member_id;
    out_acc.member_name = acc.member_name;
    out_acc.isin = acc.isin;
    out_acc.security_desc = acc.security_desc;

    //  out_acc.mat_date = acc.mat_date;
    out_acc.mat_date = if let Some(dt) = acc.mat_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.face_value = acc.face_value;
    out_acc.face_val_treps = acc.face_val_treps;
    out_acc.balance = acc.balance;
    out_acc.isin_cred_lend = acc.isin_cred_lend;
    out_acc.security_des = acc.security_des;
    out_acc.face_val_rec = acc.face_val_rec;

    out_acc.mar_val = acc.mark_val;
    out_acc.book_val = acc.book_val;
    out_acc.os_amt = acc.os_amt;

    out_acc
}
