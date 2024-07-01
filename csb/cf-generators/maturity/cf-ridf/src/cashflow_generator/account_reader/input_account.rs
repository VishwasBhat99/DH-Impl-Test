use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub sl_no: i64,
    pub deposit_number: String,
    pub financial_year: String,
    pub demand_no: String,
    pub deposit_date: Option<NaiveDate>,
    pub administering_inst: String,
    pub gl_code: String,
    pub nature_of_dep: String,
    pub dep_type: String,
    pub int_rate: f64,
    pub tenor: f64,
    pub tenor_unit: String,
    pub investment_amt: f64,
    pub remarks: String,
    pub mat_date: Option<NaiveDate>,
    pub closure_date: Option<NaiveDate>,
    pub mat_amt: f64,
    pub currency: String,
    pub net_val: f64,
    pub gl_desc: String,
    pub w4b_cd: String,
    pub w4b_desc: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            sl_no: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not read property `acc_no`.");
                }
            },
            deposit_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `br_cd`.");
                }
            },
            financial_year: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_id`.");
                }
            },
            demand_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `ucic_id`.");
                }
            },
            deposit_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `ccy`.");
                }
            },
            administering_inst: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `prod_cd`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `gl_cd`.");
                }
            },
            nature_of_dep: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `gl_comp_portion`.");
                }
            },
            dep_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `acc_open_dt`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `effc_dt`.");
                }
            },
            tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `bal_os`.");
                }
            },
            tenor_unit: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `bal_os_cly`.");
                }
            },
            investment_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_comp_type`.");
                }
            },
            remarks: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `compo_int_amt`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `int_rt`.");
                }
            },
            closure_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not read property `mat_dt`.");
                }
            },
            mat_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `dep_amt`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `dep_amt_lcy`.");
                }
            },
            net_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not read property `int_amt`.");
                }
            },
            gl_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `int_acc_amt`.");
                }
            },
            w4b_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `non_with_flag`.");
                }
            },
            w4b_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `notice_day`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cust_const_code`.");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `cntrct_num`.");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `pta_5`.");
                }
            },
        };
        Ok(input_account)
    }
}
