use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub account_number: String,
    pub amount: f64,
    pub currency_code: String,
    pub int_rate: f64,
    pub maturity_date: Option<NaiveDate>,
    pub next_payment_date: Option<NaiveDate>,
    pub int_pay_freq: i64,
    pub int_calc_basis: String,
    pub next_repr_date: Option<NaiveDate>,
}

impl InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, String> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `account_number`.".to_string());
                }
            },
            amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `amount`.".to_string());
                }
            },
            currency_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `currency_code`.".to_string());
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_rate`.".to_string());
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `maturity date`.".to_string());
                }
            },
            next_payment_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `next_payment_date`.".to_string());
                }
            },
            int_pay_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `int_pay_freq`.".to_string());
                }
            },
            int_calc_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `int_calc_basis`.".to_string());
                }
            },
            next_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `next_reprice_date`.".to_string());
                }
            },
        };

        Ok(input_account)
    }
}
