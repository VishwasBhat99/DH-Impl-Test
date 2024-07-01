use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub key_1: String,
    pub gl_class_code: String,
    pub status: String,
    pub balance: f64,
    pub old_bad_debt_ind: i64,
    pub i_or_b: String,
    pub crm_flag: String,
    pub app_amt: f64,
    pub lmt1: f64,
    pub lmt2: f64,
    pub lmt3: f64,
    pub lmt4: f64,
    pub od_lmt: f64,
    pub adv_val: f64,
    pub basel_class: String,
    pub limit_exp_date: Option<NaiveDate>,
    pub lending_status: i64,
    pub dp: String,
    pub drawing_amt: f64,
    pub od_multi_lim_allow: i64,
    pub ccy: String,
    pub group: String,
    pub llg: String,
    pub limit_amt: f64,
    pub dp_amt: f64,
    pub undrawn_sls_amt: f64,
    pub undrawn_lcr_amt: f64,
    pub undrawn_nsfr_amt: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
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
            status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `status`.");
                }
            },
            balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `balance`.");
                }
            },
            old_bad_debt_ind: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `old_bad_debt_ind`.");
                }
            },
            i_or_b: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `i_or_b`.");
                }
            },
            crm_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `crm_flag`.");
                }
            },
            app_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `app_amt`.");
                }
            },
            lmt1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lmt1`.");
                }
            },
            lmt2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lmt2`.");
                }
            },
            lmt3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lmt3`.");
                }
            },
            lmt4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lmt4`.");
                }
            },
            od_lmt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `od_lmt`.");
                }
            },
            adv_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `adv_val`.");
                }
            },
            basel_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `basel_class`.");
                }
            },
            limit_exp_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `limit_exp_date`.");
                }
            },
            lending_status: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `lending_status`.");
                }
            },
            dp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dp`.");
                }
            },
            drawing_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `drawing_amt`.");
                }
            },
            od_multi_lim_allow: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `lending_status`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            group: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group`.");
                }
            },
            llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `llg`.");
                }
            },
            limit_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `limit_amt`.");
                }
            },
            dp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dp_amt`.");
                }
            },
            undrawn_sls_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `undrawn_sls_amt`.");
                }
            },
            undrawn_lcr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `undrawn_lcr_amt`.");
                }
            },
            undrawn_nsfr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `undrawn_nsfr_amt`.");
                }
            },
        };
        Ok(input_account)
    }
}
