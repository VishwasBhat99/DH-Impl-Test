use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_id: String,
    pub inst_name: String,
    pub oper_type: String,
    pub counter_party: String,
    pub deal_dt: Option<NaiveDate>,
    pub val_dt: Option<NaiveDate>,
    pub ccy: String,
    pub lcy_amt: f64,
    pub roi: f64,
    pub tenor_days: i64,
    pub mat_dt: Option<NaiveDate>,
    pub int_amt: f64,
    pub mat_amt: Option<f64>,
    pub dealer_name: String,
    pub ndsref: String,
    pub deal_status: String,
    pub nds_time: String,
    pub aip: f64,
    pub air: f64,
    pub code: String,
    pub as_on_dt: NaiveDate,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_dt_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            deal_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_id`.");
                }
            },
            inst_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inst_name`.");
                }
            },
            oper_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `oper_type`.");
                }
            },
            counter_party: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counter_party`.");
                }
            },
            deal_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deal_dt`.");
                }
            },
            val_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `val_dt`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            lcy_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcy_amt`.");
                }
            },
            roi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `roi`.");
                }
            },
            tenor_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `tenor_days`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_dt`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt`.");
                }
            },
            mat_amt: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `mat_amt`.");
                }
            },
            dealer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dealer_name`.");
                }
            },
            ndsref: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ndsref`.");
                }
            },
            deal_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_status`.");
                }
            },
            nds_time: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `nds_time`.");
                }
            },
            aip: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `aip`.");
                }
            },
            air: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `air`.");
                }
            },
            code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `code`.");
                }
            },
            as_on_dt: match value_iterator.next() {
                Some(val) => dmy_dt_parser.parse(val),
                None => {
                    return Err("Could not parse property `as_on_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}
