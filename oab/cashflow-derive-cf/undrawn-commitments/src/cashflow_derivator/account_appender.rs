use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_without_cashflows::OutputAccount;
use rbdate::timestamp;
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, _log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    // Standard Fields
    out_acc.account_id = account.account_id;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.funded = account.funded;
    out_acc.non_funded = account.non_funded;
    out_acc.limit_structure = account.limit_structure;
    out_acc.customer_loc = account.customer_loc;
    out_acc.ctp = account.ctp;
    out_acc.expiry_date = if let Some(dt) = account.expiry_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    out_acc
}
