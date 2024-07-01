use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub date: Option<NaiveDate>,
    pub segment: String,
    pub sub_segment: String,
    pub member_id: String,
    pub member_name: String,
    pub isin: String,
    pub security_desc: String,
    pub mat_date: Option<NaiveDate>,
    pub face_value: i64,
    pub face_val_treps: String,
    pub balance: String,
    pub isin_cred_lend: String,
    pub security_des: String,
    pub face_val_rec: String,
    pub mark_val: f64,
    pub book_val: f64,
    pub os_amt: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `date`.");
                }
            },
            segment: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `segment`.");
                }
            },
            sub_segment: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub-segment`.");
                }
            },
            member_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `member_id`.");
                }
            },
            member_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `member_name`.");
                }
            },
            isin: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin`.");
                }
            },
            security_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_cd`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `security_desc`.");
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            face_val_treps: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            balance: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balance`.");
                }
            },
            isin_cred_lend: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_cd`.");
                }
            },
            security_des: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin_cred_lend`.");
                }
            },
            face_val_rec: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `face_val_rec`.");
                }
            },

            mark_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mark_val`.");
                }
            },
            book_val: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_val`.");
                }
            },
            os_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_amt`.");
                }
            },
        };
        Ok(input_account)
    }
}
