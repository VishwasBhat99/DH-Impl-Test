use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.deal_number = account.deal_number;
    out_acc.book_value =
        if let Some(book_value) = account.book_value {
            book_value
        } else {
            DEFAULT_FLOAT
        };
    out_acc.market_value = account.market_value;
    out_acc.currency = account.currency;
    out_acc.listing_status = account.listing_status;
    out_acc.listing_exchange = account.listing_exchange;
    out_acc.equity_id = account.equity_id;
    out_acc.equity_name = account.equity_name;
    out_acc.equity_issuer_type = account.equity_issuer_type;
    out_acc.issuer_country = account.issuer_country;
    out_acc.isin = account.isin;
    out_acc.asset_type = account.asset_type;
    out_acc.asset_cat = account.asset_cat;
    out_acc.gl_code = account.gl_code;

    out_acc
}
