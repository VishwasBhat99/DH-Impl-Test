use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{timestamp, NaiveDate};
use slog::Logger;
use statics::*;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut cf_date = account.maturity_date.expect("Error Reading Mat-Date");
    let mut os_bal = account.outstanding_bal;
    let emi_amt = account.emi_amt;
    let mut cashflows: Vec<Cashflow> = Vec::new();

    while cf_date >= as_on_dt && os_bal >= emi_amt {
        cashflows.push(new_cashflow(DEFAULT_FLOAT, emi_amt, timestamp(cf_date)));
        cf_date = rbdate::decr_dt_by_mon_presrv_eom(cf_date, 1).expect("Error Reading Cf-Date");
        os_bal -= emi_amt;

        if cf_date < as_on_dt {
            cf_date = rbdate::incr_dt_by_mon_presrv_eom_checked(cf_date, 1)
                .expect("Error Reading Cf-Date");
            cashflows.push(new_cashflow(DEFAULT_FLOAT, os_bal, timestamp(cf_date)));
            log_debug!(
                log,
                "Adjusting Amount: {:?} to Date: {:?} for Account: {:?}",
                os_bal,
                cf_date,
                account.card_no
            );
            break;
        }

        if os_bal < emi_amt && os_bal > 0.0 {
            cashflows.push(new_cashflow(DEFAULT_FLOAT, os_bal, timestamp(cf_date)));
            log_debug!(
                log,
                "Remaining Amount: {:?} written into Date: {:?} for Account: {:?}",
                os_bal,
                cf_date,
                account.card_no
            );
            break;
        }
    }

    cashflows
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
