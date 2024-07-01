use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_number: String,
    pub deal_type: String,
    pub collateral_id: String,
    pub collateral_amount: Option<f64>,
    pub collateral_market_value: String,
    pub currency: String,
    pub location: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_number`.");
                }
            },
            deal_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_type`.");
                }
            },
            collateral_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `collateral_id`.");
                }
            },
            collateral_amount: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `collateral_amount`.");
                }
            },
            collateral_market_value: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `collateral_market_value`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            location: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `location`.");
                }
            },
        };
        Ok(input_account)
    }
}
