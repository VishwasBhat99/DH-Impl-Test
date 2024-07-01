use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{timestamp, NaiveDate};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let maturity_date = match account.maturity_date {
        Some(m_dt) => m_dt,
        None => {
            log_error!(
                log,
                "`maturity_date` is not well-formatted for account: `{}`.",
                account.account_id,
            );

            *config_params.as_on_date()
        }
    };

    let start_date = match account.start_date {
        Some(i_dt) => i_dt,
        None => {
            log_error!(
                log,
                "`start_date` is not well-formatted for account: `{}`.",
                account.account_id,
            );

            *config_params.as_on_date()
        }
    };

    // Calculation of Principal
    let prin_amt: f64 = account.outstanding_bal;

    // Calculation of Interest
    let since = NaiveDate::signed_duration_since;
    let date_diff = since(maturity_date, start_date).num_days() as f64;
    let int_amt: f64 = (account.outstanding_bal * account.repo_rate * date_diff) / 36500.0;

    new_cashflow(int_amt, prin_amt, timestamp(maturity_date))
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
