#[derive(Debug, Clone)]
pub struct AccFieldNames {
    pub account_id: String,
    pub book_value: String,
    pub cf_amount: String,
    pub currency: String,
    pub listing_status: String,
    pub listed_exchange: String,
    pub equity_id: String,
    pub equity_name: String,
    pub equity_issuer_type: String,
    pub issuer_country: String,
    pub customer_id: String,
    pub customer_name: String,
    pub customer_type: String,
    pub isin: String,
    pub ifrs9cat: String,
    pub start_date: String,
    pub branch: String,
    pub rm: String,
    pub department: String,
    pub gl: String,
    pub product_code: String,
    pub inv_type: String,
    pub aorl: String,
    pub rl1: String,
    pub rl2: String,
    pub rl3: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn get_input_fields_names() -> AccFieldNames {
        AccFieldNames {
            account_id: "account_id".to_string(),
            book_value: "book_value".to_string(),
            cf_amount: "cf_amount".to_string(),
            currency: "currency".to_string(),
            listing_status: "listing_status".to_string(),
            listed_exchange: "listed_exchange".to_string(),
            equity_id: "equity_id".to_string(),
            equity_name: "equity_name".to_string(),
            equity_issuer_type: "equity_issuer_type".to_string(),
            issuer_country: "issuer_country".to_string(),
            customer_id: "customer_id".to_string(),
            customer_name: "customer_name".to_string(),
            customer_type: "customer_type".to_string(),
            isin: "isin".to_string(),
            ifrs9cat: "ifrs9cat".to_string(),
            start_date: "start_date".to_string(),
            branch: "branch".to_string(),
            rm: "rm".to_string(),
            department: "department".to_string(),
            gl: "gl".to_string(),
            product_code: "product_code".to_string(),
            inv_type: "inv_type".to_string(),
            aorl: "aorl".to_string(),
            rl1: "rl1".to_string(),
            rl2: "rl2".to_string(),
            rl3: "rl3".to_string(),
            cashflows: "cashflows".to_string(),
        }
    }
}