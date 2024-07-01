use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub branchcode: String,
    pub currency: String,
    pub currencyconvertionrate: f64,
    pub acct_num: String,
    pub product_id: String,
    pub customer_id: String,
    pub customer_name: String,
    pub start_date: String,
    pub sanctioned_amt: f64,
    pub distributed_amt: f64,
    pub out_amt: f64,
    pub int_rate: f64,
    pub mat_date: Option<NaiveDate>,
    pub inst_prin_amt: f64,
    pub inst_int_amt: f64,
    pub acc_end_date: Option<NaiveDate>,
    pub int_cal_freq: String,
    pub is_floating_rate: String,
    pub benchmark_ass: String,
    pub spread: String,
    pub min_int_rate: f64,
    pub max_int_rate: f64,
    pub early_date: Option<NaiveDate>,
    pub rep_freq: String,
    pub cust_ctry_code: String,
    pub cust_crtd_rt: String,
    pub cust_sect_code: String,
    pub cust_indt_code: String,
    pub custom1: String,
    pub custom2: String,
    pub npa_classification: String,
    pub overdue_days: String,
    pub wcdl_bucket_days: String,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub seg_code: String,
    pub final_seg_code: String,
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
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            currencyconvertionrate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `currencyconvertionrate`.");
                }
            },
            acct_num: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acct_num`.");
                }
            },
            product_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_id`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            start_date: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `start_date`.");
                }
            },
            sanctioned_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sanctioned_amt`.");
                }
            },
            distributed_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `distributed_amt`.");
                }
            },
            out_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `out_amt`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_date`.");
                }
            },
            inst_prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inst_prin_amt`.");
                }
            },
            inst_int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `inst_int_amt`.");
                }
            },
            acc_end_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_end_date`.");
                }
            },
            int_cal_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_cal_freq`.");
                }
            },
            is_floating_rate: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_floating_rate`.");
                }
            },
            benchmark_ass: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `benchmark_ass`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            min_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `min_int_rate`.");
                }
            },
            max_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `max_int_rate`.");
                }
            },
            early_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `early_date`.");
                }
            },
            rep_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rep_freq`.");
                }
            },
            cust_ctry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_ctry_code`.");
                }
            },
            cust_crtd_rt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_crtd_rt`.");
                }
            },
            cust_sect_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_sect_code`.");
                }
            },
            cust_indt_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_indt_code`.");
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
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            overdue_days: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_days`.");
                }
            },
            wcdl_bucket_days: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `wcdl_bucket_days`.");
                }
            },
            gl_sub_head_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_sub_head_code`.");
                }
            },
            schm_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `schm_code`.");
                }
            },
            seg_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `seg_code`.");
                }
            },
            final_seg_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_seg_code`.");
                }
            },
        };
        Ok(input_account)
    }
}
