use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    // Standard Fields
    pub account_id: i64,
    pub currency: String,
    pub outstanding_bal: f64,
    // Passthrough
    pub funded: f64,
    pub non_funded: f64,
    pub limit_structure: String,
    pub customer_loc: String,
    pub ctp: String,
    pub expiry_date: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            // Standard Fields
            account_id: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            outstanding_bal: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `outstanding_bal`.");
                }
            },
            funded: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `funded`.");
                }
            },
            non_funded: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `non_funded`.");
                }
            },
            limit_structure: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `limit_structure`.");
                }
            },
            customer_loc: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_loc`.");
                }
            },
            ctp: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `ctp`.");
                }
            },
            expiry_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val.trim()),
                None => {
                    return Err("Could not parse property `expiry_date`.");
                }
            },
        };
        Ok(input_account)
    }
}
