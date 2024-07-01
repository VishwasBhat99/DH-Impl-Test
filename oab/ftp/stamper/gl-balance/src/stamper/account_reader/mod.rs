#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub gl_item: String,
    pub branch: String,
    pub basic: String,
    pub suffix: String,
    pub currency: String,
    pub cf_amount: String,
    pub balance_in_omr: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            gl_item: "gl_item".to_string(),
            branch: "branch".to_string(),
            basic: "basic".to_string(),
            suffix: "suffix".to_string(),
            currency: "currency".to_string(),
            cf_amount: "cf_amount".to_string(),
            balance_in_omr: "balance_in_omr".to_string(),
        }
    }
}
