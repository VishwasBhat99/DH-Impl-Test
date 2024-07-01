use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate;
use rbdate::NaiveDate;

pub fn generate_cashflows<'a>(
    prin_amt: f64,
    int_amt: f64,
    mut early_date: &'a NaiveDate,
    maturity_date: &'a NaiveDate,
) -> Cashflow {
    if early_date == &NaiveDate::from_ymd(1970, 1, 1) {
        early_date = maturity_date
    } else if maturity_date < early_date {
        early_date = maturity_date;
    }
    return new_cashflow(int_amt, prin_amt, early_date);
}

fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = rbdate::timestamp(*d);

    cf
}
