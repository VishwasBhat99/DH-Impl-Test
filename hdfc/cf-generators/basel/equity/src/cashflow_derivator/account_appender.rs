use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_without_cashflows::OutputAccount;

pub fn create_account_without_cashflows(
    account: InputAccount,
    index_flag: String,
) -> OutputAccount {
    let mut out_acc = OutputAccount::new();

    out_acc.deal_no = account.deal_no;
    out_acc.isin = account.isin;
    out_acc.book_value = account.book_value;
    out_acc.market_value = account.market_value;
    out_acc.index_flag = index_flag;

    out_acc
}
