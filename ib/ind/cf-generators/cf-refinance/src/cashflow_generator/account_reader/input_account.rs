use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug)]
pub struct InputAccount {
    pub description: String,
    pub due_date: NaiveDate,
    pub current_balance_amount: f64,
    pub ccy: String,
    pub int_rate: f64,
}

impl InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, String> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            description: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `description`.".to_string());
                }
            },
            due_date: match value_iterator.next() {
                Some(val) => {
                    let mat_dt = dmy_date_parser.parse_opt(val);
                    if mat_dt.is_none() {
                        return Err("Could not parse property `due_date`.".to_string());
                    }
                    mat_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `due_date`.".to_string());
                }
            },
            current_balance_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `current_balance_amount`.".to_string());
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ccy`.".to_string());
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_rate`.".to_string());
                }
            },
        };

        Ok(input_account)
    }
}
