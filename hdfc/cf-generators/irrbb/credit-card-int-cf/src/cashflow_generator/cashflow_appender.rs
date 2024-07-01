use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use protobuf;
use rbdate;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut account_with_cashflows = AccountWithCashflows::new();

    account_with_cashflows.account_number = account.account_number;
    account_with_cashflows.maturity_date =
        rbdate::timestamp(account.maturity_date.expect("Cannot parse Maturity Date"));
    account_with_cashflows.next_payment_date = {
        if account.next_payment_date.is_some() {
            rbdate::timestamp(
                account
                    .next_payment_date
                    .expect("Cannot parse Next Payment Date"),
            )
        } else {
            0
        }
    };
    account_with_cashflows.currency_code = account.currency_code;
    account_with_cashflows.int_pay_freq = account.int_pay_freq;
    account_with_cashflows.int_rate = account.int_rate;
    account_with_cashflows.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    account_with_cashflows
}
