use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_ref: String,
    pub intt_name: String,
    pub cntpty_name: String,
    pub deal_date: Option<NaiveDate>,
    pub value_date: Option<NaiveDate>,
    pub mat_date: Option<NaiveDate>,
    pub deal_amt_act: f64,
    pub deal_amt_plc: f64,
    pub roi: f64,
    pub int_amt_fx_deal: f64,
    pub mat_amt_fx_deal: f64,
    pub practice: String,
    pub spread: String,
    pub benchmark: String,
    pub rate_sett_freq: String,
    pub sett_freq: String,
    pub sett_typ: String,
    pub dealer: String,
    pub cntpty_id: String,
    pub inv_curcy: String,
    pub gl_code: String,
    pub int_type: String,
    pub cgl: String,
    pub group: String,
    pub llg: String,
    pub cf_type: String,
    pub cf_currency: String,
    pub cf_amount: f64,
    pub cf_date: Option<NaiveDate>,
    pub cf_date_2: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String, dmy: &DateParser) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_ref: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_ref`.");
                }
            },
            intt_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intt_name`.");
                }
            },
            cntpty_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntpty_name`.");
                }
            },
            deal_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_date`.");
                }
            },
            value_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `value_date`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_date`.");
                }
            },
            deal_amt_act: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `deal_amt_act`.");
                }
            },
            deal_amt_plc: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `deal_amt_plc`.");
                }
            },
            roi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `roi`.");
                }
            },
            int_amt_fx_deal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `int_amt_fx_deal`.");
                }
            },
            mat_amt_fx_deal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `mat_amt_fx_deal`.");
                }
            },
            practice: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `practice`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            rate_sett_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate_sett_freq`.");
                }
            },
            sett_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sett_freq`.");
                }
            },
            sett_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sett_typ`.");
                }
            },
            dealer: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dealer`.");
                }
            },
            cntpty_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntpty_id`.");
                }
            },
            inv_curcy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inv_curcy`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            cgl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
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
            cf_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_type`.");
                }
            },
            cf_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cf_currency`.");
                }
            },
            cf_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `cf_amount`.");
                }
            },
            cf_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_date`.");
                }
            },
            cf_date_2: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_date_2`.");
                }
            },
        };
        Ok(input_account)
    }
}
