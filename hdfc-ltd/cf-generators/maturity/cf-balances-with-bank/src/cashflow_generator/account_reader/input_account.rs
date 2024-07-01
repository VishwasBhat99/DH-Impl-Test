use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub bank_code: String,
    pub acc_open_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub initial_deposit_amount: Option<f64>,
    pub initial_deposit_amountlcy: f64,
    pub int_rate: Option<f64>,
    pub int_payment_fq: String,
    pub acc_no: String,
    pub gl_code: i64,
    pub slr_nonslr: String,
    pub ccy: String,
    pub product_code: String,
    pub code_gl: String,
    pub holding_period: String,
    pub interest_accrued: f64,
    pub broken_quat_int: Option<f64>,
    pub app1: String,
    pub app2: String,
    pub app3: String,
    pub app4: String,
    pub app5: String,
    pub app6: String,
    pub app7: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            bank_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_code`.");
                }
            },
            acc_open_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            initial_deposit_amount: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `initial_deposit_amount`.");
                }
            },
            initial_deposit_amountlcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `initial_deposit_amountlcy`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_payment_fq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_payment_fq`.");
                }
            },
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            slr_nonslr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `slr_nonslr`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            code_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `code_gl`.");
                }
            },
            holding_period: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `holding_period`.");
                }
            },
            interest_accrued: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_accrued`.");
                }
            },
            broken_quat_int: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `broken_quat_int`.");
                }
            },
            app1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app1`.");
                }
            },
            app2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app2`.");
                }
            },
            app3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app3`.");
                }
            },
            app4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app4`.");
                }
            },
            app5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app5`.");
                }
            },
            app6: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app6`.");
                }
            },
            app7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app7`.");
                }
            },
        };
        Ok(input_account)
    }
}
