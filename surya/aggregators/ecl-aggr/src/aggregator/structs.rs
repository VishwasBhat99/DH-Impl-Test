#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggrKey {
    pub bucket_id: i32,
    pub cf_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggrData {
    pub data: OutputData,
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputData {
    pub account_num: String,
    pub as_on_date: String, // format: %d/%m/%Y
    pub cf_count: String,
    pub scenario_num: String,
    pub cf_date: String, // format: %d/%m/%Y
    pub data_origin: String,
    pub cf_type: String,
    pub amount: String,
    pub fin_ele_type: String,
    pub currency_code: String,
    pub org_unit_code: String,
    pub date: String, // format: %Y%m%d
    pub n_acc_id: String,
    pub currency_type_code: String,
    pub gl_code: String,
}
