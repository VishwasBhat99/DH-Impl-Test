use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub key_1: String,
    pub branch_no: String,
    pub curr_status: String,
    pub acc_type: String,
    pub int_cat: String,
    pub inv_type: String,
    pub currency: String,
    pub customer_no: String,
    pub cr_limit: f64,
    pub curr_bal: f64,
    pub wdl_flag: String,
    pub int_available: f64,
    pub int_proj: f64,
    pub acct_open_dt: Option<NaiveDate>,
    pub int_frm_dt: Option<NaiveDate>,
    pub int_to_dt: Option<NaiveDate>,
    pub no_dues: f64,
    pub var_int_rate: f64,
    pub rval_ind: f64,
    pub mat_dt: Option<NaiveDate>,
    pub mat_amt: f64,
    pub lst_rollovr_dt: Option<NaiveDate>,
    pub lst_ovr_limit_dt: Option<NaiveDate>,
    pub cr_store_rate: f64,
    pub dr_store_rate: f64,
    pub gl_class_code: String,
    pub mop_type: String,
    pub instl_due_day: i64,
    pub term_int_comp_freq: i64,
    pub term_int_comp_sop_dt: Option<NaiveDate>,
    pub term_int_comp_eop_dt: Option<NaiveDate>,
    pub term_int_comp_amt: f64,
    pub lending_status: String,
    pub int_repay_freq: i64,
    pub name: String,
    pub cust_acct_no: String,
    pub prim_acct: String,
    pub segment_code: String,
    pub industry_code: String,
    pub group_code: String,
    pub bus_sector_code: String,
    pub tier_cust_type: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub glcode: String,
    pub int_rate: f64,
    pub curr_bal_lcy: f64,
    pub as_on_date: Option<NaiveDate>,
}

impl InputAccount {
    pub fn new_from_line<'a>(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &str> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            key_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `key_1`.");
                }
            },
            branch_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `branch_no`.");
                }
            },
            curr_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `curr_status`.");
                }
            },
            acc_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `acc_type`.");
                }
            },
            int_cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `int_cat`.");
                }
            },
            inv_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `inv_type`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `currency`.");
                }
            },
            customer_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `customer_no`.");
                }
            },
            cr_limit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `cr_limit`.");
                }
            },
            curr_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `curr_bal`.");
                }
            },
            wdl_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `wdl_flag`.");
                }
            },
            int_available: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_available`.");
                }
            },
            int_proj: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_proj`.");
                }
            },
            acct_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `acct_open_dt`.");
                }
            },
            int_frm_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `int_frm_dt`.");
                }
            },
            int_to_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `int_to_dt`.");
                }
            },
            no_dues: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `no_dues`.");
                }
            },
            var_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `var_int_rate`.");
                }
            },
            rval_ind: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `rval_ind`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `mat_dt`.");
                }
            },
            mat_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `mat_amt`.");
                }
            },
            lst_rollovr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `lst_rollovr_dt`.");
                }
            },
            lst_ovr_limit_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `lst_ovr_limit_dt`.");
                }
            },
            cr_store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `cr_store_rate`.");
                }
            },
            dr_store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dr_store_rate`.");
                }
            },
            gl_class_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `gl_class_code`.");
                }
            },
            mop_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `mop_type`.");
                }
            },
            instl_due_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `instl_due_day`.");
                }
            },
            term_int_comp_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `term_int_comp_freq`.");
                }
            },
            term_int_comp_sop_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `term_int_comp_sop_dt`.");
                }
            },
            term_int_comp_eop_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `term_int_comp_eop_dt`.");
                }
            },
            term_int_comp_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `term_int_comp_amt`.");
                }
            },
            lending_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lending_status`.");
                }
            },
            int_repay_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `int_repay_freq`.");
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
            group_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_code`.");
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
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a5`.");
                }
            },
            a6: match value_iterator.next() {
                Some(val) => val.to_string(),
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
            glcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `glcode`.");
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
