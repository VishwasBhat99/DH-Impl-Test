use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_no: String,
    pub bank_oper_typ: String,
    pub deal_dt: Option<NaiveDate>,
    pub value_dt: Option<NaiveDate>,
    pub deal_type: String,
    pub slr_type: String,
    pub category: String,
    pub portfolio: String,
    pub counter_party: String,
    pub settle_amt: f64,
    pub accr_int: f64,
    pub sec_setdate: Option<NaiveDate>,
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
            deal_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_no`.");
                }
            },
            bank_oper_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_oper_typ`.");
                }
            },
            deal_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_dt`.");
                }
            },
            value_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `value_dt`.");
                }
            },
            deal_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            slr_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_freq`.");
                }
            },
            category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `last_cpn_dt`.");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            counter_party: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party`.");
                }
            },
            settle_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `settle_amt`.");
                }
            },
            accr_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accr_int`.");
                }
            },
            sec_setdate: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `sec_setdate`.");
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
