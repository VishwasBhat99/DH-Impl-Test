use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use slog::Logger;
use statics::*;
pub fn gen_cashflow(
    input_account: &InputAccount,
    config_params: &ConfigurationParameters,
    _log: &Logger,
    tot_cfs: &mut usize,
    tot_acc_with_cfs: &mut i64,
    tot_prin_in_ip: &mut f64,
    tot_prin_in_op: &mut f64,
    tot_int_in_op: &mut f64,
    tot_int_in_ip: &mut f64,
) -> Vec<Cashflow> {
    let mut cfs: Vec<Cashflow> = Vec::new();
    let mut cf;
    *tot_acc_with_cfs += 1;
    //starting CF DATE
    let mut cf_date = input_account
        .acct_open_dt
        .unwrap_or(*config_params.as_on_date());
    let mut total_principal = DEFAULT_FLOAT;
    let principal_amount = input_account.a8.parse().unwrap_or(DEFAULT_FLOAT);
    *tot_prin_in_ip += principal_amount;

    let outstanding_amt = input_account.curr_bal;
    let as_on_date = input_account
        .as_on_date
        .unwrap_or(*config_params.as_on_date());
    let interest_rate = input_account.var_int_rate;
    let mut interest_amount = DEFAULT_FLOAT;
    *tot_int_in_ip += interest_amount;
    //Case1:Default Payment
    while cf_date <= as_on_date && outstanding_amt - principal_amount >= total_principal {
        *tot_cfs += 1;
        interest_amount = (principal_amount * interest_rate) / 1200.00;
        cf = new_cashflow(
            interest_amount,
            principal_amount,
            rbdate::timestamp(cf_date),
        );
        cfs.push(cf);
        total_principal += principal_amount;
        cf_date = incr_dt_by_mon_presrv_eom_checked(cf_date, 1 as usize)
            .unwrap_or(*config_params.as_on_date());
        *tot_int_in_op += interest_amount;
        *tot_prin_in_op += principal_amount;
    }
    //Case2: Advance Payment:
    if outstanding_amt < total_principal {
        *tot_cfs += 1;
        cf_date = input_account
            .as_on_date
            .unwrap_or(*config_params.as_on_date());
        cf = new_cashflow(
            interest_amount,
            outstanding_amt - total_principal,
            rbdate::timestamp(cf_date),
        );
        *tot_int_in_op += interest_amount;
        *tot_prin_in_op += outstanding_amt;
        cfs.push(cf);
    }
    cfs
}
pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
