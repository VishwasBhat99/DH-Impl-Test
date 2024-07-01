use cashflow_derivator::account_reader::InputAccount;
use cashflow_derivator::account_without_cashflows::OutputAccount;
use macros;
use rbdate::NaiveDate;
use rbdate::{timestamp, DateParser};
use slog::Logger;
use statics::*;

pub fn create_account_without_cashflows(account: InputAccount, log: &Logger) -> OutputAccount {
    let mut out_acc = OutputAccount::new();
    let dmy_date_parser = DateParser::new("%d-%b-%Y".to_string(), false);

    out_acc.account_id = account.account_id;
    out_acc.cf_date = timestamp(match dmy_date_parser.parse_opt(account.cf_date.trim()) {
        Some(val) => val,
        None => {
            log_error!(log, "Could not parse date: '{}'", account.cf_date);

            NaiveDate::from_ymd(1970, 1, 1)
        }
    });
    out_acc.cf_amount = account
        .cf_amount
        .trim()
        .replace(",", "")
        .parse::<f64>()
        .unwrap_or(DEFAULT_FLOAT);
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };

    out_acc
}
