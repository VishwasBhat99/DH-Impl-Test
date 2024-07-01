use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub branchcode: String,
    pub ccy: String,
    pub currencyconversionrt: f64,
    pub amount: f64,
    pub intrrt: f64,
    pub maturitydt: NaiveDate,
    pub foracid: String,
    pub startdt: NaiveDate,
    pub intrcompfreq: String,
    pub isfloatingrt: String,
    pub floatingrtbenchmark: String,
    pub custname: String,
    pub spread: f64,
    pub schmcode: String,
    pub maturityamt: f64,
    pub custid: String,
    pub maxintrt: f64,
    pub minintrt: f64,
    pub custctrycode: String,
    pub custcrdtrtng: String,
    pub custsectcode: String,
    pub custindtcode: String,
    pub custom1: String,
    pub custom2: String,
    pub gl_sub_head_code:String,
    pub cust_hlth_code:String,
    pub schm_type:String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            branchcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branchcode`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            currencyconversionrt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `currencyconversionrt`.");
                }
            },
            amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `amount`.");
                }
            },
            intrrt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `intrrt`.");
                }
            },
            maturitydt: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch maturity date from input."),
                },
                None => {
                    return Err("Could not parse property `maturitydt`.");
                }
            },
            foracid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `foracid`.");
                }
            },
            startdt: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch start date from input."),
                },
                None => {
                    return Err("Could not parse property `startdt`.");
                }
            },
            intrcompfreq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intrcompfreq`.");
                }
            },
            isfloatingrt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isfloatingrt`.");
                }
            },
            floatingrtbenchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `floatingrtbenchmark`.");
                }
            },
            custname: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custname`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            schmcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schmcode`.");
                }
            },
            maturityamt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `maturityamt`.");
                }
            },
            custid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custid`.");
                }
            },
            maxintrt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `maxintrt`.");
                }
            },
            minintrt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `minintrt`.");
                }
            },
            custctrycode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custctrycode`.");
                }
            },
            custcrdtrtng: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not read property `account_start_date`.");
                }
            },
            custsectcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custsectcode`.");
                }
            },
            custindtcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custindtcode`.");
                }
            },
            custom1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom1`.");
                }
            },
            custom2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custom2`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
                }
            },
            cust_hlth_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_hlth_code`.");
                }
            },
            schm_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_type`.");
                }
            },
        };
        Ok(input_account)
    }
    pub fn get(&self, field_string: &str) -> Result<String, String> {
        match field_string {
          "branchcode" => Ok(self.branchcode.to_owned()),
            "ccy" => Ok(self.ccy.to_owned()),
            "foracid" => Ok(self.foracid.to_owned()),
            "intrcompfreq" => Ok(self.intrcompfreq.to_owned()),
            "isfloatingrt" => Ok(self.isfloatingrt.to_owned()),
            "floatingrtbenchmark" => Ok(self.floatingrtbenchmark.to_owned()),
            "custname" => Ok(self.custname.to_owned()),
            "schmcode" => Ok(self.schmcode.to_owned()),
            "custid" => Ok(self.custid.to_owned()),
            "custctrycode" => Ok(self.custctrycode.to_owned()),
            "custcrdtrtng" => Ok(self.custcrdtrtng.to_owned()),
            "custsectcode" => Ok(self.custsectcode.to_owned()),
            "custindtcode" => Ok(self.custindtcode.to_owned()),
            "custom1" => Ok(self.custom1.to_owned()),
            "custom2" => Ok(self.custom2.to_owned()),
            "gl_sub_head_code" => Ok(self.gl_sub_head_code.to_owned()),
            "cust_hlth_code" => Ok(self.cust_hlth_code.to_owned()),
            "schm_type" => Ok(self.schm_type.to_owned()),
            _ => Err(format!("invalid field name to get '{}'", field_string)),
        }
    }
}
