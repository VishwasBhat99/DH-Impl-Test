use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    // Standard Fields
    pub account_id: String,
    pub currency: String,
    pub int_rate: f64,
    pub outstanding_bal: f64,
    pub gl: i64,
    pub start_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub customer_id: i64,
    pub customer_type: String,
    pub lcy_amount: f64,
    pub reference: String,
    pub npa_flag: String,
    pub npa_type: String,
    pub interest_type: String,
    pub int_repayment_frequency: i64,
    pub last_repr_date: Option<NaiveDate>,
    pub next_repr_date: Option<NaiveDate>,
    pub cust_constitution_code: i64,
    pub rate_flag: String,
    pub customer_name: String,
    pub product_code: String,
    pub account_type: String,
    pub branch: String,
    pub rm: String,
    pub group_code: String,
    pub monthly_avg_bal: String,
    pub customer_rating: String,
    pub p2: String,
    pub waiver_flag: String,
    pub accrued_int_amt: String,
    pub string1: String,
    pub string2: String,
    pub string3: String,
    pub number1: i64,
    pub number2: i64,
    pub number3: i64 
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
            currency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            outstanding_bal: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `outstanding_bal`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl`.");
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
            customer_id: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_type`.");
                }
            },
            lcy_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcy_amount`.");
                }
            },
            reference: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `reference`.");
                }
            },
            npa_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `npa_flag`.");
                }
            },
            npa_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `npa_type`.");
                }
            },
            interest_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `interest_type`.");
                }
            },
            int_repayment_frequency: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `int_repayment_frequency`.");
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
            cust_constitution_code: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cust_constitution_code`.");
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            account_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `account_type`.");
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
            group_code: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `group_code`.");
                }
            },
            monthly_avg_bal: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `monthly_avg_bal`.");
                }
            },
            customer_rating: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `customer_rating`.");
                }
            },
            p2: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `p2`.");
                }
            },
            waiver_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `waiver_flag`.");
                }
            },
            accrued_int_amt: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `accrued_int_amt`.");
                }
            },
            string1: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `string1`.");
                }
            },
            string2: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `string2`.");
                }
            },
            string3: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `string3`.");
                }
            },
            number1: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `number1`.");
                }
            },
            number2: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `number2`.");
                }
            },
            number3: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `number3`.");
                }
            },
        };
        Ok(input_account)
    }
}
