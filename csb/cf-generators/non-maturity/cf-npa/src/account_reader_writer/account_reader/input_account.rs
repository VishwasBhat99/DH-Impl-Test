use rbdate::DateParser;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_no: String,
    pub asset_cd: String,
    pub acc_bal: f64,
    pub ho_bal: f64,
    pub ho_prov: f64,
    pub npa_amt: f64,
    pub ccy: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        _: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            asset_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_cd`.");
                }
            },
            acc_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `acc_bal`.");
                }
            },
            ho_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ho_bal`.");
                }
            },
            ho_prov: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ho_prov`.");
                }
            },
            npa_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npa_amt`.");
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
