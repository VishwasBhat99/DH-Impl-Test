use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub loan_acc_no: String,
    pub cust_id: i64,
    pub cust_name: String,
    pub prod_name: String,
    pub no_of_installments: i64,
    pub sanc_amt: f64,
    pub disbursed_amt: f64,
    pub prin_os: f64,
    pub od_prin: f64,
    pub od_int: f64,
    pub int_rate: f64,
    pub int_type: String,
    pub currency: String,
    pub branch_cd: String,
    pub value_dt: Option<NaiveDate>,
    pub maturity_dt: Option<NaiveDate>,
    pub gl_code: i64,
    pub date: String,
    pub principal: f64,
    pub interest: f64,
    pub principal_os: f64,
    pub spread: f64,
    pub securitization_percentage: String,
    pub last_payment_dt: Option<NaiveDate>,
    pub next_reset_date: Option<NaiveDate>,
    pub last_reset_dt: Option<NaiveDate>,
    pub division: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub tenure: i64,
    pub remaining_tenure: i64,
    pub as_on_date: Option<NaiveDate>,
    pub cust_type: String,
    pub npa_type: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            loan_acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_acc_no`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            prod_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_name`.");
                }
            },
            no_of_installments: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `no_of_installments`.");
                }
            },
            sanc_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sanc_amt`.");
                }
            },
            disbursed_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `disbursed_amt`.");
                }
            },
            prin_os: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_os`.");
                }
            },
            od_prin: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `od_prin`.");
                }
            },
            od_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `od_int`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            branch_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_cd`.");
                }
            },
            value_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `value_dt`.");
                }
            },
            maturity_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_dt`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `date`.");
                }
            },
            principal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `principal`.");
                }
            },
            interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest`.");
                }
            },
            principal_os: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `principal_OS`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            securitization_percentage: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `securitization_percentage`.");
                }
            },
            last_payment_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_payment_dt`.");
                }
            },
            next_reset_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reset_dt`.");
                }
            },
            last_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_reset_dt`.");
                }
            },
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            ia_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ia_llg`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_llg`.");
                }
            },
            tenure: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `tenure`.");
                }
            },
            remaining_tenure: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `remaining_tenure`.");
                }
            },
            as_on_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            cust_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_type`.");
                }
            },
            npa_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_type`.");
                }
            },
        };
        Ok(input_account)
    }
}
