use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_without_cashflows::OutputAccount;
use slog::Logger;

pub fn create_account_without_cashflows(account: InputAccount, _log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.gl_item = account.gl_item;
    out_acc.branch = account.branch;
    out_acc.basic = account.basic;
    out_acc.suffix = account.suffix;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.cf_amount = account.cf_amount;
    out_acc.balance_in_omr = account.balance_in_omr;

    out_acc
}
