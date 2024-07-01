use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug)]
pub struct Input {
    pub key_1: String,
    pub br_no: String,
    pub act_type: String,
    pub purpose_code_a: String,
    pub applic_amount: f64,
    pub repay_count: i64,
    pub repay_day: i64,
    pub repay_freq: String,
    pub app_amt: f64,
    pub loan_bal: f64,
    pub adv_bal: f64,
    pub theo_loan_bal: f64,
    pub loan_repay: f64,
    pub pend_dues: f64,
    pub apprv_date: Option<NaiveDate>,
    pub lst_fin_date: Option<NaiveDate>,
    pub lst_arr_date: Option<NaiveDate>,
    pub pend_dues_date: Option<NaiveDate>,
    pub int_rate: f64,
    pub cat: String,
    pub loan_trm: i64,
    pub bad_debt_ind: String,
    pub arr_int_accr: f64,
    pub arr_int_incr: f64,
    pub rt_incr: i64,
    pub customer_no: String,
    pub currency_ind: String,
    pub store_rate: f64,
    pub cr_rating: String,
    pub gl_class_code: String,
    pub theo_unpd_arrs_int: i64,
    pub security_amount: i64,
    pub last_credit_dt: String,
    pub old_bad_debt_ind: String,
    pub npa_date: Option<NaiveDate>,
    pub collection_amt: f64,
    pub provision_amount: f64,
    pub last_repriced_date: Option<NaiveDate>,
    pub next_repriced_date: Option<NaiveDate>,
    pub repricing_frequency: String,
    pub inca: String,
    pub rating_source: String,
    pub rating_code: String,
    pub benchmark: String,
    pub name: String,
    pub cust_acct_no: String,
    pub prim_acct: String,
    pub segment_code: String,
    pub industry_code: String,
    pub grup_code: String,
    pub bus_sector_code: String,
    pub tier_cust_type: String,
    pub a1: i64,
    pub a2: i64,
    pub a3: i64,
    pub a4: Option<NaiveDate>,
    pub a5: Option<NaiveDate>,
    pub a6: Option<NaiveDate>,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub asondate: Option<NaiveDate>,
    pub mat_dt: Option<NaiveDate>,
}

impl Input {
    pub fn identifier(&self) -> String {
        format!("{}", self.key_1)
    }
}

impl Input {
    pub fn new_from_line(line: &str, dmy: &DateParser) -> InputParseResult {
        let mut value_iterator = line.split('|');

        let input = Input {
            key_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `key_1`."));
                }
            },
            br_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `br_no`."));
                }
            },
            act_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `act_type`."
                    ));
                }
            },
            purpose_code_a: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `purpose_code_a`."
                    ));
                }
            },
            applic_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `applic_amount`."
                    ));
                }
            },
            repay_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `repay_count`."
                    ));
                }
            },
            repay_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `repay_day`."
                    ));
                }
            },
            repay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `repay_freq`."
                    ));
                }
            },
            app_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `app_amt`."));
                }
            },
            loan_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `loan_type`."
                    ));
                }
            },
            adv_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `adv_bal`."));
                }
            },
            theo_loan_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `theo_loan_bal`."
                    ));
                }
            },
            loan_repay: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `loan_repay`."
                    ));
                }
            },
            pend_dues: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pend_dues`."
                    ));
                }
            },
            apprv_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `apprv_date`."
                    ));
                }
            },
            lst_fin_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `lst_fin_date`."
                    ));
                }
            },
            lst_arr_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `lst_arr_date`."
                    ));
                }
            },
            pend_dues_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `pend_dues_date`."
                    ));
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_rate`."
                    ));
                }
            },
            loan_trm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `loan_trm`."
                    ));
                }
            },
            bad_debt_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `bad_debt_ind`."
                    ));
                }
            },
            arr_int_accr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `arr_int_accr`."
                    ));
                }
            },
            arr_int_incr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `arr_int_incr`."
                    ));
                }
            },
            rt_incr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `rt_incr`."));
                }
            },
            customer_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `customer_no`."
                    ));
                }
            },
            currency_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `currency_ind`."
                    ));
                }
            },
            store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `store_rate`."
                    ));
                }
            },
            cr_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `cr_rating`."
                    ));
                }
            },
            gl_class_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `gl_class_code`."
                    ));
                }
            },
            theo_unpd_arrs_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `theo_unpd_arrs_int`."
                    ));
                }
            },
            security_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `security_amount`."
                    ));
                }
            },
            last_credit_dt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_credit_dt`."
                    ));
                }
            },
            old_bad_debt_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `old_bad_debt_ind`."
                    ));
                }
            },
            npa_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `npa_date`."
                    ));
                }
            },
            collection_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `collection_amt`."
                    ));
                }
            },
            provision_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `provision_amount`."
                    ));
                }
            },
            last_repriced_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_repriced_date`."
                    ));
                }
            },
            next_repriced_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `next_repriced_date`."
                    ));
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `repricing_frequency`."
                    ));
                }
            },
            inca: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `inca`."));
                }
            },
            rating_source: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `rating_source`."
                    ));
                }
            },
            rating_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `rating_code`."
                    ));
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `benchmark`."
                    ));
                }
            },
            name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `name`."));
                }
            },
            cust_acct_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `cust_acct_no`."
                    ));
                }
            },
            prim_acct: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `prim_acct`."
                    ));
                }
            },
            segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `segment_code`."
                    ));
                }
            },
            industry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `industry_code`."
                    ));
                }
            },
            grup_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `grup_code`."
                    ));
                }
            },
            bus_sector_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_rate`."
                    ));
                }
            },
            tier_cust_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `cost_center2`."
                    ));
                }
            },
            a1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a1`."));
                }
            },
            a2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a2`."));
                }
            },
            a3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a3`."));
                }
            },
            a4: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a4`."));
                }
            },
            a5: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a5`."));
                }
            },
            a6: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a6`."));
                }
            },
            a7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a7`."));
                }
            },
            a8: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a8`."));
                }
            },
            a9: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a9`."));
                }
            },
            a10: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `a10`."));
                }
            },
            asondate: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `asondate`."
                    ));
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `mat_dt`."));
                }
            },
            cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `cat`."));
                }
            },
        };
        return InputParseResult::Some(input);
    }
}

pub enum InputParseResult {
    Error(String),
    Some(Input),
}
