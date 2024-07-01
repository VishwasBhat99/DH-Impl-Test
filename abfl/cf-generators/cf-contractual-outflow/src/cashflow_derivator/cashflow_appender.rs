use crate::cashflow_derivator::account_reader::input_account::InputAccount;
use crate::cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use crate::cashflow_derivator::account_with_cashflows::Cashflow;
use crate::statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    out_acc.serial_no = account.serial_no;
    out_acc.incr_dt = if let Some(dt) = account.incr_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.incr_amt = account.incr_amt;
    out_acc.applicable_dt = if let Some(dt) = account.applicable_dt {
        rbdate::timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.projected_outflow = account.projected_outflow;
    out_acc.lob = account.lob;
    out_acc.currency = account.currency;
    out_acc.add_field1 = account.add_field1;
    out_acc.add_field2 = account.add_field2;
    out_acc.add_field3 = account.add_field3;
    out_acc.add_field4 = account.add_field4;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    out_acc
}
