use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
#[derive(Debug, Clone)]
pub struct InputAccount {
    pub loan_account_no: String,
    pub loan_id: i64,
    pub customer_id: String,
    pub product_id: i64,
    pub product_desc: String,
    pub product_type: String,
    pub recovery_type: String,
    pub cust_name: String,
    pub disbursal_date: Option<NaiveDate>,
    pub due_day: i64,
    pub maturity_date: Option<NaiveDate>,
    pub original_tenure: i64,
    pub current_tenure: i64,
    pub balance_installments: f64,
    pub installment_type: String,
    pub int_recry_freq: String,
    pub principal_recry_freq: String,
    pub days_past_due: i64,
    pub asset_clsfn: String,
    pub int_type: String,
    pub cust_int_rate: f64,
    pub rate_type: String,
    pub overdue_prin_amount: f64,
    pub overdue_interest_amount: f64,
    pub os_prin: f64,
    pub emi_amount: f64,
    pub accrued_not_recieved_int: f64,
    pub last_payment_date: Option<NaiveDate>,
    pub next_instmt_due_date: Option<NaiveDate>,
    pub branch_id: i64,
    pub currency_code: String,
    pub fraud: String,
    pub restructure: String,
    pub A1: f64,
    pub A2: f64,
    pub A3: f64,
    pub A4: f64,
    pub A5: f64,
    pub A6: f64,
    pub A7: f64,
    pub A8: Option<NaiveDate>,
    pub A9: i64,
    pub A10: i64,
    pub A11: i64,
    pub A12: i64,
    pub A13: i64,
    pub A14: i64,
    pub A15: String,
    pub A16: String,
    pub A17: String,
    pub A18: String,
    pub A19: String,
    pub A20: String,
    pub A21: String,
    pub A22: String,
    pub A23: String,
    pub A24: String,
    pub A25: String,
    pub A26: String,
    pub A27: String,
    pub A28: String,
    pub A29: String,
    pub A30: String,
    pub A31: String,
    pub A32: String,
    pub A33: String,
    pub A34: String,
    pub A35: String,
    pub A36: Option<NaiveDate>,
    pub A37: Option<NaiveDate>,
    pub A38: Option<NaiveDate>,
    pub A39: Option<NaiveDate>,
    pub A40: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: String, dmy: &DateParser) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            loan_account_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_account_no`.");
                }
            },
            loan_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `loan_id`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            product_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `product_id`.");
                }
            },
            product_desc: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_desc`.");
                }
            },
            product_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `product_type`.");
                }
            },
            recovery_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `recovery_type`.");
                }
            },
            cust_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_name`.");
                }
            },
            disbursal_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `disbursal_date`.");
                }
            },
            due_day: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `due_day`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            original_tenure: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `original_tenure`.");
                }
            },
            current_tenure: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `current_tenure`.");
                }
            },
            balance_installments: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `balance_installments`.");
                }
            },
            installment_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `installment_type`.");
                }
            },
            int_recry_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_recry_freq`.");
                }
            },
            principal_recry_freq: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `principal_recry_freq`.");
                }
            },
            days_past_due: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `days_past_due`.");
                }
            },
            asset_clsfn: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_clsfn`.");
                }
            },
            int_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            cust_int_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cust_int_rate`.");
                }
            },
            rate_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rate_type`.");
                }
            },
            overdue_prin_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_type`.");
                }
            },
            overdue_interest_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `overdue_interest_amount`.");
                }
            },
            os_prin: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `os_prin`.");
                }
            },
            emi_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amount`.");
                }
            },
            accrued_not_recieved_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accrued_not_recieved_int`.");
                }
            },
            last_payment_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_payment_date`.");
                }
            },
            next_instmt_due_date: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_instmt_due_date`.");
                }
            },
            branch_id: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `branch_id`.");
                }
            },
            currency_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency_code`.");
                }
            },
            fraud: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fraud`.");
                }
            },
            restructure: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `restructure`.");
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
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A6`.");
                }
            },
            A7: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `A7`.");
                }
            },
            A8: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
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
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A11`.");
                }
            },
            A12: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A12`.");
                }
            },
            A13: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `A13`.");
                }
            },
            A14: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
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
            },
            A21: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A21`.");
                }
            },
            A22: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A22`.");
                }
            },
            A23: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A23`.");
                }
            },
            A24: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A24`.");
                }
            },
            A25: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A25`.");
                }
            },
            A26: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A26`.");
                }
            },
            A27: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A27`.");
                }
            },
            A28: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A28`.");
                }
            },
            A29: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A29`.");
                }
            },
            A30: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A30`.");
                }
            },
            A31: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A31`.");
                }
            },
            A32: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A32`.");
                }
            },
            A33: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A33`.");
                }
            },
            A34: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A34`.");
                }
            },
            A35: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `A35`.");
                }
            },
            A36: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `A36`.");
                }
            },
            A37: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `A37`.");
                }
            },
            A38: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `A38`.");
                }
            },
            A39: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `A39`.");
                }
            },
            A40: match value_iterator.next() {
                Some(val) => dmy.parse_opt(val),
                None => {
                    return Err("Could not parse property `A40`.");
                }
            },
        };
        Ok(input_account)
    }
}
