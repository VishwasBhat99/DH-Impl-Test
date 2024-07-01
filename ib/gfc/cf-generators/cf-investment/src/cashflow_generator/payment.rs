use rbdate::NaiveDate;
#[derive(PartialEq, Clone, Default, Debug)]
pub struct PaymentDetails {
    pub interest_rate: f64,
    pub due_date: NaiveDate,
}

impl PaymentDetails {
    pub fn assignvalues(int_rate: f64, due_date: NaiveDate) -> PaymentDetails {
        PaymentDetails {
            interest_rate: int_rate,
            due_date: due_date,
        }
    }
}
