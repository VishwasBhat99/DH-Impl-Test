use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub key_1: String,
    pub br_no: String,
    pub act_type: String,
    pub purpose_code_a: String,
    pub applic_amount: f64,
    pub repay_count: i32,
    pub repay_day: i32,
    pub repay_freq: i32,
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
    pub loan_trm: f64,
    pub bad_debt_ind: String,
    pub arr_int_accr: f64,
    pub arr_int_incr: f64,
    pub rt_incr: f64,
    pub customer_no: String,
    pub currency_ind: String,
    pub store_rate: i32,
    pub cr_rating: String,
    pub gl_class_code: String,
    pub theo_unpd_arrs_int: f64,
    pub security_amount: f64,
    pub last_credit_dt: Option<NaiveDate>,
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
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub a4: String,
    pub a5: String,
    pub a6: Option<NaiveDate>,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub asondate: Option<NaiveDate>,
    pub mat_dt: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            key_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `key_1`.");
                }
            },
            br_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `br_no`.");
                }
            },
            act_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `act_type`.");
                }
            },
            purpose_code_a: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `purpose_code_a`.");
                }
            },
            applic_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `applic_amount`.");
                }
            },
            repay_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `repay_count`.");
                }
            },
            repay_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `repay_day`.");
                }
            },
            repay_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `repay_freq`.");
                }
            },
            app_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `app_amt`.");
                }
            },
            loan_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `loan_bal`.");
                }
            },
            adv_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `adv_bal`.");
                }
            },
            theo_loan_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `theo_loan_bal`.");
                }
            },
            loan_repay: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `loan_repay`.");
                }
            },
            pend_dues: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `pend_dues`.");
                }
            },
            apprv_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `apprv_date`.");
                }
            },
            lst_fin_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_fin_date`.");
                }
            },
            lst_arr_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lst_arr_date`.");
                }
            },
            pend_dues_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `pend_dues_date`.");
                }
            },
            int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            cat: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cat`.");
                }
            },
            loan_trm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `loan_trm`.");
                }
            },
            bad_debt_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bad_debt_ind`.");
                }
            },
            arr_int_accr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `arr_int_accr`.");
                }
            },
            arr_int_incr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `arr_int_incr`.");
                }
            },
            rt_incr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `rt_incr`.");
                }
            },
            customer_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_no`.");
                }
            },
            currency_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency_ind`.");
                }
            },
            store_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `store_rate`.");
                }
            },
            cr_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cr_rating`.");
                }
            },
            gl_class_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_class_code`.");
                }
            },
            theo_unpd_arrs_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `theo_unpd_arrs_int`.");
                }
            },
            security_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `security_amount`.");
                }
            },
            last_credit_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_credit_dt`.");
                }
            },
            old_bad_debt_ind: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `old_bad_debt_ind`.");
                }
            },
            npa_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `npa_date`.");
                }
            },
            collection_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `collection_amt`.");
                }
            },
            provision_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `provision_amount`.");
                }
            },
            last_repriced_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repriced_date`.");
                }
            },
            next_repriced_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repriced_date`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            inca: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inca`.");
                }
            },
            rating_source: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating_source`.");
                }
            },
            rating_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rating_code`.");
                }
            },
            benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `benchmark`.");
                }
            },
            name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `name`.");
                }
            },
            cust_acct_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_acct_no`.");
                }
            },
            prim_acct: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prim_acct`.");
                }
            },
            segment_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `segment_code`.");
                }
            },
            industry_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry_code`.");
                }
            },
            grup_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `grup_code`.");
                }
            },
            bus_sector_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bus_sector_code`.");
                }
            },
            tier_cust_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `tier_cust_type`.");
                }
            },
            a1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `a1`.");
                }
            },
            a2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `a2`.");
                }
            },
            a3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `a3`.");
                }
            },
            a4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a4`.");
                }
            },
            a5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a5`.");
                }
            },
            a6: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `a6`.");
                }
            },
            a7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a7`.");
                }
            },
            a8: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a8`.");
                }
            },
            a9: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a9`.");
                }
            },
            a10: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `a10`.");
                }
            },
            asondate: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `asondate`.");
                }
            },
            mat_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_dt`.");
                }
            },
        };
        Ok(input_account)
    }
}

#[derive(Debug, Clone)]
pub struct AddBORMData {
    pub key_1: String,
    pub unpd_prin_bal: f64,
    pub theo_unpd_prin_bal: f64,
    pub comp_amt: f64,
    pub comp_freq: String,
    pub last_rep_date: String,
    pub inca: String,
    pub moi: String,
    pub npa_date: String,
}

impl AddBORMData {
    pub fn new(borm_data: Vec<&str>, row: usize) -> AddBORMData {
        AddBORMData {
            key_1: get_data(&borm_data, 0, row),
            unpd_prin_bal: get_data(&borm_data, 1, row).parse::<f64>().unwrap_or(0.0),
            theo_unpd_prin_bal: get_data(&borm_data, 2, row).parse::<f64>().unwrap_or(0.0),
            comp_amt: get_data(&borm_data, 3, row).parse::<f64>().unwrap_or(0.0),
            comp_freq: get_data(&borm_data, 4, row),
            last_rep_date: get_data(&borm_data, 5, row),
            inca: get_data(&borm_data, 6, row),
            moi: get_data(&borm_data, 7, row),
            npa_date: get_data(&borm_data, 8, row),
        }
    }
}

pub fn get_data(data: &Vec<&str>, index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not read {}th column from {}th row: {}\nExpected 9 fields from each row in BORM Data File",
                index + 1,
                row + 1,
                data.join("|")
            )
        })
        .to_string()
}
