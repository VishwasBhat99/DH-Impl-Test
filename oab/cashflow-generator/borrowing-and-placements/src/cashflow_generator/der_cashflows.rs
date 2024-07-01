use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let as_on_dt: i64 = timestamp(*config_params.as_on_date());

    let prin_amt: f64 = account.cf_principal_amount;
    let int_amt: f64 = account.cf_interest_amount;
    let mut cf_dt = match account.cf_date {
        Some(dt) => timestamp(dt),
        None => timestamp(*config_params.as_on_date()),
    };

    // Case 1: When CFPAmt or Outstanding Balance is negative
    if account.outstanding_bal < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.account_id,
        );
    }

    // Case 2: When CF Date < AsOn Date (Overdue)
    if cf_dt < as_on_dt {
        cf_dt = timestamp(account.start_date);
    }

    // Case 3: When CF Date is null/empty
    if cf_dt == 0 {
        cf_dt = timestamp(account.maturity_date);
    }

    // Case 4: When CF Date > Maturity Date
    if cf_dt > timestamp(account.maturity_date) {
        log_info!(log, "`overdue` for account: `{}`.", account.account_id,);
        cf_dt = timestamp(account.start_date);
    }

    new_cashflow(int_amt, prin_amt, cf_dt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
