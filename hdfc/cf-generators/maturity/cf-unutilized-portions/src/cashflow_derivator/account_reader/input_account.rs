use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub cust_ref_code: Option<i64>,
    pub pp_table: String,
    pub ccf_percent: Option<f64>,
    pub exp_start_date: Option<NaiveDate>,
    pub exp_end_date: Option<NaiveDate>,
    pub undrawn_amount: Option<f64>,
    pub prod_type_desc: String,
    pub party_type_desc: String,
    pub undrn_cov_amount: Option<f64>,
    pub pre_mitigation_rw_ul: Option<f64>,
    pub undrn_rwa: Option<f64>,
    pub gl_code: Option<i64>,
    pub ccy_code: String,
    pub ret_corporate_ind: String,
    pub as_on_dt: NaiveDate,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_dt_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            cust_ref_code: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `cust_ref_code`.");
                }
            },
            pp_table: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pp_table`.");
                }
            },
            ccf_percent: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `ccf_percent`.");
                }
            },
            exp_start_date: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `exp_start_date`.");
                }
            },
            exp_end_date: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `exp_end_date`.");
                }
            },
            undrawn_amount: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `undrawn_amount`.");
                }
            },
            prod_type_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_type_desc`.");
                }
            },
            party_type_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `party_type_desc`.");
                }
            },
            undrn_cov_amount: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `undrn_cov_amount`.");
                }
            },
            pre_mitigation_rw_ul: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `pre_mitigation_rw_ul`.");
                }
            },
            undrn_rwa: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `undrn_rwa`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            ccy_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy_code`.");
                }
            },
            ret_corporate_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ret_corporate_ind`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse(val),
                None => {
                    return Err("Could not parse property `as_on_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
