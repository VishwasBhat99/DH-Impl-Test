use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::Duration;
use macros;
use rbdate;
use rbdate::NaiveDate;
use slog::Logger;

pub fn generate_cashflows(
    as_on_date: &NaiveDate,
    account: &mut InputAccount,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut cashflows = Vec::new();
    let int_amt = 0.0;
    let day1 = *as_on_date + Duration::days(135);
    let day2 = *as_on_date + Duration::days(270);
    let day3 = *as_on_date + Duration::days(720);
    let day4 = *as_on_date + Duration::days(1440);
    let day5 = *as_on_date + Duration::days(2700);

    let prin_amt1 = 3.0 * account.factor;
    let prin_amt2 = 6.0 * account.factor;
    let prin_amt3 = 24.0 * account.factor;
    let prin_amt4 = 24.0 * account.factor;
    let prin_amt5 =
        account.principal_ouststanding_amount - prin_amt1 - prin_amt2 - prin_amt3 - prin_amt4;

    let cf1 = new_cashflow(int_amt, prin_amt1, day1);
    cashflows.push(cf1);
    let cf2 = new_cashflow(int_amt, prin_amt2, day2);
    cashflows.push(cf2);
    let cf3 = new_cashflow(int_amt, prin_amt3, day3);
    cashflows.push(cf3);
    let cf4 = new_cashflow(int_amt, prin_amt4, day4);
    cashflows.push(cf4);
    let cf5 = new_cashflow(int_amt, prin_amt5, day5);
    cashflows.push(cf5);
    Ok(cashflows)
}
fn new_cashflow(i_a: f64, p_a: f64, d: NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = rbdate::timestamp(d);

    cf
}
