use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use protobuf;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut out_acc = AccountWithCashflows::new();
    let mut tot_int_amt = 0.0;
    let mut tot_prin_amt = 0.0;

    out_acc.account_id = account.account_id;
    out_acc.reference = account.reference;
    out_acc.start_date = rbdate::timestamp(account.start_date);
    out_acc.maturity_date = rbdate::timestamp(account.maturity_date);
    out_acc.outstanding_bal = account.outstanding_bal;
    out_acc.currency = if account.currency == "" {
        "NA".to_string()
    } else {
        account.currency
    };
    out_acc.lcy_amount = account.lcy_amount;
    out_acc.customer_type = account.customer_type;

    for cf in &cashflows {
        tot_int_amt += cf.int_amt;
        tot_prin_amt += cf.prin_amt;
    }

    out_acc.total_interest_amount = tot_int_amt;
    out_acc.total_principal_amount = tot_prin_amt;
    out_acc.cashflows = protobuf::RepeatedField::from_vec(cashflows.clone());

    out_acc
}
