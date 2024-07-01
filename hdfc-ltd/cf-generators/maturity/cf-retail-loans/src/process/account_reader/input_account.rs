use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub acc_no: String,
    pub disbursed_amt: f64,
    pub os_loan_bal_local_currency: f64,
    pub curr_applicable_interest_rate: f64,
    pub ei_amount_current: f64,
    pub interest_type: String,
    pub os_p_bal_due_local_currency: f64,
    pub os_i_bal_due_local_currency: f64,
    pub ei_amt_paid_advance_local_curr: f64,
    pub pre_ei_bal_local_curr: f64,
    pub account_open_value_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub ei_start_date_current: Option<NaiveDate>,
    pub ei_end_date_current: Option<NaiveDate>,
    pub ei_payment_frequency_current: String,
    pub emi_last_paid_date_current: Option<NaiveDate>,
    pub ei_payment_day: String,
    pub ei_orginal_term: String,
    pub ei_balance_term: String,
    pub repricing_benchmark: String,
    pub spread: String,
    pub last_repricing_date: Option<NaiveDate>,
    pub next_repricing_date: Option<NaiveDate>,
    pub repricing_frequency: String,
    pub number_ei_structures: i64,
    pub npa_classification: String,
    pub remark: String,
    pub months_os_comb: String,
    pub moratorium_type: String,
    pub from_moratorium_date: Option<NaiveDate>,
    pub to_moratorium_date: Option<NaiveDate>,
    pub recalculate_ei_amount_flag: String,
    pub moratorium_interest_calculation: String,
    pub bullet_payment_flag: String,
    pub restructured_flag: String,
    pub residential_mortgage: String,
    pub risk_weight: String,
    pub internal_rating: String,
    pub external_rating: String,
    pub contractual_tenor: String,
    pub residual_tenor: String,
    pub customer_constitution_code: String,
    pub product_code: String,
    pub p_gl_code: String,
    pub m_npa_classification: String,
    pub accrued_interest: String,
    pub customer_id: String,
    pub customer_name: String,
    pub group_id: String,
    pub group_name: String,
    pub branch_code: String,
    pub sector: String,
    pub industry: String,
    pub ltv: String,
    pub overdue_account: String,
    pub excess_account: String,
    pub loan_type: String,
    pub residual_interest: String,
    pub currency: String,
    pub hdfc_ltd_percentage: f64,
    pub securitization_percentage: f64,
    pub overdue_type: String,
    pub alm_line: String,
    pub emi_overdue_gl_cd: i64,
    pub pre_emi_overdue_gl_cd: i64,
    pub excess_emi_gl_cd: i64,
    pub excess_pre_emi_gl_cd: i64,
    pub sma_flag: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `acc_no`.");
                }
            },
            disbursed_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `disbursed_amt`.");
                }
            },
            os_loan_bal_local_currency: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `os_loan_bal_local_currency`.");
                }
            },
            curr_applicable_interest_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `curr_applicable_interest_rate`.");
                }
            },
            ei_amount_current: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `ei_amount_current`.");
                }
            },
            interest_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_type`.");
                }
            },
            os_p_bal_due_local_currency: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `os_p_bal_due_local_currency`.");
                }
            },
            os_i_bal_due_local_currency: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `os_i_bal_due_local_currency`.");
                }
            },
            ei_amt_paid_advance_local_curr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `ei_amt_paid_advance_local_curr`.");
                }
            },
            pre_ei_bal_local_curr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `pre_ei_bal_local_curr`.");
                }
            },
            account_open_value_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `account_open_value_date`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            ei_start_date_current: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ei_start_date_current`.");
                }
            },
            ei_end_date_current: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ei_end_date_current`.");
                }
            },
            ei_payment_frequency_current: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_payment_frequency_current`.");
                }
            },
            emi_last_paid_date_current: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_last_paid_date_current`.");
                }
            },
            ei_payment_day: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_payment_day`.");
                }
            },
            ei_orginal_term: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_orginal_term`.");
                }
            },
            ei_balance_term: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ei_balance_term`.");
                }
            },
            repricing_benchmark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_benchmark`.");
                }
            },
            spread: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `spread`.");
                }
            },
            last_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_repricing_date`.");
                }
            },
            next_repricing_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_repricing_date`.");
                }
            },
            repricing_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `repricing_frequency`.");
                }
            },
            number_ei_structures: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `number_ei_structures`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            remark: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `remark`.");
                }
            },
            months_os_comb: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `months_os_comb`.");
                }
            },
            moratorium_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `moratorium_type`.");
                }
            },
            from_moratorium_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `from_moratorium_date`.");
                }
            },
            to_moratorium_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `to_moratorium_date`.");
                }
            },
            recalculate_ei_amount_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `recalculate_ei_amount_flag`.");
                }
            },
            moratorium_interest_calculation: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `moratorium_interest_calculation`.");
                }
            },
            bullet_payment_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bullet_payment_flag`.");
                }
            },
            restructured_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `restructured_flag`.");
                }
            },
            residential_mortgage: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `residential_mortgage`.");
                }
            },
            risk_weight: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `risk_weight`.");
                }
            },
            internal_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_rating`.");
                }
            },
            external_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `external_rating`.");
                }
            },
            contractual_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contractual_tenor`.");
                }
            },
            residual_tenor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `residual_tenor`.");
                }
            },
            customer_constitution_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_constitution_code`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            p_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `p_gl_code`.");
                }
            },
            m_npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `m_npa_classification`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
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
            group_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_id`.");
                }
            },
            group_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_name`.");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_code`.");
                }
            },
            sector: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sector`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry`.");
                }
            },
            ltv: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ltv`.");
                }
            },
            overdue_account: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_account`.");
                }
            },
            excess_account: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `excess_account`.");
                }
            },
            loan_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_type`.");
                }
            },
            residual_interest: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `residual_interest`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            hdfc_ltd_percentage: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `hdfc_ltd_percentage`.");
                }
            },
            securitization_percentage: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `securitization_percentage`.");
                }
            },
            overdue_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `overdue_type`.");
                }
            },
            alm_line: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `alm_line`.");
                }
            },
            emi_overdue_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `emi_overdue_gl_cd`.");
                }
            },
            pre_emi_overdue_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `pre_emi_overdue_gl_cd`.");
                }
            },
            excess_emi_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `excess_emi_gl_cd`.");
                }
            },
            excess_pre_emi_gl_cd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `excess_pre_emi_gl_cd`.");
                }
            },
            sma_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sma_flag`.");
                }
            },
        };
        Ok(input_account)
    }
}
