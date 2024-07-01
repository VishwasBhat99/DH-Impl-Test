use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_no: String,
    pub int_basis: String,
    pub portfolio: String,
    pub gl: i64,
    pub couprt: Option<f64>,
    pub deal_dt: Option<NaiveDate>,
    pub val_dt: Option<NaiveDate>,
    pub mat_dt: Option<NaiveDate>,
    pub currcd: String,
    pub orgballcy: Option<f64>,
    pub orgbaltcy: f64,
    pub avgbalvdlcy: f64,
    pub avgbalvdtcy: f64,
    pub oscostlcy: f64,
    pub oscosttcy: f64,
    pub finallcy: f64,
    pub finaltcy: f64,
    pub int_amt: Option<f64>,
    pub lcyinterestamount: i64,
    pub tcyprodcd: String,
    pub proddesc: String,
    pub counterpartyname: String,
    pub as_of_date: NaiveDate,
    pub paymenttype: String,
    pub rt_flag: String,
    pub reprice_index: String,
    pub reprice_spread: i64,
    pub alm_line: String,
    pub div: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_id`.");
                }
            },
            int_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accural_basis`.");
                }
            },
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            gl: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl`.");
                }
            },
            couprt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `couprt`.");
                }
            },
            deal_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `dealdt`.");
                }
            },
            val_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `valudt`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `matudt`.");
                }
            },
            currcd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currcd`.");
                }
            },
            orgballcy: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `orgballcy`.");
                }
            },
            orgbaltcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `orgbaltcy`.");
                }
            },
            avgbalvdlcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `avgbaldlcy`.");
                }
            },
            avgbalvdtcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `avgbalvdtcy`.");
                }
            },
            oscostlcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `oscostlcy`.");
                }
            },
            oscosttcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `oscosttcy`.");
                }
            },
            finallcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `finallcy`.");
                }
            },
            finaltcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `finaltcy`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `int_amt`.");
                }
            },
            lcyinterestamount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `lcyinterestamount`.");
                }
            },
            tcyprodcd: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tcyprodcd`.");
                }
            },
            proddesc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `proddesc`.");
                }
            },
            counterpartyname: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterpartyname`.");
                }
            },
            as_of_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse(val),
                None => {
                    return Err("Could not parse property `asofdate`.");
                }
            },
            paymenttype: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `paymenttype`.");
                }
            },
            rt_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rateflag`.");
                }
            },
            reprice_index: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repriceindex`.");
                }
            },
            reprice_spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `repricespread`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            div: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
        };
        Ok(input_account)
    }
}
