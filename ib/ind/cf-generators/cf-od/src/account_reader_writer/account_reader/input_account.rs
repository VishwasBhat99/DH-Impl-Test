use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub key_1: String,
    pub branch_no: String,
    pub curr_status: String,
    pub acct_type: String,
    pub int_cat: String,
    pub inv_type: String,
    pub currency: String,
    pub customer_no: String,
    pub cr_limit: f64,
    pub curr_bal: f64,
    pub int_available: f64,
    pub acct_open_dt: Option<NaiveDate>,
    pub int_from_dt: Option<NaiveDate>,
    pub int_to_dt: Option<NaiveDate>,
    pub no_dues: f64,
    pub var_int_rate: f64,
    pub rval_ind: String,
    pub lst_ovr_limit_date: Option<NaiveDate>,
    pub cr_store_rate: f64,
    pub dr_store_rate: f64,
    pub gl_class_code: String,
    pub mop_type: String,
    pub instl_due_day: f64,
    pub lending_status: String,
    pub npa_clsfn: String,
    pub name: String,
    pub cust_acc_no: String,
    pub prim_accnt: String,
    pub segment_code: String,
    pub industry_code: String,
    pub grup_code: String,
    pub bus_sector_code: String,
    pub tier_cust_type: String,
    pub a1: String,
    pub a2: String,
    pub a3: f64,
    pub a4: Option<NaiveDate>,
    pub a5: Option<NaiveDate>,
    pub a6: Option<NaiveDate>,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub gl_code: String,
    pub int_rate: f64,
    pub curr_bal_lcy: f64,
    pub as_on_date: Option<NaiveDate>,
    pub account_status: String,
    pub a12: String,
    pub a13: String,
    pub a14: f64,
    pub a15: f64,
    pub a16: f64,
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
            branch_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_no`.");
                }
            },
            curr_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `curr_status`.");
                }
            },
            acct_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_type`.");
                }
            },
            int_cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_cat`.");
                }
            },
            inv_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inv_type`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            customer_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_no`.");
                }
            },
            cr_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cr_limit`.");
                }
            },
            curr_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `curr_bal`.");
                }
            },
            int_available: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_available`.");
                }
            },
            acct_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acct_open_dt`.");
                }
            },
            int_from_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `int_from_dt`.");
                }
            },
            int_to_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` int_to_dt`.");
                }
            },
            no_dues: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `no_dues`.");
                }
            },
            var_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `var_int_rate`.");
                }
            },
            rval_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rval_ind`.");
                }
            },
            lst_ovr_limit_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_ovr_limit_date`.");
                }
            },
            cr_store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cr_store_rate`.");
                }
            },
            dr_store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dr_store_rate`.");
                }
            },
            gl_class_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_class_code`.");
                }
            },
            mop_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mop_type`.");
                }
            },
            instl_due_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `instl_due_day`.");
                }
            },
            lending_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lending_status`.");
                }
            },
            npa_clsfn: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_clsfn`.");
                }
            },
            name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `name`.");
                }
            },
            cust_acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_acc_no`.");
                }
            },
            prim_accnt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prim_accnt`.");
                }
            },
            segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `segment_code`.");
                }
            },
            industry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry_code`.");
                }
            },
            grup_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `grup_code`.");
                }
            },
            bus_sector_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bus_sector_code`.");
                }
            },
            tier_cust_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tier_cust_type`.");
                }
            },
            a1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a1`.");
                }
            },
            a2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a2`.");
                }
            },
            a3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `a3`.");
                }
            },
            a4: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `a4`.");
                }
            },
            a5: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `a5`.");
                }
            },
            a6: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `a6`.");
                }
            },
            a7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a7`.");
                }
            },
            a8: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a8`.");
                }
            },
            a9: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a9`.");
                }
            },
            a10: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a10`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            curr_bal_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `curr_bal_lcy`.");
                }
            },
            as_on_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            account_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_status`.");
                }
            },
            a12: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a12`.");
                }
            },
            a13: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a13`.");
                }
            },
            a14: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `a14`.");
                }
            },
            a15: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `a15`.");
                }
            },
            a16: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `a16`.");
                }
            },
        };
        Ok(input_account)
    }
}
