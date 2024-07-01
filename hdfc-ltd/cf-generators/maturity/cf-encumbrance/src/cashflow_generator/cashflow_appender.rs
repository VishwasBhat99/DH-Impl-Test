use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::{timestamp, NaiveDate};
use statics::*;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.deal_number = account.deal_number;
    out_acc.deal_type = account.deal_type;
    out_acc.collateral_id = account.collateral_id;
    out_acc.collateral_amount =
        if let Some(collateral_amount) = account.collateral_amount {
            collateral_amount
        } else {
            DEFAULT_FLOAT
        };
    out_acc.collateral_market_value = account.collateral_market_value;
    out_acc.currency = account.currency;
    out_acc.location = account.location;
    
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows);
    out_acc
}
