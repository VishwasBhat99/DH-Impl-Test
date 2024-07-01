use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_id: String,
    pub sec_id: String,
    pub product: String,
    pub product_type: String,
    pub face_amt: f64,
    pub outstanding_bal: f64,
    pub currency: String,
    pub cmne: String,
    pub sn: String,
    pub c_type: String,
    pub start_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub repo_rate: f64,
    pub int_calc_type: String,
    pub spread: f64,
    pub benchmark: String,
    pub last_repr_date: Option<NaiveDate>,
    pub next_repr_date: Option<NaiveDate>,
    pub int_rate: f64,
    pub rate_flag: String,
    pub product_code: String,
    pub customer_id: String,
    pub branch: String,
    pub rm: String,
    pub department: String,
    pub gl: String,
    pub customer_name: String,
    pub monthly_avg_bal: String,
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
            product: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `product`.");
                }
            },
            product_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `product_type`.");
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
            cmne: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `cmne`.");
                }
            },
            sn: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `sn`.");
                }
            },
            c_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `c_type`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            repo_rate: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `repo_rate`.");
                }
            },
            int_calc_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `int_calc_type`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            last_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repr_date`.");
                }
            },
            next_repr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repr_date`.");
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
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `monthly_avg_bal`.");
                }
            },
        };
        Ok(input_account)
    }
}
