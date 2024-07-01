use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    Ok(vec![new_cashflow(
        account.amt,
        0.0,
        rbdate::timestamp(account.flow_date.unwrap_or(*config_params.as_on_date())),
    )])
}

fn new_cashflow(prin_amt: f64, int_amt: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.principal_amount = prin_amt;
    cf.interest_amount = int_amt;
    cf.date = d;

    cf
}
