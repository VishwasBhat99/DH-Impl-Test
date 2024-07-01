use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub serial_no: String,
    pub state: String,
    pub region: String,
    pub branch_name: String,
    pub village: String,
    pub group: String,
    pub group_id: String,
    pub branch_model: String,
    pub member_name: String,
    pub member_id: String,
    pub prod_name: String,
    pub purpose_of_loan: String,
    pub rate_of_int: f64,
    pub loan_id: i64,
    pub disb_date: Option<NaiveDate>,
    pub amt_disbursed: f64,
    pub first_od_date: Option<NaiveDate>,
    pub npa_date: Option<NaiveDate>,
    pub prin_due_npa: f64,
    pub int_due_npa: f64,
    pub prin_os_npa: f64,
    pub prin_collected: f64,
    pub int_collected: f64,
    pub dpd_day: i64,
    pub standard: String,
    pub prin_due_reporting: f64,
    pub int_due_reporting: f64,
    pub prin_os_reporting: f64,
    pub npa_int_accr: f64,
    pub npa_int_during_period: f64,
    pub funder_name: String,
    pub maturity_dt: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            serial_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `serial_no`.");
                }
            },
            state: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `state`.");
                }
            },
            region: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `region`.");
                }
            },
            branch_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_name`.");
                }
            },
            village: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `village`.");
                }
            },
            group: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group`.");
                }
            },
            group_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_id`.");
                }
            },
            branch_model: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_model`.");
                }
            },
            member_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `member_name`.");
                }
            },
            member_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `member_id`.");
                }
            },
            prod_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_name`.");
                }
            },
            purpose_of_loan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_name`.");
                }
            },
            rate_of_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rate_of_int`.");
                }
            },
            loan_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `purpose_of_loan`.");
                }
            },
            disb_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `disb_date`.");
                }
            },
            amt_disbursed: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amt_disbursed`.");
                }
            },
            first_od_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `first_od_date`.");
                }
            },
            npa_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `npa_date`.");
                }
            },
            prin_due_npa: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_due_npa`.");
                }
            },
            int_due_npa: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_due_npa`.");
                }
            },
            prin_os_npa: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_os_npa`.");
                }
            },
            prin_collected: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_collected`.");
                }
            },
            int_collected: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_collected`.");
                }
            },
            dpd_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `dpd_day`.");
                }
            },
            standard: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `standard`.");
                }
            },
            prin_due_reporting: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_due_reporting`.");
                }
            },
            int_due_reporting: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_due_reporting`.");
                }
            },
            prin_os_reporting: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_os_reporting`.");
                }
            },
            npa_int_accr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_int_accr`.");
                }
            },
            npa_int_during_period: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_int_during_period`.");
                }
            },
            funder_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `funder_name`.");
                }
            },
            maturity_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
