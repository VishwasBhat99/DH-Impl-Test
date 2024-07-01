use rbdate::NaiveDate;

#[derive(Clone, Debug)]
pub struct ResData {
    pub struct_number: String,
    pub interest_type: String,
    pub expected_interest_rate: f64,
    pub ei_period: String,
    pub ei_amount: f64,
    pub ei_start_date: NaiveDate,
    pub ei_end_date: NaiveDate,
    pub ei_pay_freq: String,
}
