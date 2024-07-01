use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub card_no: String,
    pub card_status: String,
    pub prd_code: String,
    pub acc_no: String,
    pub outstanding_bal: f64,
    pub emi_amt: f64,
    pub in_date: Option<NaiveDate>,
    pub tenurs: f64,
    pub del_cnt: String,
    pub cif_no: String,
    pub pan_no: String,
    pub intrate: String,
    pub maturity_date: Option<NaiveDate>,
    pub bgl: String,
    pub cgl: String,
    pub branchcode: String,
    pub intamount: String,
    pub duedate: String,
    pub group: String,
    pub llg: String,
    pub currency: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            card_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `card_no`.");
                }
            },
            card_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `card_status`.");
                }
            },
            prd_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prd_code`.");
                }
            },
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            outstanding_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `outstanding_bal`.");
                }
            },
            emi_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt`.");
                }
            },
            in_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `in_date`.");
                }
            },
            tenurs: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `tenurs`.");
                }
            },
            del_cnt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `del_cnt`.");
                }
            },
            cif_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cif_no`.");
                }
            },
            pan_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pan_no`.");
                }
            },
            intrate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intrate`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            bgl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bgl`.");
                }
            },
            cgl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cgl`.");
                }
            },
            branchcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branchcode`.");
                }
            },
            intamount: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intamount`.");
                }
            },
            duedate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_name`.");
                }
            },
            group: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group`.");
                }
            },
            llg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `llg`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
        };
        Ok(input_account)
    }
}
