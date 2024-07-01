use rbdate::DateParser;
use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub product_id: String,
    pub account_number: String,
    pub bill_due_day: NaiveDate,
    pub outstanding_balance_inr: f64,
    pub currency_loan: String,
    pub rate_of_int: f64,
    pub total_tenure: i64,
    pub completed_tenure: i64,
    pub customer_id: String,
    pub emi_amount: f64,
    pub int_day_count: i64,
    pub data_process_date: NaiveDate,
    pub report_date: NaiveDate,
    pub gl_code: String,
    pub installment_frequency: String,
    pub npa_classification: String,
    pub npa_amount: f64,
    pub cust_hlth_code: String,
    pub cust_npa_class: String,
    pub final_npa_class: String,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            product_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            account_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `account_number`.");
                }
            },
            bill_due_day: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch bill_due_day from input."),
                },
                None => {
                    return Err("Could not parse property `bill_due_day`.");
                }
            },
            outstanding_balance_inr: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `outstanding_balance_inr`.");
                }
            },
            currency_loan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency_loan`.");
                }
            },
            rate_of_int: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `rate_of_int`.");
                }
            },
            total_tenure: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `total_tenure`.");
                }
            },
            completed_tenure: match value_iterator.next() {
                Some(val) => val.parse::<i64>().unwrap_or(0),
                None => {
                    return Err("Could not parse property `completed_tenure`.");
                }
            },
            customer_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `customer_id`.");
                }
            },
            emi_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `emi_amount`.");
                }
            },
            int_day_count: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0),
                None => {
                    return Err("Could not parse property `int_day_count`.");
                }
            },
            data_process_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch data_process_date from input."),
                },
                None => {
                    return Err("Could not parse property `data_process_date`.");
                }
            },
            report_date: match value_iterator.next() {
                Some(val) => match dmy_date_parser.parse_opt(val) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd_opt(1970, 1, 1)
                        .expect("Could not fetch report_date from input."),
                },
                None => {
                    return Err("Could not parse property `report_date`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            installment_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `installment_frequency`.");
                }
            },
            npa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa_classification`.");
                }
            },
            npa_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(0.0),
                None => {
                    return Err("Could not parse property `npa_amount`.");
                }
            },
            cust_hlth_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_hlth_code`.");
                }
            },
            cust_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cust_npa_class`.");
                }
            },
            final_npa_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `final_npa_class`.");
                }
            },
        };
        Ok(input_account)
    }
}
