use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub flow_id: String,
    pub grp_id: String,
    pub llg_id: String,
    pub amount: f64,
    pub ccy_id: String,
    pub intr_rate: f64,
    pub reprice_freq: String,
    pub reprice_dt: Option<NaiveDate>,
    pub mat_dt: Option<NaiveDate>,
    pub acc_num: String,
    pub strt_dt: Option<NaiveDate>,
    pub intr_cal_freq: String,
    pub is_float_rate: String,
    pub float_rate_bm: String,
    pub bu_id: String,
    pub cust_id: String,
    pub cust_name: String,
    pub sprd: f64,
    pub schm_code: String,
    pub min_ir: f64,
    pub max_ir: f64,
    pub dep_amount: f64,
    pub mat_amt: f64,
    pub exch_rate: f64,
    pub cust_ctry_code: String,
    pub cust_crdt_rtng: String,
    pub cust_sect_code: String,
    pub cust_indt_code: String,
    pub custom1: String,
    pub custom2: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            flow_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            grp_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            llg_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            ccy_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            intr_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            reprice_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            reprice_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            acc_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            strt_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            intr_cal_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            is_float_rate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            float_rate_bm: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            bu_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            sprd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            min_ir: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            max_ir: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            dep_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            mat_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            exch_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            cust_ctry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            cust_crdt_rtng: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            cust_sect_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            cust_indt_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            custom1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
            custom2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property ``.");
                }
            },
        };
        Ok(input_account)
    }
}
