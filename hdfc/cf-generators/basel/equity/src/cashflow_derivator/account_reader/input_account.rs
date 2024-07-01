use rbdate::DateParser;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_no: String,
    pub isin: String,
    pub book_value: f64,
    pub market_value: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        _dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_no: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `deal_no`.");
                }
            },
            isin: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `isin`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            market_value: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_value`.");
                }
            },
        };
        Ok(input_account)
    }
}
