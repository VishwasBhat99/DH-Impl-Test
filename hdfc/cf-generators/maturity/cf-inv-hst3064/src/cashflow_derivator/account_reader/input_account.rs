use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub security_desc: String,
    pub maturity: Option<NaiveDate>,
    pub face_value: Option<i64>,
    pub book_value: Option<i64>,
    pub int_rate: f64,
    pub int_freq: String,
    pub last_cpn_dt: Option<NaiveDate>,
    pub nxt_cpn_dt: Option<NaiveDate>,
    pub no_of_days: Option<i64>,
    pub amt: f64,
    pub as_on_dt: NaiveDate,
    pub ccy: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_dt_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            security_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `security_desc`.");
                }
            },
            maturity: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity`.");
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            int_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_freq`.");
                }
            },
            last_cpn_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_cpn_dt`.");
                }
            },
            nxt_cpn_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `nxt_cpn_dt`.");
                }
            },
            no_of_days: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `no_of_days`.");
                }
            },
            amt: match value_iterator.next() {
                Some(val) => {
                    let bal = val.parse().unwrap_or(DEFAULT_FLOAT);
                    if bal < 0.0 {
                        DEFAULT_FLOAT
                    } else {
                        bal
                    }
                }
                None => {
                    return Err("Could not parse property `amt`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse(val),
                None => {
                    return Err("Could not parse property `as_on_dt`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
        };
        Ok(input_account)
    }
}
