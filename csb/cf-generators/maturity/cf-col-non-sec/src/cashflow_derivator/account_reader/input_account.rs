use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub col_id: i64,
    pub acc_id: String,
    pub acc_type: String,
    pub cust_id: i64,
    pub col_type_cd: String,
    pub col_type_desc: String,
    pub tot_val_of_col: f64,
    pub ccy: String,
    pub tot_mk_val_of_col: f64,
    pub mat_dt: NaiveDate,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            col_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `col_id`.");
                }
            },
            acc_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_id`.");
                }
            },
            acc_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_type`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            col_type_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `col_type_cd`.");
                }
            },
            col_type_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `col_type_desc`.");
                }
            },
            tot_val_of_col: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tot_val_of_col`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            tot_mk_val_of_col: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tot_mk_val_of_col`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not parse property `mat_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
