use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate::NaiveDate;

pub fn generate_cashflows(account: &mut InputAccount) -> Result<Vec<Cashflow>, String> {
    Ok(vec![new_cashflow(
        0.0,
        account.out_amt,
        account.mat_date.unwrap_or_else(|| {
            panic!(
                "Error in Parsing CF-Date (Mat-Date): {:?} for Account: {:?}",
                account.mat_date, account.acct_num
            )
        }),
    )])
}

fn new_cashflow(i_a: f64, p_a: f64, d: NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = rbdate::timestamp(d);

    cf
}
