pub struct AccFieldNames {
    pub account_number: String,
    pub concat: String,
    pub cashflows: String,
    pub institution: String,
    pub interest_rate: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        // TODO: Read the file to get this structure. Currently, it is hard-coded for deposits.
        AccFieldNames {
            account_number: "account_number".to_string(),
            concat: "concat".to_string(),
            cashflows: "cashflows".to_string(),
            institution: "institution".to_string(),
            interest_rate: "int_rate".to_string(),
        }
    }
}
