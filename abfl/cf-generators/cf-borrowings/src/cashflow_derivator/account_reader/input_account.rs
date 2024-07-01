use crate::statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub cust_no: String,
    pub reference: String,
    pub cust_name: String,
    pub branch_cd: String,
    pub norm_int_rt: Option<f64>,
    pub acurl_freq: String,
    pub book_dt: Option<rbdate::NaiveDate>,
    pub val_dt: Option<rbdate::NaiveDate>,
    pub mat_dt: Option<rbdate::NaiveDate>,
    pub due_dt: Option<rbdate::NaiveDate>,
    pub user_def_stats: String,
    pub prod_cd: String,
    pub gl: i32,
    pub curr: String,
    pub prin_ost_bal: f64,
    pub component: String,
    pub amt_due: f64,
    pub amt_setld: f64,
    pub cf_amt: Option<f64>,
    pub spread: f64,
    pub bucket_category: String,
    pub is_secured: String,
    pub product_type: String,
    pub comp_perc: f64,
    pub old_rt_typ: String,
    pub old_benchmark: String,
    pub nxt_call_dt: Option<rbdate::NaiveDate>,
    pub nxt_put_dt: Option<rbdate::NaiveDate>,
    pub rt_flag_new: String,
    pub rt_cd_new: String,
    pub ucid: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub coupon_freq: String,
    pub nxt_repricing_dt: Option<rbdate::NaiveDate>,
    pub lst_repricing_dt: Option<rbdate::NaiveDate>,
    pub as_on_dt: rbdate::NaiveDate,
    pub int_basis: String,
    pub int_calc_typ: String,
    pub cust_typ: String,
    pub npa_typ: String,
    pub bmid: String,
    pub division: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &rbdate::DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            cust_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer number`.");
                }
            },
            reference: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `reference`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer name`.");
                }
            },
            branch_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch code`.");
                }
            },
            norm_int_rt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `norm_int_rate`.");
                }
            },
            acurl_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accural frequency`.");
                }
            },
            book_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `booking date`.");
                }
            },
            val_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `value dt`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturuty date`.");
                }
            },
            due_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `due date`.");
                }
            },
            user_def_stats: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `user defined status`.");
                }
            },
            prod_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product code`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            curr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            prin_ost_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `principal outstanding balance`.");
                }
            },
            component: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `component`.");
                }
            },
            amt_due: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amt due`.");
                }
            },
            amt_setld: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amt settled`.");
                }
            },
            cf_amt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `CFAmt`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),

                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            bucket_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bucket_category`.");
                }
            },
            is_secured: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_secured`.");
                }
            },
            product_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_type`.");
                }
            },
            comp_perc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `comp_perc.");
                }
            },
            old_rt_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `old rate typ`.");
                }
            },
            old_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `old benchmark`.");
                }
            },
            nxt_call_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next reset date`.");
                }
            },
            nxt_put_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last reset date`.");
                }
            },
            rt_flag_new: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate flag new`.");
                }
            },
            rt_cd_new: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate code new`.");
                }
            },
            ucid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ucid`.");
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
                    return Err("Could not parse property `ia llg`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm llg`.");
                }
            },
            coupon_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing frequency`.");
                }
            },
            nxt_repricing_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next repricing date`.");
                }
            },
            lst_repricing_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last repricing date`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not parse property `as on date`.");
                }
            },
            int_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest basis`.");
                }
            },
            int_calc_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest calc type`.");
                }
            },
            cust_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust type`.");
                }
            },
            npa_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa type`.");
                }
            },
            bmid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bmid`.");
                }
            },
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    //For Term-Loans Input-Data
                    "NA".to_string()
                }
            },
        };
        Ok(input_account)
    }
}
