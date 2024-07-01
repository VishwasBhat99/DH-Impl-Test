use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub accno: String,
    pub branchcode: String,
    pub custno: String,
    pub uccid: String,
    pub ccy: String,
    pub product: String,
    pub acc_date: Option<NaiveDate>,
    pub gl_code: i64,
    pub glcode_compounded_portion: i64,
    pub glcode_int_accrued: i64,
    pub deposit_date: Option<NaiveDate>,
    pub initial_deposit_amount: i64,
    pub initial_dep_amtlcy: i64,
    pub current_outstanding_bal: Option<f64>,
    pub current_outstandingbal_lcy: i64,
    pub cum_interest: f64,
    pub cum_interest_amt_lcy: f64,
    pub maturity_date: Option<NaiveDate>,
    pub interest_type: String,
    pub interst_acrrual_basis: String,
    pub interest_accured_amount: f64,
    pub interest_compution_type: String,
    pub interest_rate: Option<f64>,
    pub interest_payment_freq: f64,
    pub next_int_payment_dt: f64,
    pub compounding_freq: i64,
    pub next_compounding_dt: Option<NaiveDate>,
    pub floating_rate_benchmark: String,
    pub spread: i64,
    pub next_repricing_dt: Option<NaiveDate>,
    pub repricing_frequency: i64,
    pub non_withdrawable_flag: String,
    pub noticedays: String,
    pub lockin_till_dt: Option<NaiveDate>,
    pub dep_pledged_against_loan_yn: String,
    pub customerconstitutioncode_1: String,
    pub customerconstitutioncode_2: String,
    pub period_months: i64,
    pub period_days: i64,
    pub intrest_craeted_upto: Option<NaiveDate>,
    pub interest_accrued_upto: Option<NaiveDate>,
    pub f_15hyear: String,
    pub customer_name: String,
    pub total_principal_amount: f64,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            accno: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accno`.");
                }
            },
            branchcode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branchcode`.");
                }
            },
            custno: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custno`.");
                }
            },
            uccid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `uccid`.");
                }
            },
            ccy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy`.");
                }
            },
            product: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product`.");
                }
            },
            acc_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_date`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            glcode_compounded_portion: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `glcode_compounded_portion`.");
                }
            },
            glcode_int_accrued: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `glcode_int_accrued`.");
                }
            },
            deposit_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `deposit_date`.");
                }
            },
            initial_deposit_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `initial_deposit_amount`.");
                }
            },
            initial_dep_amtlcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `initial_dep_amtlcy`.");
                }
            },
            current_outstanding_bal: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `current_outstanding_bal`.");
                }
            },
            current_outstandingbal_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `current_outstandingbal_lcy`.");
                }
            },
            cum_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cum_interest`.");
                }
            },
            cum_interest_amt_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cum_interest_amt_lcy`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            interest_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_type`.");
                }
            },
            interst_acrrual_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interst_acrrual_basis`.");
                }
            },
            interest_accured_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_accured_amount`.");
                }
            },
            interest_compution_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_compution_type`.");
                }
            },
            interest_rate: match value_iterator.next() {
                Some(val) => val.parse().ok(),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            interest_payment_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_payment_freq`.");
                }
            },
            next_int_payment_dt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `next_int_payment_dt`.");
                }
            },
            compounding_freq: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `compounding_freq`.");
                }
            },
            next_compounding_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_compounding_dt`.");
                }
            },
            floating_rate_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `floating_rate_benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            next_repricing_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_dt`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            non_withdrawable_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `non_withdrawable_flag`.");
                }
            },
            noticedays: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `noticedays`.");
                }
            },
            lockin_till_dt: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lockin_till_dt`.");
                }
            },
            dep_pledged_against_loan_yn: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `dep_pledged_against_loan_yn`.");
                }
            },
            customerconstitutioncode_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customerconstitutioncode_1`.");
                }
            },
            customerconstitutioncode_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customerconstitutioncode_2`.");
                }
            },
            period_months: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `period_months`.");
                }
            },
            period_days: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `period_days`.");
                }
            },
            intrest_craeted_upto: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `intrest_craeted_upto`.");
                }
            },
            interest_accrued_upto: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `interest_accrued_upto`.");
                }
            },
            f_15hyear: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `f_15hyear`.");
                }
            },
            customer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_name`.");
                }
            },
            total_principal_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `total_principal_amount`.");
                }
            },
        };
        Ok(input_account)
    }
}
