use rbdate::{DateParser, NaiveDate};
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_id: String,
    pub book_value: f64,
    pub cf_amount: f64,
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
    pub start_date: Option<NaiveDate>,
    pub branch: String,
    pub rm: String,
    pub department: String,
    pub gl: String,
    pub product_code: String,
    pub inv_type: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            account_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            cf_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_amount`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            listing_status: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `listing_status`.");
                }
            },
            listed_exchange: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `listed_exchange`.");
                }
            },
            equity_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `equity_id`.");
                }
            },
            equity_name: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `equity_name`.");
                }
            },
            equity_issuer_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `equity_issuer_type`.");
                }
            },
            issuer_country: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `issuer_country`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_type`.");
                }
            },
            isin: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `isin`.");
                }
            },
            ifrs9cat: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `ifrs9cat`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `branch`.");
                }
            },
            rm: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `rm`.");
                }
            },
            department: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `department`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            inv_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `inv_type`.");
                }
            },
        };
        Ok(input_account)
    }
}
