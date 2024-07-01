use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug)]
pub struct Input {
    pub account_number: String,
    pub accrual_basis: String,
    pub accrued_interest: f64,
    pub branch: i64,
    pub curr_code: String,
    pub current_bal: f64,
    pub due_date: Option<NaiveDate>,
    pub interest_pay_freq: String,
    pub intt_rate: f64,
    pub product_code: String,
    pub mat_date: Option<NaiveDate>,
    pub original_balance: f64,
    pub orig_term: i64,
    pub org_date: Option<NaiveDate>,
    pub emi: f64,
    pub payment_freq: String,
    pub payment_type: i64,
    pub rate_flag: String,
    pub repricing_index: String,
    pub dpd: f64,
    pub customer_name: String,
    pub scheme_id: String,
    pub psl: String,
    pub npa: String,
    pub inst_st_dt: Option<NaiveDate>,
    pub weaker: String,
    pub current_book_balance: f64,
    pub first_inst_date: Option<NaiveDate>,
    pub inst_num: i64,
    pub num_inst_paid: i64,
    pub last_inst_date: Option<NaiveDate>,
    pub indv_corp_flag: String,
    pub customer_type: String,
    pub gr_dr: f64,
    pub gr_cr: f64,
    pub re_dr: f64,
    pub re_cr: f64,
    pub is_dr: f64,
    pub is_cr: f64,
    pub ui_dr: f64,
    pub ui_cr: f64,
    pub asset_class_id: String,
    pub customer_id: String,
    pub prod_type: String,
    pub is_ofs_gl: String,
    pub gr_ofs_gl: String,
    pub re_ofs_gl: String,
    pub ui_ofs_gl: String,
    pub as_on_date: Option<NaiveDate>,
    pub final_int_rate: f64,
    pub cost_centre: i64,
    pub alm_line: String,
    pub coa: String,
    pub division: String,
    pub rep_freq: i64,
    pub next_repricing_date: Option<NaiveDate>,
    pub last_repricing_date: Option<NaiveDate>,
    pub asset_class: String,
    pub al_line: String,
    pub balm_l2: String,
    pub ia_line: String,
    pub orig_bm: String,
    pub der_int_rate: f64,
    pub bnchmrk_rate: f64,
    pub spread: f64,
    pub fully_floating_flg: String,
    pub convention: String,
}

impl Input {
    pub fn identifier(&self) -> String {
        format!("{}", self.account_number)
    }
}

impl Input {
    pub fn new_from_line(line: &str, dmy: &DateParser) -> InputParseResult {
        let mut value_iterator = line.split('|');

        let input = Input {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `account_number`."
                    ));
                }
            },
            accrual_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `accrual_basis`."
                    ));
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `accrued_interest`."
                    ));
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `branch`."));
                }
            },
            curr_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `curr_code`."
                    ));
                }
            },
            current_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `current_bal`."
                    ));
                }
            },
            due_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `due_date`."
                    ));
                }
            },
            interest_pay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `interest_pay_freq`."
                    ));
                }
            },
            intt_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `intt_rate`."
                    ));
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `loan_type`."
                    ));
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `mat_date`."
                    ));
                }
            },
            original_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `original_balance`."
                    ));
                }
            },
            orig_term: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `orig_term`."
                    ));
                }
            },
            org_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `org_date`."
                    ));
                }
            },
            emi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `emi`."));
                }
            },
            payment_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `payment_freq`."
                    ));
                }
            },
            payment_type: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `payment_type`."
                    ));
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `rate_flag`."
                    ));
                }
            },
            repricing_index: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `repricing_index`."
                    ));
                }
            },
            dpd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `dpd`."));
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `customer_name`."
                    ));
                }
            },
            scheme_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `scheme_id`."
                    ));
                }
            },
            psl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `psl`."));
                }
            },
            npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `npa`."));
                }
            },
            inst_st_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `as_on_date`."
                    ));
                }
            },
            weaker: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `weaker`."));
                }
            },
            current_book_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `current_book_balance`."
                    ));
                }
            },
            first_inst_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `first_inst_date`."
                    ));
                }
            },
            inst_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `inst_num`."
                    ));
                }
            },
            num_inst_paid: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `num_inst_paid`."
                    ));
                }
            },
            last_inst_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_inst_date`."
                    ));
                }
            },
            indv_corp_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `indv_corp_flag`."
                    ));
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `customer_type`."
                    ));
                }
            },
            gr_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `gr_dr`."));
                }
            },
            gr_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `gr_cr`."));
                }
            },
            re_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `re_dr`."));
                }
            },
            re_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `re_cr`."));
                }
            },
            is_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `is_dr`."));
                }
            },
            is_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `is_cr`."));
                }
            },
            ui_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ui_dr`."));
                }
            },
            ui_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ui_cr`."));
                }
            },
            asset_class_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `asset_class_id`."
                    ));
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `customer_id`."
                    ));
                }
            },
            prod_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `prod_type`."
                    ));
                }
            },
            is_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `is_ofs_gl`."
                    ));
                }
            },
            gr_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `gr_ofs_gl`."
                    ));
                }
            },
            re_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `re_ofs_gl`."
                    ));
                }
            },
            ui_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `ui_ofs_gl`."
                    ));
                }
            },
            as_on_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `as_on_date`."
                    ));
                }
            },
            final_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `int_rate`."
                    ));
                }
            },
            cost_centre: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `cost_center2`."
                    ));
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `alm_line`."
                    ));
                }
            },
            coa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `coa`."));
                }
            },
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `division`."
                    ));
                }
            },
            rep_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `rep_freq`."
                    ));
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `next_repricing_date`."
                    ));
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `last_repricing_date`."
                    ));
                }
            },
            asset_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `asset_class`."
                    ));
                }
            },
            al_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `al_line`."));
                }
            },
            balm_l2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `balm_l2`."));
                }
            },
            ia_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `ia_line`."));
                }
            },
            orig_bm: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `orig_bmid`."
                    ));
                }
            },
            der_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `der_int_rate`."
                    ));
                }
            },
            bnchmrk_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `bnchmrk_rate`."
                    ));
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return InputParseResult::Error(format!("Could not parse property `spread`."));
                }
            },
            fully_floating_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `fully_floating_flg`."
                    ));
                }
            },
            convention: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return InputParseResult::Error(format!(
                        "Could not parse property `convention`."
                    ));
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
