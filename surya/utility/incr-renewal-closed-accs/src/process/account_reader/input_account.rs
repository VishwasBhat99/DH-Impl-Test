use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub account_id: String,
    pub as_on_date: NaiveDate,
    pub acc_open_date: NaiveDate,
    pub acc_crncy_code: String,
    pub out_bal: f64,
    pub out_bal_lcy: f64,
    pub maturity_date: NaiveDate,
    pub interest_rate: f64,
    pub next_reprise_date: NaiveDate,
    pub last_reprise_date: NaiveDate,
    pub gl_code: String,
    pub scheme_code: String,
    pub customer_id: String,
    pub customer_type: String,
    pub cust_const_code: String,
    pub customer_name: String,
    pub total_int_amt: f64,
    pub total_prin_amt: f64,
    pub pt_f64_1: f64,
    pub pt_f64_2: f64,
    pub pt_f64_3: f64,
    pub pt_f64_4: f64,
    pub pt_i64_1: i64,
    pub pt_i64_2: i64,
    pub pt_i64_3: i64,
    pub pt_i64_4: i64,
    pub pt_str_1: String,
    pub pt_str_2: String,
    pub pt_str_3: String,
    pub pt_str_4: String,
    pub cashflows: String,
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
            account_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            as_on_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val).unwrap_or_default(),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            acc_open_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val).unwrap_or_default(),
                None => {
                    return Err("Could not parse property `acc_open_date`.");
                }
            },
            acc_crncy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_crncy_code`.");
                }
            },
            out_bal: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `out_bal`.");
                }
            },
            out_bal_lcy: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `out_bal_lcy`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val).unwrap_or_default(),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            interest_rate: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            next_reprise_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val).unwrap_or_default(),
                None => {
                    return Err("Could not parse property `next_reprise_date`.");
                }
            },
            last_reprise_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val).unwrap_or_default(),
                None => {
                    return Err("Could not parse property `last_reprise_date`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            scheme_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `scheme_code`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_type`.");
                }
            },
            cust_const_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_const_code`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            total_int_amt: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `total_int_amt`.");
                }
            },
            total_prin_amt: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `total_prin_amt`.");
                }
            },
            pt_f64_1: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `pt_f64_1`.");
                }
            },
            pt_f64_2: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `pt_f64_2`.");
                }
            },
            pt_f64_3: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `pt_f64_3`.");
                }
            },
            pt_f64_4: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `pt_f64_4`.");
                }
            },
            pt_i64_1: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `pt_i64_1`.");
                }
            },
            pt_i64_2: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `pt_i64_2`.");
                }
            },
            pt_i64_3: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `pt_i64_3`.");
                }
            },
            pt_i64_4: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `pt_i64_4`.");
                }
            },
            pt_str_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_1`.");
                }
            },
            pt_str_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_2`.");
                }
            },
            pt_str_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_3`.");
                }
            },
            pt_str_4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pt_str_4`.");
                }
            },
            cashflows: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cashflows`.");
                }
            },
        };
        Ok(input_account)
    }
}

#[derive(Clone, Debug)]
pub struct MasterAccount {
    pub account_number: String,
    pub account_id: String,
    pub as_on_date: NaiveDate,
    pub acc_open_date: NaiveDate,
    pub acc_crncy_code: String,
    pub out_bal: f64,
    pub out_bal_lcy: f64,
    pub maturity_date: NaiveDate,
    pub interest_rate: f64,
    pub next_reprise_date: NaiveDate,
    pub last_reprise_date: NaiveDate,
    pub gl_code: String,
    pub scheme_code: String,
    pub customer_id: String,
    pub customer_type: String,
    pub cust_const_code: String,
    pub customer_name: String,
    pub tot_int_amt: f64,
    pub total_prin_amt: f64,
    pub acct_type: String,
    pub pt_f64_1: f64,
    pub pt_f64_2: f64,
    pub pt_f64_3: f64,
    pub pt_f64_4: f64,
    pub pt_i64_1: i64,
    pub pt_i64_2: i64,
    pub pt_i64_3: i64,
    pub pt_i64_4: i64,
    pub pt_str_1: String,
    pub pt_str_2: String,
    pub pt_str_3: String,
    pub pt_str_4: String,
    pub cashflows: String,
}
