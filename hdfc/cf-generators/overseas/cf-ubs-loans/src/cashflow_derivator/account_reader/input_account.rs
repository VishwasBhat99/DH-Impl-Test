use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub cust_no: String,
    pub reference: String,
    pub cust_name: String,
    pub branch_cd: String,
    pub norm_int_rt: Option<f64>,
    pub acurl_freq: String,
    pub book_dt: Option<NaiveDate>,
    pub val_dt: Option<NaiveDate>,
    pub mat_dt: Option<NaiveDate>,
    pub due_dt: Option<NaiveDate>,
    pub user_def_stats: String,
    pub prod_cd: String,
    pub gl: String,
    pub curr: String,
    pub prin_ost_bal: f64,
    pub component: String,
    pub amt_due: f64,
    pub amt_setld: f64,
    pub cf_amt: Option<f64>,
    pub spread: f64,
    pub compmis1: i64,
    pub compmis2: i64,
    pub compmis3: i64,
    pub old_rt_cd: String,
    pub old_rt_typ: String,
    pub old_benchmark: String,
    pub nxt_reset_dt: Option<NaiveDate>,
    pub last_reset_dt: Option<NaiveDate>,
    pub rt_flag_new: String,
    pub rt_cd_new: String,
    pub division: String,
    pub concat: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub repricing_freq: String,
    pub nxt_repricing_dt: Option<NaiveDate>,
    pub lst_repricing_dt: Option<NaiveDate>,
    pub as_on_dt: Option<NaiveDate>,
    pub int_basis: String,
    pub int_calc_typ: String,
    pub cust_typ: String,
    pub npa_typ: String,
    pub bmid: String,
    pub cntr_party: String,
    pub lcy_amount: f64,
    pub raw_benchmark: String,
    pub sma_flag: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
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
                Some(val) => val.to_string(),
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
            compmis1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `compmis1`.");
                }
            },
            compmis2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `compmis2`.");
                }
            },
            compmis3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `compmis3`.");
                }
            },
            old_rt_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `old rate code`.");
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
            nxt_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next reset date`.");
                }
            },
            last_reset_dt: match value_iterator.next() {
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
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            concat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `concat`.");
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
            repricing_freq: match value_iterator.next() {
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
                Some(val) => dmy_date_parser.parse_opt(val),
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
            cntr_party: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_party`.");
                }
            },
            lcy_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcy_amount`.");
                }
            },
            raw_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `raw_benchmark`.");
                }
            },
            sma_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sma_flag`.");
                }
            },
        };
        Ok(input_account)
    }
}
