#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub account_id: String,
    pub reference: String,
    pub start_date: String,
    pub maturity_date: String,
    pub outstanding_bal: String,
    pub currency: String,
    pub lcy_amount: String,
    pub customer_type: String,
    pub total_interest_amount: String,
    pub total_principal_amount: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            account_id: "account_id".to_string(),
            reference: "reference".to_string(),
            start_date: "start_date".to_string(),
            maturity_date: "maturity_date".to_string(),
            outstanding_bal: "outstanding_bal".to_string(),
            currency: "currency".to_string(),
            lcy_amount: "lcy_amount".to_string(),
            customer_type: "customer_type".to_string(),
            total_interest_amount: "total_interest_amount".to_string(),
            total_principal_amount: "total_principal_amount".to_string(),
            cashflows: "cashflows".to_string(),
        }
    }
}
