use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_without_cashflows::OutputAccount;
use rbdate::timestamp;
use statics::*;

pub fn create_account_without_cashflows(acc: InputAccount, ccy: &str) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.as_on=if let Some(dt) = acc.as_on {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.segment=acc.segment;
    out_acc.sub_segment=acc.sub_segment;
    out_acc.member_id=acc.member_id;
    out_acc.member_name=acc.member_name;
    out_acc.cash=acc.cash;
    
    out_acc
}
