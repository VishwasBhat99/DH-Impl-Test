#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub account_id: String,
    pub currency: String,
    pub outstanding_bal: String,
    pub funded: String,
    pub non_funded: String,
    pub limit_structure: String,
    pub customer_loc: String,
    pub ctp: String,
    pub expiry_date: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            account_id: "account_id".to_string(),
            currency: "currency".to_string(),
            outstanding_bal: "outstanding_bal".to_string(),
            funded: "funded".to_string(),
            non_funded: "non_funded".to_string(),
            limit_structure: "limit_structure".to_string(),
            customer_loc: "customer_loc".to_string(),
            ctp: "ctp".to_string(),
            expiry_date: "expiry_date".to_string(),
        }
    }
}
