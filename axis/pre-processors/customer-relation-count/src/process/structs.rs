use rbdate::date_from_timestamp;
use rbdate::NaiveDate;

#[derive(Clone, Debug)]
pub struct OutputData {
    pub as_on_dt: NaiveDate,
    pub cust_id: String,
    pub country: String,
    pub advances_relation: String,
    pub liability_relation: i64,
    pub salary_relation: i64,
    pub constitutio_code: String,
    pub total_amount: f64,
    pub insured_amount: f64,
    pub uninsured_amount: f64,
    pub lcr_category: String,
}

impl OutputData {
    pub fn new() -> OutputData {
        OutputData {
            as_on_dt: date_from_timestamp(0),
            cust_id: "".to_string(),
            country: "".to_string(),
            advances_relation: "".to_string(),
            liability_relation: 0,
            salary_relation: 0,
            constitutio_code: "".to_string(),
            total_amount: 0.0,
            insured_amount: 0.0,
            uninsured_amount: 0.0,
            lcr_category: "".to_string(),
        }
    }
}

pub fn op_data_format(op_value: OutputData) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        op_value.as_on_dt.format("%d-%m-%Y"),
        op_value.cust_id,
        op_value.country,
        op_value.advances_relation,
        op_value.liability_relation,
        op_value.salary_relation,
        op_value.constitutio_code,
        op_value.total_amount,
        op_value.insured_amount,
        op_value.uninsured_amount,
        op_value.lcr_category
    )
}
