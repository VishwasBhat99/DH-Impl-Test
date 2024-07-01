use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::Duration;
use rbdate::{increment_date_by_months_unchecked, NaiveDate};

pub fn derive_cashflows(as_on_date: NaiveDate, account: &InputAccount) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();

    let date = as_on_date;

    let dt1 = date + Duration::days(1);
    cashflows.push(new_cashflow(0.0, account.day1_sum, dt1));

    let dt2 = date + Duration::days(2);
    cashflows.push(new_cashflow(0.0, account.day2_sum, dt2));

    let dt3 = date + Duration::days(3);
    cashflows.push(new_cashflow(0.0, account.day3_sum, dt3));

    let dt4 = date + Duration::days(4);
    cashflows.push(new_cashflow(0.0, account.day4_sum, dt4));

    let dt5 = date + Duration::days(5);
    cashflows.push(new_cashflow(0.0, account.day5_sum, dt5));

    let dt6 = date + Duration::days(6);
    cashflows.push(new_cashflow(0.0, account.day6_sum, dt6));

    let dt7 = date + Duration::days(7);
    cashflows.push(new_cashflow(0.0, account.day7_sum, dt7));

    let dt14 = date + Duration::days(14);
    cashflows.push(new_cashflow(0.0, account.day8_14_sum, dt14));

    let dt28 = date + Duration::days(28);
    cashflows.push(new_cashflow(0.0, account.day15_28_sum, dt28));

    let dt30 = date + Duration::days(30);
    cashflows.push(new_cashflow(0.0, account.day29_30_sum, dt30));

    let dt2m = increment_date_by_months_unchecked(date, 2);
    cashflows.push(new_cashflow(0.0, account.m2_sum, dt2m));

    let dt3m = increment_date_by_months_unchecked(date, 3);
    cashflows.push(new_cashflow(0.0, account.m3_sum, dt3m));

    let dt4m = increment_date_by_months_unchecked(date, 4);
    cashflows.push(new_cashflow(0.0, account.m4_sum, dt4m));

    let dt5m = increment_date_by_months_unchecked(date, 5);
    cashflows.push(new_cashflow(0.0, account.m5_sum, dt5m));

    let dt6m = increment_date_by_months_unchecked(date, 6);
    cashflows.push(new_cashflow(0.0, account.m6_sum, dt6m));

    let dt7m = increment_date_by_months_unchecked(date, 7);
    cashflows.push(new_cashflow(0.0, account.m7_sum, dt7m));

    let dt8m = increment_date_by_months_unchecked(date, 8);
    cashflows.push(new_cashflow(0.0, account.m8_sum, dt8m));

    let dt10m = increment_date_by_months_unchecked(date, 10);
    cashflows.push(new_cashflow(0.0, account.m10_sum, dt10m));

    let dt11m = increment_date_by_months_unchecked(date, 11);
    cashflows.push(new_cashflow(0.0, account.m11_sum, dt11m));

    let dt12m = increment_date_by_months_unchecked(date, 12);
    cashflows.push(new_cashflow(0.0, account.m12_sum, dt12m));

    let dt3y = increment_date_by_months_unchecked(date, 12 * 3);
    cashflows.push(new_cashflow(0.0, account.y1_3_sum, dt3y));

    let dt5y = increment_date_by_months_unchecked(date, 12 * 5);
    cashflows.push(new_cashflow(0.0, account.y3_5_sum, dt5y));

    let dt7y = increment_date_by_months_unchecked(date, 12 * 7);
    cashflows.push(new_cashflow(0.0, account.y5_7_sum, dt7y));

    let dt8y = increment_date_by_months_unchecked(date, 12 * 8);
    cashflows.push(new_cashflow(0.0, account.y7_8_sum, dt8y));

    let dt9y = increment_date_by_months_unchecked(date, 12 * 9);
    cashflows.push(new_cashflow(0.0, account.y8_9_sum, dt9y));

    let dt10y = increment_date_by_months_unchecked(date, 12 * 10);
    cashflows.push(new_cashflow(0.0, account.y9_10_sum, dt10y));

    let dt15y = increment_date_by_months_unchecked(date, 12 * 15);
    cashflows.push(new_cashflow(0.0, account.y10_15_sum, dt15y));

    let dt16y = increment_date_by_months_unchecked(date, 12 * 16);
    cashflows.push(new_cashflow(0.0, account.y15_sum, dt16y));

    cashflows
}

fn new_cashflow(i_a: f64, p_a: f64, d: NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = rbdate::timestamp(d);

    cf
}
