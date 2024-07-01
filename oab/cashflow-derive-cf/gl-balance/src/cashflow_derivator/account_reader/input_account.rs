use rbdate::DateParser;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub gl_item: String,
    pub branch: i64,
    pub basic: i64,
    pub suffix: i64,
    pub currency: String,
    pub cf_amount: f64,
    pub balance_in_omr: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        _dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            gl_item: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `gl_item`.");
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `branch`.");
                }
            },
            basic: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `basic`.");
                }
            },
            suffix: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `suffix`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            cf_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_amount`.");
                }
            },
            balance_in_omr: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `balance_in_omr`.");
                }
            },
        };
        Ok(input_account)
    }
}
