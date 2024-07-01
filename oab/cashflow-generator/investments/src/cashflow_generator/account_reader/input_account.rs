use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_id: String,
    pub sec_id: String,
    pub face_amt: f64,
    pub outstanding_bal: f64,
    pub currency: String,
    pub cf_type: String,
    pub cf_amount: f64,
    pub cf_date: Option<NaiveDate>,
    pub prod_type: String,
    pub coup_rate: f64,
    pub maturity_date: Option<NaiveDate>,
    pub next_repr_date: Option<NaiveDate>,
    pub repricing_frequency: String,
    pub last_repr_date: Option<NaiveDate>,
    pub benchmark: String,
    pub spread: f64,
    pub ctype: String,
    pub acct_ng_type: String,
    pub guarantor: String,
    pub ccode: String,
    pub start_date: Option<NaiveDate>,
    pub call_date: Option<NaiveDate>,
    pub inv_type: String,
    pub int_rate: f64,
    pub rate_flag: String,
    pub product_code: String,
    pub customer_id: String,
    pub branch: String,
    pub rm: String,
    pub department: String,
    pub gl: String,
    pub customer_name: String,
    pub monthly_avg_bal: f64,
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
            sec_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `sec_id`.");
                }
            },
            face_amt: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `face_amt`.");
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
            cf_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `cf_type`.");
                }
            },
            cf_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_amount`.");
                }
            },
            cf_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_date`.");
                }
            },
            prod_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `prod_type`.");
                }
            },
            coup_rate: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `coup_rate`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            next_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repr_date`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            last_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repr_date`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            ctype: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `ctype`.");
                }
            },
            acct_ng_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `acct_ng_type`.");
                }
            },
            guarantor: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `guarantor`.");
                }
            },
            ccode: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `ccode`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            call_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `call_date`.");
                }
            },
            inv_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `inv_type`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
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
            customer_name: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            monthly_avg_bal: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `monthly_avg_bal`.");
                }
            },
        };
        Ok(input_account)
    }
}
