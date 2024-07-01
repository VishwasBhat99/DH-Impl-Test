use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_id: String,
    pub outstanding_bal: f64,
    pub currency: String,
    pub schd_type: String,
    pub cf_principal_amount: f64,
    pub cf_interest_amount: f64,
    pub cf_date: Option<NaiveDate>,
    pub start_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub int_rate: f64,
    pub int_rate_classification: String,
    pub benchmark: String,
    pub repricing_frequency: String,
    pub last_repr_date: String,
    pub next_repr_date: String,
    pub coupon_payment_start_date: String,
    pub coupon_payment_frequency: String,
    pub cust_constitution_code: i64,
    pub instrument: String,
    pub counter_party_id: String,
    pub counter_party_name: String,
    pub counter_party_type: String,
    pub customer_id: i64,
    pub customer_name: String,
    pub product_code: String,
    pub account_type: String,
    pub gl: String,
    pub rate_flag: String,
    pub branch: String,
    pub rm: String,
    pub group_code: String,
    pub monthly_avg_bal: f64,
    pub tenor: i64,
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
            schd_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `schd_type`.");
                }
            },
            cf_principal_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_principal_amount`.");
                }
            },
            cf_interest_amount: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cf_interest_amount`.");
                }
            },
            cf_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val.trim()),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => {
                    let st_dt = dmy_date_parser.parse_opt(val.trim());
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
                    let st_dt = dmy_date_parser.parse_opt(val.trim());
                    if st_dt.is_none() {
                        return Err("Could not parse property `maturity_date`.");
                    }
                    st_dt.unwrap()
                }
                None => {
                    return Err("Could not read property `maturity_date`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_rate_classification: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `int_rate_classification`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            last_repr_date: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `last_repr_date`.");
                }
            },
            next_repr_date: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `next_repr_date`.");
                }
            },
            coupon_payment_start_date: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `coupon_payment_start_date`.");
                }
            },
            coupon_payment_frequency: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `coupon_payment_frequency`.");
                }
            },
            cust_constitution_code: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cust_constitution_code`.");
                }
            },
            instrument: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `instrument`.");
                }
            },
            counter_party_id: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `counter_party_id`.");
                }
            },
            counter_party_name: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `counter_party_name`.");
                }
            },
            counter_party_type: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `counter_party_type`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `customer_id`.");
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
            gl: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.trim().to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
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
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `monthly_avg_bal`.");
                }
            },
            tenor: match value_iterator.next() {
                Some(val) => val.trim().parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `tenor`.");
                }
            },
        };
        Ok(input_account)
    }
}
