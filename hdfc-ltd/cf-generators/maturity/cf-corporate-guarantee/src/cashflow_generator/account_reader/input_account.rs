use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub branch_code: String,
    pub customer_id: String,
    pub customer_name: String,
    pub currency: String,
    pub gl_code: String,
    pub currentoutstandingbal: Option<f64>,
    pub issue_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub guarantee_type: String,
    pub app1: String,
    pub app2: String,
    pub app3: String,
    pub app4: String,
    pub app5: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_code`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            currentoutstandingbal: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `currentoutstandingbal`.");
                }
            },
            issue_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `issue_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            guarantee_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `guarantee_type`.");
                }
            },
            app1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app1`.");
                }
            },
            app2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app2`.");
                }
            },
            app3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app3`.");
                }
            },
            app4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app4`.");
                }
            },
            app5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app5`.");
                }
            },
        };
        Ok(input_account)
    }
}
