use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub transche_desc: String,
    pub date_of_mobilization: Option<NaiveDate>,
    pub deposit: f64,
    pub rate_of_interest: f64,
    pub due_date: Option<NaiveDate>,
    pub repaid_on: String,
    pub repay_amt: f64,
    pub os_bal: f64,
    pub tr_dt: Option<NaiveDate>,
    pub tr_type: String,
    pub cmuser: String,
    pub cmdate: Option<NaiveDate>,
    pub vuser: String,
    pub vdate: Option<NaiveDate>,
    pub curr: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            transche_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `transche_desc`.");
                }
            },
            date_of_mobilization: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` mat_date`.");
                }
            },
            deposit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `deposit`.");
                }
            },
            rate_of_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rate_of_interest`.");
                }
            },
            due_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` due_date`.");
                }
            },
            repaid_on: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repaid_on`.");
                }
            },
            repay_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `repay_amt`.");
                }
            },
            os_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_bal`.");
                }
            },
            tr_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` tr_dt`.");
                }
            },
            tr_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tr_type`.");
                }
            },
            cmuser: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cmuser`.");
                }
            },
            cmdate: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` cmdate`.");
                }
            },
            vuser: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `vuser`.");
                }
            },
            vdate: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `vdate`.");
                }
            },
            curr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `curr`.");
                }
            },
        };
        Ok(input_account)
    }
}
