use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub acc_no: String,
    pub br_cd: i64,
    pub cust_id: i64,
    pub ucic_id: i64,
    pub ccy: String,
    pub prod_cd: i64,
    pub gl_cd: i64,
    pub gl_comp_portion: String,
    pub acc_open_dt: NaiveDate,
    pub effc_dt: Option<NaiveDate>,
    pub bal_os: f64,
    pub bal_os_cly: f64,
    pub int_comp_type: String,
    pub compo_int_amt: f64,
    pub int_rt: f64,
    pub mat_dt: NaiveDate,
    pub dep_amt: f64,
    pub dep_amt_lcy: f64,
    pub int_amt: f64,
    pub int_acc_amt: f64,
    pub non_with_flag: String,
    pub notice_day: String,
    pub cust_const_code: i64,
    pub cntrct_num: i64,
    pub as_on: Option<NaiveDate>,
    pub comp_freq: String,
    pub pay_freq: String,
    pub over_dt: Option<NaiveDate>,
    pub lst_int_acr_dt: Option<NaiveDate>,
    pub int_pay_amt: f64,
    pub is_overdue: String,
    pub max_date: NaiveDate,
    pub resid_days: i64,
    pub over_int_rt: f64,
}

impl InputAccount {
    pub fn new_from_line<'a>(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &str> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `acc_no`.");
                }
            },
            br_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `br_cd`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cust_id`.");
                }
            },
            ucic_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `ucic_id`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ccy`.");
                }
            },
            prod_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `prod_cd`.");
                }
            },
            gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `gl_cd`.");
                }
            },
            gl_comp_portion: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `gl_comp_portion`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `acc_open_dt`.");
                }
            },
            effc_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `effc_dt`.");
                }
            },
            bal_os: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `bal_os`.");
                }
            },
            bal_os_cly: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `bal_os_cly`.");
                }
            },
            int_comp_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `int_comp_type`.");
                }
            },
            compo_int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `compo_int_amt`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_rt`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `mat_dt`.");
                }
            },
            dep_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dep_amt`.");
                }
            },
            dep_amt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dep_amt_lcy`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_amt`.");
                }
            },
            int_acc_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_acc_amt`.");
                }
            },
            non_with_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `non_with_flag`.");
                }
            },
            notice_day: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `notice_day`.");
                }
            },
            cust_const_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cust_const_code`.");
                }
            },
            cntrct_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `cntrct_num`.");
                }
            },
            as_on: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `as_on`.");
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `comp_freq`.");
                }
            },
            pay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pay_freq`.");
                }
            },
            over_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `as_on`.");
                }
            },
            lst_int_acr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `lst_int_acr_dt`.");
                }
            },
            int_pay_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_pay_amt`.");
                }
            },
            is_overdue: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `is_overdue`.");
                }
            },
            max_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not read property `max_date`.");
                }
            },
            resid_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `resid_days`.");
                }
            },
            over_int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `over_int_rt`.");
                }
            },
        };

        Ok(input_account)
    }
}
