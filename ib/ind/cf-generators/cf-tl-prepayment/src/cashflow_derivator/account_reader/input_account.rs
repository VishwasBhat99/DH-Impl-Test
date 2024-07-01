use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
#[derive(Debug, Clone)]
pub struct InputAccount {
    pub customer_no: String,
    pub key_1: String,
    pub gl_class_code: String,
    pub acct_type: String,
    pub int_cat: f64,
    pub apprv_date: Option<NaiveDate>,
    pub app_amt: f64,
    pub premat_amt: f64,
    pub first_emi_date: Option<NaiveDate>,
    pub ccy: String,
    pub lst_fin_date: Option<NaiveDate>,
    pub eff_int_rt: f64,
    pub matdt: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String, dmy: &DateParser) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            customer_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_no`.");
                }
            },
            key_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `key_1`.");
                }
            },
            gl_class_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_class_code`.");
                }
            },
            acct_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_type`.");
                }
            },
            int_cat: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `int_cat`.");
                }
            },
            apprv_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `apprv_date`.");
                }
            },
            app_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `app_amt`.");
                }
            },
            premat_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `premat_amt`.");
                }
            },
            first_emi_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `first_emi_date`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            lst_fin_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_fin_date`.");
                }
            },

            eff_int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `eff_int_rt`.");
                }
            },
            matdt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `matdt`.");
                }
            },
        };
        Ok(input_account)
    }
}
