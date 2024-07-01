use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_id: String,
    pub reference: String,
    pub start_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub outstanding_bal: f64,
    pub currency: String,
    pub lcy_amount: f64,
    pub customer_type: String,
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
            reference: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `reference`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => {
                    let st_dt = dmy_date_parser.parse_opt(val);
                    if st_dt.is_none() {
                        return Err("Could not parse property `start_date`.");
                    }
                    st_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `start_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => {
                    let st_dt = dmy_date_parser.parse_opt(val);
                    if st_dt.is_none() {
                        return Err("Could not parse property `maturity_date`.");
                    }
                    st_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `maturity_date`.");
                }
            },
            outstanding_bal: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `outstanding_bal`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            lcy_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcy_amount`.");
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_type`.");
                }
            },
        };
        Ok(input_account)
    }
}
