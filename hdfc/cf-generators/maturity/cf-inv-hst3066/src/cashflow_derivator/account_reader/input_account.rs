use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub portfolio_name: String,
    pub account_number: String,
    pub maturity: Option<NaiveDate>,
    pub face_value: Option<i64>,
    pub book_value: Option<i64>,
    pub accr_amt: f64,
    pub inv_type: String,
    pub as_on_dt: NaiveDate,
    pub ccy: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_dt_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            portfolio_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio_name`.");
                }
            },
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            maturity: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity`.");
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            accr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accr_amt`.");
                }
            },
            inv_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inv_type`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse(val),
                None => {
                    return Err("Could not parse property `as_on_dt`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
        };
        Ok(input_account)
    }
}
