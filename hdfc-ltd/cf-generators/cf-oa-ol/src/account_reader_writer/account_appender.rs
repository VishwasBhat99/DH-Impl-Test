use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::{Cashflow, OutputAccount};
use rbdate::*;
use slog::Logger;
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
    log: &Logger,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    out_acc.id = account.id;
    out_acc.desc = account.desc;
    out_acc.amt = account.amt;
    out_acc.flow_date = if let Some(dt) = account.flow_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    out_acc.gl_code = account.gl_code;
    out_acc.inflow_outflow = account.inflow_outflow;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
