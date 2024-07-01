use super::process::*;
use crate::cf_date_iterator::CFDateIterator;
use crate::structs::*;
use sdb_day_convention::days_with_convn;

pub fn generate_compounding_interest_cfs(acc_data: AccData) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();
    let start_date = acc_data.acc_start_date;
    let maturity_date = acc_data.maturity_date;
    let compound_interest_advance_by_months = acc_data.comp_freq.unwrap() as usize;
    let pay_cf_advance_by_months = get_pay_freq(acc_data.int_payout_freq.as_ref().unwrap());
    let i_r = acc_data.int_rate;
    let as_on_date = acc_data.as_on_date;
    let cf_date_iterator = CFDateIterator::new(
        pay_cf_advance_by_months,
        &start_date,
        &maturity_date,
        as_on_date,
    );
    let int_pay_freq = get_pay_freq(acc_data.int_payout_freq.as_ref().unwrap());
    let mut last_pay_date = get_last_pay_date(start_date, &as_on_date, int_pay_freq);
    for cashflow_date in cf_date_iterator {
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = acc_data.ost_bal;
        let mut should_break_loop = false;
        loop {
            if should_break_loop {
                break;
            }
            let mut next_cf_compounding_date = rbdate::incr_dt_by_mon_presrv_eom(
                last_pay_date,
                compound_interest_advance_by_months,
            )
            .unwrap();

            if next_cf_compounding_date >= cashflow_date {
                next_cf_compounding_date = cashflow_date;
                should_break_loop = true;
            }

            let days = days_with_convn(
                last_pay_date,
                next_cf_compounding_date,
                &acc_data.convention,
            )
            .unwrap();
            let pre_pay_month_idx =
                get_curr_month_idx(cashflow_date, acc_data.as_on_date, int_pay_freq);
            let pre_payment_rate = acc_data
                .pre_payment_rates
                .get(pre_pay_month_idx)
                .unwrap_or(&0.0);
            let pre_payment_amt = compounding_principal_amount * pre_payment_rate/100.0;
            compounding_principal_amount -= pre_payment_amt;
            let new_i_amount = interest_amount(compounding_principal_amount, i_r, days);

            interest_to_pay_amount += new_i_amount;
            compounding_principal_amount += new_i_amount;
            last_pay_date = next_cf_compounding_date;
        }

        let cf = new_cashflow(interest_to_pay_amount, 0.0, &cashflow_date);
        cashflows.push(cf);
    }

    cashflows
        .last_mut()
        .expect("Cashflows vec has no value after account matured.")
        .prin_amt = acc_data.ost_bal;

    cashflows
}
