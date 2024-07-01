use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use rbdate::{timestamp, NaiveDate,num_days_start_to_end};
use statics::*;
use configuration_parameters::ConfigurationParameters;

pub fn create_account_with_cashflows(
    account: InputAccount,
    config_params: &ConfigurationParameters,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    
    out_acc.isin = account.isin;
    out_acc.security_name = account.security_name;
    out_acc.ccy = account.ccy;
    out_acc.face_value = account.face_value;
    out_acc.book_value = if let Some(book_value) = account.book_value {
        book_value
    } else {
        DEFAULT_FLOAT
    };
    out_acc.market_value = account.market_value;
    out_acc.deal_id = account.deal_id;
    out_acc.maturity_date = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };
    let no_of_days = num_days_start_to_end(*config_params.as_on_date(),account.maturity_date.expect("sdxc"));
    if no_of_days <= 180 {
        out_acc.bucket = "LT6M".to_string();
    }
    else if no_of_days >180 &&  no_of_days <= 365 {
        out_acc.bucket = "6M1Y".to_string();
    }
    else {
        out_acc.bucket = "GT1Y".to_string();
    }
    out_acc
}
