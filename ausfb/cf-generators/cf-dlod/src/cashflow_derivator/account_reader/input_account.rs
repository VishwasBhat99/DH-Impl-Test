use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
#[derive(Debug, Clone)]
pub struct InputAccount {
    pub customer_id: String,
    pub account_id: String,
    pub prod_type: String,
    pub scheme_type: String,
    pub prod_code: String,
    pub currency: String,
    pub customer_type: String,
    pub gl_account_principal: String,
    pub open_date: Option<NaiveDate>,
    pub value_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub limit_amt: f64,
    pub curr_bal_amount: f64,
    pub flg_fixed_floating: String,
    pub interest_paid: f64,
    pub interest_received: f64,
    pub flg_performing_npa: String,
    pub asset_type: String,
    pub cod_acct_no: String,
    pub cod_limit_no: String,
    pub loan_limit_amount: String,
    pub index_code: String,
    pub index_name: String,
    pub index_rate: f64,
    pub effective_roi: f64,
    pub reset_frequency: String,
    pub next_reset_date: Option<NaiveDate>,
    pub tenure: String,
    pub classification: String,
    pub derived_reset_date: Option<NaiveDate>,
    pub final_reset_date: Option<NaiveDate>,
    pub npa_status: String,
    pub npa_final_status: String,
    pub add_string_1: String,
    pub add_string_2: String,
    pub add_float_1: f64,
    pub add_float_2: f64,
    pub add_int_1: i64,
    pub add_int_2: i64,
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
            account_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `account_id`."),
            },
            prod_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `prod_type`."),
            },
            scheme_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `scheme_type`."),
            },
            prod_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `prod_code`."),
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `currency`."),
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `customer_type`."),
            },
            gl_account_principal: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `gl_account_principal`."),
            },
            open_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `open_date`.");
                }
            },
            value_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `value_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            limit_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `limit_amt`."),
            },
            curr_bal_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `curr_bal_amount`."),
            },
            flg_fixed_floating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `flg_fixed_floating`."),
            },
            interest_paid: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `interest_paid`."),
            },
            interest_received: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `interest_received`."),
            },
            flg_performing_npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `flg_performing_npa`."),
            },
            asset_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `asset_type`."),
            },
            cod_acct_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `cod_acct_no`."),
            },
            cod_limit_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `cod_limit_no`."),
            },
            loan_limit_amount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `loan_limit_amount`."),
            },
            index_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `index_code`."),
            },
            index_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `index_name`."),
            },
            index_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `index_rate`."),
            },
            effective_roi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `effective_roi`."),
            },
            reset_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `reset_frequency`."),
            },
            next_reset_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_reset_date`.");
                }
            },
            tenure: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `tenure`."),
            },
            classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `classification`."),
            },
            derived_reset_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `derived_reset_date`.");
                }
            },
            final_reset_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `final_reset_date`.");
                }
            },
            npa_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `npa_status`."),
            },
            npa_final_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `npa_final_status`."),
            },
            add_string_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `add_string_1`."),
            },
            add_string_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `add_string_2`."),
            },
            add_float_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `add_float_1`."),
            },
            add_float_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => return Err("Could not parse property `add_float_2`."),
            },
            add_int_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => return Err("Could not parse property `add_int_1`."),
            },
            add_int_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => return Err("Could not parse property `add_int_2`."),
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
