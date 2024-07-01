use derive::account::Account;

pub fn create_account_without_cashflows(fields: &[&str], bal: f64, ccy: &str) -> Account {
    let mut account = Account::new();
    account.set_src_system(fields[0].to_string());
    account.set_amt(bal);
    account.set_asset_class(fields[2].to_string());
    account.set_ccy(ccy.to_string());

    account
}
