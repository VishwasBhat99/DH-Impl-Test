use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_id: String,
    pub deal_type: String,
    pub deal_ref: String,
    pub leg_type: String,
    pub leg_number: String,
    pub trading_banking: String,
    pub counter_party_id: String,
    pub counter_party_name: String,
    pub deal_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub currency: String,
    pub exchange_rate: f64,
    pub currency_amt_inr: Option<f64>,
    pub currency_amt: f64,
    pub reval_rate: f64,
    pub netplamount: f64,
    pub treasury_gl_code: String,
    pub app1: String,
    pub app2: String,
    pub app3: String,
    pub app4: String,
    pub app5: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_id`.");
                }
            },
            deal_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_type`.");
                }
            },
            deal_ref: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_ref`.");
                }
            },
            leg_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `leg_type`.");
                }
            },
            leg_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `leg_number`.");
                }
            },
            trading_banking: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trading_banking`.");
                }
            },
            counter_party_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party_id`.");
                }
            },
            counter_party_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party_name`.");
                }
            },
            deal_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            exchange_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exchange_rate`.");
                }
            },
            currency_amt_inr: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `currency_amt_inr`.");
                }
            },
            currency_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `currency_amt`.");
                }
            },
            reval_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `reval_rate`.");
                }
            },
            netplamount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `netplamount`.");
                }
            },
            treasury_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `treasury_gl_code`.");
                }
            },
            app1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app1`.");
                }
            },
            app2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app2`.");
                }
            },
            app3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app3`.");
                }
            },
            app4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app4`.");
                }
            },
            app5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `app5`.");
                }
            },
        };
        Ok(input_account)
    }
}
