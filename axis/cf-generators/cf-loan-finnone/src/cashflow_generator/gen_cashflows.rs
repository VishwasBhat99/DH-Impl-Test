use cashflow_generator::account_with_cashflows::Cashflow;
use rbdate;
use rbdate::NaiveDate;

pub fn generate_cashflows(prin_amt: f64, int_amt: f64, maturity_date: NaiveDate) -> Cashflow {
    return new_cashflow(int_amt, prin_amt, maturity_date);
}

fn new_cashflow(i_a: f64, p_a: f64, d: NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = rbdate::timestamp(d);

    cf
}
