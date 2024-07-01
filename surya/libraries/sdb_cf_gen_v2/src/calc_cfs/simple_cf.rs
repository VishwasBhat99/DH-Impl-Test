use super::process::*;
use crate::cf_date_iterator::CFDateIterator;
use crate::structs::*;
use sdb_day_convention::days_with_convn;

pub fn generate_simple_interest_cfs(acc_data: AccData) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();

    let mut o_a = acc_data.ost_bal;
    let i_r = acc_data.int_rate;

    let int_pay_freq = get_pay_freq(acc_data.int_payout_freq.as_ref().unwrap());
    let mut last_cf_date =
        get_last_pay_date(acc_data.acc_start_date, &acc_data.as_on_date, int_pay_freq);
    let cf_date_iterator = CFDateIterator::new_from_account(&acc_data);

    for cashflow_date in cf_date_iterator {
        let days = days_with_convn(last_cf_date, cashflow_date, &acc_data.convention).unwrap();

        let pre_pay_month_idx =
            get_curr_month_idx(cashflow_date, acc_data.as_on_date, int_pay_freq);
        let pre_payment_rate = acc_data
            .pre_payment_rates
            .get(pre_pay_month_idx)
            .unwrap_or(&0.0);
        let pre_payment_amt = (pre_payment_rate / 100.0) as f64 * o_a;
        o_a -= pre_payment_amt;

        let i_a = interest_amount(o_a, i_r, days);
        let cf = new_cashflow(i_a, pre_payment_amt, &cashflow_date);
        cashflows.push(cf);

        last_cf_date = cashflow_date;
    }

    cashflows
        .last_mut()
        .expect("Cashflows matured without generating any cashflows.")
        .prin_amt = o_a;

    cashflows
}
