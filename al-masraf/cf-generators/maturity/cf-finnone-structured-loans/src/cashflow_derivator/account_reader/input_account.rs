use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub accrual_basis: String,
    pub accrued_interest: f64,
    pub branch: String,
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
    pub payment_type: String,
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
    pub int_amt: f64,
    pub prin_amt: f64,
    pub cf_dt: Option<NaiveDate>,
    pub as_on_date: Option<NaiveDate>,
    pub final_int_rate: f64,
    pub cost_centre: i64,
    pub alm_line: String,
    pub coa: String,
    pub division: String,
    pub rep_freq: String,
    pub next_repricing_date: Option<NaiveDate>,
    pub last_repricing_date: Option<NaiveDate>,
    pub asset_class: String,
    pub al_line: String,
    pub balm_l2: String,
    pub bmid: String,
    pub ia_line: String,
    pub weaker_code: String,
    pub der_int_rate: f64,
    pub bnchmrk_rate: f64,
    pub spread: f64,
    pub fully_floating_flg: String,
    pub A1: f64,
    pub A2: f64,
    pub A3: f64,
    pub A4: f64,
    pub A5: f64,
    pub A6: i64,
    pub A7: i64,
    pub A8: i64,
    pub A9: i64,
    pub A10: i64,
    pub A11: String,
    pub A12: String,
    pub A13: String,
    pub A14: String,
    pub A15: String,
    pub A16: String,
    pub A17: String,
    pub A18: String,
    pub A19: String,
    pub A20: String
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String, dmy: &DateParser) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');

        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            accrual_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accrual_basis`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
                }
            },
            branch: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch`.");
                }
            },
            curr_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `curr_code`.");
                }
            },
            current_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `current_bal`.");
                }
            },
            due_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `due_date`.");
                }
            },
            interest_pay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_pay_freq`.");
                }
            },
            intt_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `intt_rate`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_type`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_date`.");
                }
            },
            original_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `original_balance`.");
                }
            },
            orig_term: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `orig_term`.");
                }
            },
            org_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `org_date`.");
                }
            },
            emi: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi`.");
                }
            },
            payment_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `payment_freq`.");
                }
            },
            payment_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `payment_type`.");
                }
            },
            rate_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate_flag`.");
                }
            },
            repricing_index: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_index`.");
                }
            },
            dpd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dpd`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            scheme_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `scheme_id`.");
                }
            },
            psl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `psl`.");
                }
            },
            npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa`.");
                }
            },
            inst_st_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            weaker: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `weaker`.");
                }
            },
            current_book_balance: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `current_book_balance`.");
                }
            },
            first_inst_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `first_inst_date`.");
                }
            },
            inst_num: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `inst_num`.");
                }
            },
            num_inst_paid: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `num_inst_paid`.");
                }
            },
            last_inst_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_inst_date`.");
                }
            },
            indv_corp_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `indv_corp_flag`.");
                }
            },
            customer_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_type`.");
                }
            },
            gr_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gr_dr`.");
                }
            },
            gr_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `gr_cr`.");
                }
            },
            re_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `re_dr`.");
                }
            },
            re_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `re_cr`.");
                }
            },
            is_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `is_dr`.");
                }
            },
            is_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `is_cr`.");
                }
            },
            ui_dr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ui_dr`.");
                }
            },
            ui_cr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ui_cr`.");
                }
            },
            asset_class_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_class_id`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            prod_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `prod_type`.");
                }
            },
            is_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `is_ofs_gl`.");
                }
            },
            gr_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gr_ofs_gl`.");
                }
            },
            re_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `re_ofs_gl`.");
                }
            },
            ui_ofs_gl: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ui_ofs_gl`.");
                }
            },
            int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt`.");
                }
            },
            prin_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt`.");
                }
            },
            cf_dt: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `asset_class`.");
                }
            },
            as_on_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `as_on_date`.");
                }
            },
            final_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_rate`.");
                }
            },
            cost_centre: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `cost_center2`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            coa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `coa`.");
                }
            },
            division: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `division`.");
                }
            },
            rep_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing frequency`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            asset_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_class`.");
                }
            },
            al_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `al_line`.");
                }
            },
            balm_l2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `balm_l2`.");
                }
            },
            bmid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bmid`.");
                }
            },
            ia_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ia_line`.");
                }
            },
            weaker_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `weaker_code`.");
                }
            },
            der_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `der_int_rate`.");
                }
            },
            bnchmrk_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bnchmrk_rate`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            fully_floating_flg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fully_floating_flg`.");
                }
            },
            A1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A1`.");
                }
            },
            A2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A2`.");
                }
            },
            A3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A3`.");
                }
            },
            A4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A4`.");
                }
            },
            A5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A5`.");
                }
            },
            A6: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A6`.");
                }
            },
            A7: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A7`.");
                }
            },
            A8: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A8`.");
                }
            },
            A9: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A9`.");
                }
            },
            A10: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A10`.");
                }
            },
            A11: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A11`.");
                }
            },
            A12: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A12`.");
                }
            },
            A13: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A13`.");
                }
            },
            A14: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A14`.");
                }
            },
            A15: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A15`.");
                }
            },
            A16: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A16`.");
                }
            },
            A17: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A17`.");
                }
            },
            A18: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A18`.");
                }
            },
            A19: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A19`.");
                }
            },
            A20: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A20`.");
                }
            }
        };
        Ok(input_account)
    }
}
