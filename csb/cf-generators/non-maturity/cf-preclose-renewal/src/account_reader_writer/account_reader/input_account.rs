use rbdate::DateParser;
use rbdate::NaiveDate;

use crate::statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub as_on_date: Option<NaiveDate>,
    pub acc_id: ::std::string::String,
    pub gl_code: ::std::string::String,
    pub acc_open_dt: Option<NaiveDate>,
    pub curr_out_bl_ccy: f64,
    pub curr_out_bl_lcy: f64,
    pub premat_renew_value_ccy: f64,
    pub premat_renew_value_lcy: f64,
    pub ccy: ::std::string::String,
    pub preclose_renew_dt: Option<NaiveDate>,
    pub int_rate: f64,
    pub actual_mat_dt: Option<NaiveDate>,
    pub prod_cd: ::std::string::String,
    pub add_dim_1: ::std::string::String,
    pub add_dim_2: ::std::string::String,
    pub add_dim_3: ::std::string::String,
    pub add_dim_4: ::std::string::String,
    pub add_dim_5: ::std::string::String,
    pub add_dim_6: ::std::string::String,
    pub add_dim_7: ::std::string::String,
    pub add_dim_8: ::std::string::String,
    pub add_dim_9: ::std::string::String,
    pub add_dim_10: ::std::string::String,
    pub event_type: ::std::string::String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            as_on_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            acc_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_id`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            acc_open_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_dt`.");
                }
            },
            curr_out_bl_ccy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `curr_out_bl_ccy`.");
                }
            },
            curr_out_bl_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `curr_out_bl_lcy`.");
                }
            },
            premat_renew_value_ccy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `premat_renew_value_ccy`.");
                }
            },
            premat_renew_value_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `premat_renew_value_lcy`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            preclose_renew_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `preclose_renew_dt`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            actual_mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `actual_mat_dt`.");
                }
            },
            prod_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_cd`.");
                }
            },
            add_dim_5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_6: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_8: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_9: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            add_dim_10: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `additional dim`.");
                }
            },
            event_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `event_type`.");
                }
            },
        };
        Ok(input_account)
    }
}
