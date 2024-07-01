use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub date_of_availment: Option<NaiveDate>,
    pub source: String,
    pub amount: f64,
    pub roi: f64,
    pub mat_date: Option<NaiveDate>,
    pub repayment_sch: String,
    pub res_mat: i64,
    pub frequency: String,
    pub pmt_st_dt: Option<NaiveDate>,
    pub remaining_prin: f64,
    pub p_installment: f64,
    pub no_of_installment: i64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            date_of_availment: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` date_of_availment`.");
                }
            },
            source: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `source`.");
                }
            },
            amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amount`.");
                }
            },
            roi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `roi`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` mat_date`.");
                }
            },
            repayment_sch: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repayment_sch`.");
                }
            },
            res_mat: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `res_mat`.");
                }
            },
            frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `frequency`.");
                }
            },
            pmt_st_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property ` pmt_st_dt`.");
                }
            },
            remaining_prin: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `remaining_prin`.");
                }
            },
            p_installment: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `p_installment`.");
                }
            },
            no_of_installment: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `no_of_installment`.");
                }
            },
        };
        Ok(input_account)
    }
}
