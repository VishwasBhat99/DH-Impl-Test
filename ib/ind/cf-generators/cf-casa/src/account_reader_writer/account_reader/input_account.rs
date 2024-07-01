use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

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
    pub cr_limit: i32,
    pub curr_bal: f64,
    pub wdl_flag: i32,
    pub int_available: i32,
    pub acc_open_dt: Option<NaiveDate>,
    pub int_frm_dt: Option<NaiveDate>,
    pub int_to_dt: Option<NaiveDate>,
    pub no_dues: i32,
    pub var_int_rate: i32,
    pub rval_ind: String,
    pub od_visa_area: String,
    pub lst_ovr_limit_date: Option<NaiveDate>,
    pub cr_store_rate: i32,
    pub dr_store_rate: i32,
    pub gl_class_code: String,
    pub mop_type: String,
    pub instl_due_day: i32,
    pub term_int_comp_freq: String,
    pub term_int_cmp_sop_dt: Option<NaiveDate>,
    pub term_int_cmp_eop_dt: Option<NaiveDate>,
    pub term_int_comp_amt: f64,
    pub lending_status: String,
    pub name: String,
    pub cust_acct_no: String,
    pub prim_acct: String,
    pub segment_code: String,
    pub industry_code: String,
    pub grup_code: String,
    pub bus_sector_code: String,
    pub tier_cust_type: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: i64,
    pub a6: i64,
    pub a7: f64,
    pub a8: f64,
    pub a9: Option<NaiveDate>,
    pub a10: Option<NaiveDate>,
    pub gl_code: String,
    pub int_rate: f64,
    pub curr_bal_lcy: f64,
    pub as_on_date: Option<NaiveDate>,
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
                Some(val) => val.parse().unwrap_or(0),
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
            wdl_flag: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `wdl_flag`.");
                }
            },
            int_available: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `int_available`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_dt`.");
                }
            },
            int_frm_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `int_frm_dt`.");
                }
            },
            int_to_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `int_to_dt`.");
                }
            },
            no_dues: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `no_dues`.");
                }
            },
            var_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
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
            od_visa_area: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `od_visa_area`.");
                }
            },
            lst_ovr_limit_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_ovr_limit_date`.");
                }
            },
            cr_store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `cr_store_rate`.");
                }
            },
            dr_store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
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
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `instl_due_day`.");
                }
            },
            term_int_comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `term_int_comp_freq`.");
                }
            },
            term_int_cmp_sop_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `term_int_cmp_sop_dt`.");
                }
            },
            term_int_cmp_eop_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `term_int_cmp_eop_dt`.");
                }
            },
            term_int_comp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `term_int_comp_amt`.");
                }
            },
            lending_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lending_status`.");
                }
            },
            name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `name`.");
                }
            },
            cust_acct_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_acct_no`.");
                }
            },
            prim_acct: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prim_acct`.");
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
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a3`.");
                }
            },
            a4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a4`.");
                }
            },
            a5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `a5`.");
                }
            },
            a6: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `a6`.");
                }
            },
            a7: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `a7`.");
                }
            },
            a8: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `a8`.");
                }
            },
            a9: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `a9`.");
                }
            },
            a10: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
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
        };
        Ok(input_account)
    }
}
