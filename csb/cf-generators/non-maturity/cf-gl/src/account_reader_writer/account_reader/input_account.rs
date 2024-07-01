use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub gl_cd: String,
    pub os_bal: f64,
    pub ccy: String,
    pub os_bal_lcy: f64,
    pub br_cd: String,
    pub dr: f64,
    pub cr: f64,
    pub as_on: Option<NaiveDate>,
    pub gl_desc: String,
    pub w4b_cd: String,
    pub w4b_desc: String,
    pub balm_llg: String,
    pub care_llg: String,
    pub ba_llg: String,
    pub cf_type: String,
    pub is_acc_gl: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            os_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_bal`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            os_bal_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_bal_lcy`.");
                }
            },
            br_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_cd`.");
                }
            },
            dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dr`.");
                }
            },
            cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cr`.");
                }
            },
            as_on: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            gl_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_desc`.");
                }
            },
            w4b_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `w4b_cd`.");
                }
            },
            w4b_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `w4b_desc`.");
                }
            },
            balm_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_llg`.");
                }
            },
            care_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `care_llg`.");
                }
            },
            ba_llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ba_llg`.");
                }
            },
            cf_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_type`.");
                }
            },
            is_acc_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_acc_gl`.");
                }
            },
        };
        Ok(input_account)
    }
}
