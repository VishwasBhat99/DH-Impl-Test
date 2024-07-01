use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub fc_ubs_acc: String,
    pub cust_name: String,
    pub pout_bal: Option<f64>,
    pub acc_int: f64,
    pub st_dt: Option<NaiveDate>,
    pub c_dt: Option<NaiveDate>,
    pub gl_cd: String,
    pub int_rt: Option<f64>,
    pub int_typ: String,
    pub int_bmark: String,
    pub spread: f64,
    pub rt_flag: String,
    pub prod_cd: String,
    pub br_id: String,
    pub nxt_pay_dt: Option<NaiveDate>,
    pub comp_freq: String,
    pub comp_freq_incr: i64,
    pub mis1: i64,
    pub mis2: i64,
    pub mis3: i64,
    pub ccy: String,
    pub dt: Option<NaiveDate>,
    pub int_portion: Option<f64>,
    pub prin_pay: Option<f64>,
    pub ratings: String,
    pub rating_agency: String,
    pub asset_class: String,
    pub div: String,
    pub typ: String,
    pub originator: String,
    pub rep_freq: String,
    pub nxt_rep_dt: Option<NaiveDate>,
    pub portfolio: String,
    pub alm_line: String,
    pub txn_mis2: i64,
    pub old_fc_ubs_acc: String,
    pub deal_name: String,
    pub cf_start_date: Option<NaiveDate>,
    pub ubs_acct_number: String,
    pub sma_flag: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            fc_ubs_acc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Accid`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `CustName`.");
                }
            },
            pout_bal: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `PoutBal`.");
                }
            },
            acc_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `AccInt`.");
                }
            },
            st_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `Sdate`.");
                }
            },
            c_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `Cdate`.");
                }
            },
            gl_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `GLCode`.");
                }
            },
            int_rt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `IntRate`.");
                }
            },
            int_typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `IntType`.");
                }
            },
            int_bmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `Spread`.");
                }
            },
            rt_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Rate Flag`.");
                }
            },
            prod_cd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Prod Code`.");
                }
            },
            br_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Branch ID`.");
                }
            },
            nxt_pay_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `NEXT_PAY_DATE`.");
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Compound Frequency`.");
                }
            },
            comp_freq_incr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `Compound Frequency Incr`.");
                }
            },
            mis1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `MIS1`.");
                }
            },
            mis2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `MIS2`.");
                }
            },
            mis3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `MIS3`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),

                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `CF Date`.");
                }
            },
            int_portion: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `Interest Amount`.");
                }
            },
            prin_pay: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `Principal Amount`.");
                }
            },
            ratings: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Ratings`.");
                }
            },
            rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Rating Agency`.");
                }
            },
            asset_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Asset Class`.");
                }
            },
            div: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Division`.");
                }
            },
            typ: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Type`.");
                }
            },
            originator: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Originator`.");
                }
            },
            rep_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ReprFreq`.");
                }
            },
            nxt_rep_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `NextReprDate`.");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `Portfolio Type`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ALM_LINE`.");
                }
            },
            txn_mis2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `TXN_MIS2`.");
                }
            },
            old_fc_ubs_acc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `old_fc_ubs_acc`.");
                }
            },
            deal_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_name`.");
                }
            },
            cf_start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `cf_start_date`.");
                }
            },
            ubs_acct_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ubs_acct_number`.");
                }
            },
            sma_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sma_flag`.");
                }
            },
        };
        Ok(input_account)
    }
}
