use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
#[derive(Debug, Clone)]
pub struct InputAccount {
    pub customer_id: String,
    pub cod_acct_no: String,
    pub prod_code: String,
    pub ccy_code: String,
    pub customer_type: String,
    pub gl_account_principal: f64,
    pub gl_account_interest: f64,
    pub gl_account_accrued: f64,
    pub acct_open_date: Option<NaiveDate>,
    pub first_disb_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub eop_balance: f64,
    pub index_rate: f64,
    pub net_rate: f64,
    pub benchmark_name: String,
    pub rate_type_1: String,
    pub npa_status: String,
    pub npa_final_status: String,
    pub ptc_flag: String,
    pub foreclosure: f64,
    pub foreclosure_rate_1: f64,
    pub foreclosure_rate_2: f64,
    pub index_code: String,
    pub rate_type_2: String,
    pub next_reset_date: Option<NaiveDate>,
    pub reset_frequency: String,
    pub derived_reset_date: Option<NaiveDate>,
    pub derived_arrear_date: Option<NaiveDate>,
    pub arrear_prin: f64,
    pub add_string_1: String,
    pub add_string_2: String,
    pub add_int_1: i64,
    pub add_int_2: i64,
    pub add_float_1: f64,
    pub add_float_2: f64,
    pub add_date_1: Option<NaiveDate>,
    pub add_date_2: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String, dmy: &DateParser) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `customer_id`."),
            },
            cod_acct_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `cod_acct_no`."),
            },
            prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `prod_code`."),
            },
            ccy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `ccy_code`."),
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `customer_type`."),
            },
            gl_account_principal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `gl_account_principal`."),
            },
            gl_account_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `gl_account_interest`."),
            },
            gl_account_accrued: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `gl_account_accrued`."),
            },
            acct_open_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `acct_open_date`.");
                }
            },
            first_disb_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `first_disb_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            eop_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `eop_balance`."),
            },
            index_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `index_rate`."),
            },
            net_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `net_rate`."),
            },
            benchmark_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `benchmark_name`."),
            },
            rate_type_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `rate_type_1`."),
            },
            npa_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `npa_status`."),
            },
            npa_final_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `npa_final_status`."),
            },

            ptc_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `ptc_flag`."),
            },
            foreclosure: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `foreclosure`."),
            },
            foreclosure_rate_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `foreclosure_rate_1`."),
            },
            foreclosure_rate_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `foreclosure_rate_2`."),
            },
            index_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `index_code`."),
            },
            rate_type_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `rate_type_2`."),
            },
            next_reset_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reset_date`.");
                }
            },
            reset_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `reset_frequency`."),
            },
            derived_reset_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `derived_reset_date`.");
                }
            },
            derived_arrear_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `derived_arrear_date`.");
                }
            },
            arrear_prin: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `arrear_prin`."),
            },
            add_string_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `add_string_1`."),
            },
            add_string_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `add_string_2`."),
            },
            add_int_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => return Err("Could not parse property `add_int_1`."),
            },
            add_int_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => return Err("Could not parse property `add_int_2`."),
            },
            add_float_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `add_float_1`."),
            },
            add_float_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `add_float_2`."),
            },
            add_date_1: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `add_date_1`.");
                }
            },
            add_date_2: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `add_date_2`.");
                }
            },
        };
        Ok(input_account)
    }
}
