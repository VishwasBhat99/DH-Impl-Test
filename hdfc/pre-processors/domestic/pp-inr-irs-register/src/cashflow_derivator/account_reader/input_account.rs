use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: String,
    pub contract_id: f64,
    pub folder: String,
    pub trading_banking: String,
    pub intrnl_extrnl: String,
    pub cntr_prty_name: String,
    pub trade_dt: Option<NaiveDate>,
    pub st_dt: Option<NaiveDate>,
    pub deal_ccy: String,
    pub org_notional: f64,
    pub pay_int_rt: f64,
    pub rec_int_rt: f64,
    pub exchng_rt: f64,
    pub pay_reset_dt: Option<NaiveDate>,
    pub rec_reset_dt: Option<NaiveDate>,
    pub pay_payment_dt: Option<NaiveDate>,
    pub rec_payment_dt: Option<NaiveDate>,
    pub pay_payment_freq: String,
    pub rec_payment_freq: String,
    pub deal_stats: String,
    pub inp_id: String,
    pub trade_bank: String,
    pub m_bank: String,
    pub flow_typ: String,
    pub flow_typ1: String,
    pub flow_typ2: String,
    pub flow_typ3: String,
    pub flow_typ4: String,
    pub flow_amt: f64,
    pub cf_dt: Option<NaiveDate>,
    pub flow_ccy: String,
    pub hkd_rt: f64,
    pub hkd_amt: f64,
    pub m_h_rep_dt2: Option<NaiveDate>,
    pub inr_amt: f64,
    pub inr_rt: f64,
    pub contract_typology: String
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            entity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `entity`.");
                }
            },
            trade_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trade_id`.");
                }
            },
            contract_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `contract_id`.");
                }
            },
            folder: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `folder`.");
                }
            },
            trading_banking: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trading_banking`.");
                }
            },
            intrnl_extrnl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intrnl_extrnl`.");
                }
            },
            cntr_prty_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cntr_prty_name`.");
                }
            },
            trade_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `trade_dt`.");
                }
            },
            st_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `st_dt`.");
                }
            },
            deal_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_ccy`.");
                }
            },
            org_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `org_notional`.");
                }
            },
            pay_int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pay_int_rt`.");
                }
            },
            rec_int_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rec_int_rt`.");
                }
            },
            exchng_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exchng_rt`.");
                }
            },
            pay_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pay_reset_dt`.");
                }
            },
            rec_reset_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rec_reset_dt`.");
                }
            },
            pay_payment_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pay_payment_dt`.");
                }
            },
            rec_payment_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `rec_payment_dt`.");
                }
            },
            pay_payment_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pay_payment_freq`.");
                }
            },
            rec_payment_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rec_payment_freq`.");
                }
            },
            deal_stats: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_stats`.");
                }
            },
            inp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inp_id`.");
                }
            },
            trade_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trade_bank`.");
                }
            },
            m_bank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `m_bank`.");
                }
            },
            flow_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_typ`.");
                }
            },
            flow_typ1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_typ1`.");
                }
            },
            flow_typ2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_typ2`.");
                }
            },
            flow_typ3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_typ3`.");
                }
            },
            flow_typ4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_typ4`.");
                }
            },
            flow_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `flow_amt`.");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_dt`.");
                }
            },
            flow_ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `flow_ccy`.");
                }
            },
            hkd_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `hkd_rt`.");
                }
            },
            hkd_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `hkd_amt`.");
                }
            },
            m_h_rep_dt2: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `m_h_rep_dt2`.");
                }
            },
            inr_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inr_amt`.");
                }
            },
            inr_rt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inr_rt`.");
                }
            },
            contract_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contract_typology`.");
                }
            },
        };
        Ok(input_account)
    }
}
