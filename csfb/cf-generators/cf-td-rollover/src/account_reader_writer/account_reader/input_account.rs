use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub account_number: String,
    pub branch_code: String,
    pub cust_id: String,
    pub group_id: String,
    pub cust_name: String,
    pub currency: String,
    pub product_code: String,
    pub gl_code: String,
    pub acc_open_date: Option<NaiveDate>,
    pub curr_outstanding_bal: f64,
    pub curr_outstanding_bal_lcy: f64,
    pub interest_rate: f64,
    pub accr_int_amt: f64,
    pub accr_int_gl_code: String,
    pub init_dep_amount: f64,
    pub init_dep_amount_lcy: f64,
    pub mat_date: Option<NaiveDate>,
    pub int_accrual_basis: String,
    pub int_comp_type: String,
    pub int_pay_freq: String,
    pub next_int_pay_date: Option<NaiveDate>,
    pub comp_freq: String,
    pub next_comp_date: Option<NaiveDate>,
    pub pledge_against_loan: String,
    pub loan_acc_no: String,
    pub loan_acc_mat_date: Option<NaiveDate>,
    pub constitution: String,
    pub roi_category: String,
    pub contract_no: String,
    pub stable_deposit: f64,
    pub effective_mat_date: Option<NaiveDate>,
    pub days_till_report: i64,
    pub volatility: String,
    pub period_of_deposits: i64,
    pub premature_ratio: f64,
    pub overall_rollover_ratio: f64,
    pub rollover_ratio_non_volatile: f64,
    pub non_rollover_ratio_non_volatile: f64,
    pub non_rollover_ratio_volatile: f64,
    pub financial_client: String,
    pub lcr_category: String,
    pub td_overdue_flag: String,
    pub line_of_credit: String,
    pub turnover: String,
    pub add_field1: i64,
    pub add_field2: f64,
    pub add_field3: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            branch_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `branch_code`.");
                }
            },
            cust_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_id`.");
                }
            },
            group_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `group_id`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            product_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_code`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            acc_open_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `acc_open_date`.");
                }
            },
            curr_outstanding_bal: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `curr_outstanding_bal`.");
                }
            },
            curr_outstanding_bal_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `curr_outstanding_bal_lcy`.");
                }
            },
            interest_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `interest_rate`.");
                }
            },
            accr_int_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accr_int_amt`.");
                }
            },
            accr_int_gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `accr_int_gl_code`.");
                }
            },
            init_dep_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `init_dep_amount`.");
                }
            },
            init_dep_amount_lcy: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `init_dep_amount_lcy`.");
                }
            },
            mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_date`.");
                }
            },
            int_accrual_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_accrual_basis`.");
                }
            },
            int_comp_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_comp_type`.");
                }
            },
            int_pay_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_pay_freq`.");
                }
            },
            next_int_pay_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_int_pay_date`.");
                }
            },
            comp_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `comp_freq`.");
                }
            },
            next_comp_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_comp_date`.");
                }
            },
            pledge_against_loan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pledge_against_loan`.");
                }
            },
            loan_acc_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_acc_no`.");
                }
            },
            loan_acc_mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `loan_acc_mat_date`.");
                }
            },
            constitution: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `constitution`.");
                }
            },
            roi_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `roi_category`.");
                }
            },
            contract_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contract_no`.");
                }
            },
            stable_deposit: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `stable_deposit`.");
                }
            },
            effective_mat_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `effective_mat_date`.");
                }
            },
            days_till_report: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `days_till_report`.");
                }
            },
            volatility: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `volatility`.");
                }
            },
            period_of_deposits: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `period_of_deposits`.");
                }
            },
            premature_ratio: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `premature_ratio`.");
                }
            },
            overall_rollover_ratio: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `overall_rollover_ratio`.");
                }
            },
            rollover_ratio_non_volatile: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `rollover_ratio_non_volatile`.");
                }
            },
            non_rollover_ratio_non_volatile: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `non_rollover_ratio_non_volatile`.");
                }
            },
            non_rollover_ratio_volatile: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `non_rollover_ratio_volatile`.");
                }
            },
            financial_client: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `financial_client`.");
                }
            },
            lcr_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lcr_category`.");
                }
            },
            td_overdue_flag: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `td_overdue_flag`.");
                }
            },
            line_of_credit: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'line_of_credit'");
                }
            },
            turnover: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'turnover'");
                }
            },
            add_field1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property 'add_field1'");
                }
            },
            add_field2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property 'add_field2'");
                }
            },
            add_field3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property 'add_field3'");
                }
            },
        };
        Ok(input_account)
    }
}
