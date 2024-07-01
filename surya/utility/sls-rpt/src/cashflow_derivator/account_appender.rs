use cashflow_derivator::account_without_cashflows::Account;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;

pub fn create_account_without_cashflows(
    llg_id: String,
    amt: f64,
    currency: String,
) -> Account {
    let mut out_acc = Account::new();

    out_acc.llg_id = llg_id;
    out_acc.ccy = currency;
    out_acc.amt = amt;

    out_acc
}
